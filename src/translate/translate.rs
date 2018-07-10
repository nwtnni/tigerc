use itertools::Itertools;
use itertools::FoldWhile::{Continue, Done};
use sym::{store, Symbol};

use ast::*;
use ir;

use config::WORD_SIZE;
use operand::{Label, Temp, Reg};
use check::context::{Binding, VarContext};
use translate::Frame;
use ty::Ty;

pub struct Unit {
    label: Label,
    prologue: Vec<ir::Stm>,
    body: Vec<ir::Stm>,
    epilogue: Vec<ir::Stm>,
    size: usize,
}

pub fn translate_simple_var(frames: &[Frame], name: &Symbol) -> ir::Tree {

    // Start off at current frame's base pointer
    let rbp = ir::Exp::Temp(Temp::Reg(Reg::RBP));
    let link = store("STATIC_LINK");

    // Follow static links
    frames.iter()
        .fold_while(rbp, |acc, frame| {
            if frame.contains(*name) {
                Done(frame.get(*name, acc))
            } else {
                Continue(frame.get(link, acc))
            }
        })
        .into_inner()
        .into()
}

pub fn translate_field_var(rec_exp: ir::Tree, index: usize) -> ir::Tree {

    // Calculate memory address offset from record pointer
    ir::Exp::Mem(
        Box::new(
            ir::Exp::Binop(
                Box::new(rec_exp.into()),
                ir::Binop::Add,
                Box::new(ir::Exp::Const(index as i32 * WORD_SIZE)),
            )
        )
    ).into()

}

pub fn translate_index_var(array_exp: ir::Tree, index_exp: ir::Tree) -> ir::Tree {

    // Multiply offset by word size
    let offset_exp = ir::Exp::Binop(
        Box::new(index_exp.into()),
        ir::Binop::Mul,
        Box::new(ir::Exp::Const(WORD_SIZE)),
    );

    // Calculate memory address offset from array pointer
    let address_exp = ir::Exp::Mem(
        Box::new(
            ir::Exp::Binop(
                Box::new(array_exp.into()),
                ir::Binop::Add,
                Box::new(offset_exp),
            )
        )
    );

    address_exp.into()
}

pub fn translate_break(loops: &[Label]) -> ir::Tree {

    // Find latest loop exit label on stack
    let label = loops.last()
        .expect("Internal error: break without enclosing loop");

    // Jump to exit label
    ir::Stm::Jump(
        ir::Exp::Name(*label),
        vec![*label],
    ).into()

}

pub fn translate_nil() -> ir::Tree {
    ir::Exp::Const(0).into()
}

pub fn translate_var(frames: &[Frame], name: &Symbol) -> ir::Tree {
    translate_simple_var(frames, name)
}

pub fn translate_int(n: i32) -> ir::Tree {
    ir::Exp::Const(n).into()
}

pub fn translate_str(data: &mut Vec<ir::Static>, string: &str) -> ir::Tree {
    let string = ir::Static::new(string.to_string());
    let label = string.label();
    data.push(string);
    ir::Exp::Name(label).into()
}

pub fn translate_call(binding: &Binding, arg_exps: Vec<ir::Tree>) -> ir::Tree {

    let mut arg_exps = arg_exps.into_iter()
        .map(|arg_exp| arg_exp.into())
        .collect::<Vec<_>>();

    let label = match binding {
    | Binding::Ext(_, _, label) => label,
    | Binding::Fun(_, _, label) => {
        arg_exps.insert(0, ir::Exp::Temp(Temp::from_reg(Reg::RBP)).into());
        label
    },
    };

    // Call function
    ir::Exp::Call(
        Box::new(ir::Exp::Name(*label)),
        arg_exps,
    ).into()
}

pub fn translate_neg(neg: ir::Tree) -> ir::Tree {

        // Subtract sub-expression from 0
        ir::Exp::Binop(
            Box::new(ir::Exp::Const(0)),
            ir::Binop::Sub,
            Box::new(neg.into()),
        ).into()
}

pub fn translate_bin(lhs_exp: ir::Tree, op: Binop, rhs_exp: ir::Tree) -> ir::Tree {

    // Straightforward arithmetic operation
    if let Some(binop) = translate_binop(&op) {
        ir::Exp::Binop(
            Box::new(lhs_exp.into()),
            binop,
            Box::new(rhs_exp.into()),
        ).into()
    }

    // Conditional operation
    else if let Some(relop) = translate_relop(&op) {
        ir::Tree::Cx(
            Box::new(move |t, f| {
                ir::Stm::CJump(lhs_exp.into(), relop, rhs_exp.into(), t, f)
            })
        )
    }

    // All operations must be covered
    else {
        panic!("Internal error: non-exhaustive binop check");
    }
}

pub fn translate_rec(fields_exp: Vec<ir::Tree>) -> ir::Tree {

    // Calculate record size for malloc
    let size = ir::Exp::Const(WORD_SIZE * fields_exp.len() as i32);

    // Retrieve malloc label
    // TODO: is it okay to hard-code this?
    let malloc = Label::from_fixed("malloc");

    // Allocate temp for record pointer
    let pointer = Temp::from_str("MALLOC");

    // Call malloc and move resulting pointer into temp
    let mut seq = vec![
        ir::Stm::Move(
            ir::Exp::Call(
                Box::new(ir::Exp::Name(malloc)),
                vec![size],
            ),
            ir::Exp::Temp(pointer),
        ),
    ];

    // Move each field into memory offset from record pointer
    for (i, field_exp) in fields_exp.into_iter().enumerate() {
        seq.push(
            ir::Stm::Move(
                field_exp.into(),
                ir::Exp::Mem(
                    Box::new(
                        ir::Exp::Binop(
                            Box::new(ir::Exp::Temp(pointer)),
                            ir::Binop::Add,
                            Box::new(ir::Exp::Const(WORD_SIZE * i as i32)),
                        )
                    )
                ),
            )
        );
    }

    // Return record pointer after initialization
    ir::Exp::ESeq(
        Box::new(ir::Stm::Seq(seq)),
        Box::new(ir::Exp::Temp(pointer)),
    ).into()
}

pub fn translate_seq(seq_exps: Vec<ir::Tree>) -> ir::Tree {

    // Unit is a no-op
    if seq_exps.is_empty() {
        return ir::Exp::Const(0).into()
    }

    let last = seq_exps.pop().unwrap();
    let rest = seq_exps.into_iter()
        .map(|seq_exp| seq_exp.into())
        .collect::<Vec<_>>();

    ir::Exp::ESeq(
        Box::new(ir::Stm::Seq(rest)),
        Box::new(last.into()),
    ).into()
}

pub fn translate_ass(lhs_exp: ir::Tree, rhs_exp: ir::Tree) -> ir::Tree {
    ir::Stm::Move(
        rhs_exp.into(),
        lhs_exp.into()
    ).into()
}

pub fn translate_if(guard_exp: ir::Tree, then_exp: ir::Tree, opt_or_exp: Option<ir::Tree>) -> ir::Tree {

    if let Some(or_exp) = opt_or_exp {

        let t_label = Label::from_str("TRUE_BRANCH");
        let f_label = Label::from_str("FALSE_BRANCH");
        let e_label = Label::from_str("EXIT_IF_ELSE");
        let result = Temp::from_str("IF_ELSE_RESULT");

        ir::Exp::ESeq(
            Box::new(ir::Stm::Seq(vec![

                // Evaluate guard expression and jump to correct branch
                ir::Stm::CJump(
                    guard_exp.into(),
                    ir::Relop::Eq,
                    ir::Exp::Const(0),
                    f_label,
                    t_label,
                ),

                // Move result of true branch
                ir::Stm::Label(t_label),
                ir::Stm::Move(
                    then_exp.into(),
                    ir::Exp::Temp(result),
                ),
                ir::Stm::Jump(
                    ir::Exp::Name(e_label),
                    vec![e_label],
                ),

                // Move result of false branch
                ir::Stm::Label(f_label),
                ir::Stm::Move(
                    or_exp.into(),
                    ir::Exp::Temp(result),
                ),
                ir::Stm::Jump(
                    ir::Exp::Name(e_label),
                    vec![e_label],
                ),

                // Exit branch
                ir::Stm::Label(e_label),
            ])),
            Box::new(ir::Exp::Temp(result)),
        ).into()

    } else {

        let t_label = Label::from_str("TRUE_BRANCH");
        let e_label = Label::from_str("EXIT_IF");

        ir::Stm::Seq(vec![

            // Evaluate guard expression and jumpt to exit if false
            ir::Stm::CJump(
                guard_exp.into(),
                ir::Relop::Eq,
                ir::Exp::Const(0),
                e_label,
                t_label,
            ),

            // Execute branch
            ir::Stm::Label(t_label),
            then_exp.into(),
            ir::Stm::Jump(
                ir::Exp::Name(e_label),
                vec![e_label],
            ),

            // Skip branch
            ir::Stm::Label(e_label),
        ]).into()

    }
}

pub fn translate_while(s_label: Label, guard_exp: ir::Tree, body_exp: ir::Tree) -> ir::Tree {

    let t_label = Label::from_str("TRUE_BRANCH");
    let e_label = Label::from_str("EXIT_WHILE");

    ir::Stm::Seq(vec![

        // Invariant: all labels must be proceeded by a jump
        ir::Stm::Jump(
            ir::Exp::Name(s_label),
            vec![s_label],
        ),

        // While loop header
        ir::Stm::Label(s_label),

        // Evaluate guard expression and jump to exit if false
        ir::Stm::CJump(
            guard_exp.into(),
            ir::Relop::Eq,
            ir::Exp::Const(0),
            e_label,
            t_label,
        ),

        // Execute loop body and repeat
        ir::Stm::Label(t_label),
        body_exp.into(),
        ir::Stm::Jump(
            ir::Exp::Name(s_label),
            vec![s_label],
        ),

        // Exit loop
        ir::Stm::Label(e_label),

    ]).into()
}

pub fn translate_for(
    frames: &mut [Frame],
    s_label: Label,
    name: &Symbol,
    escape: bool,
    lo_exp: ir::Tree,
    hi_exp: ir::Tree,
    body_exp: ir::Tree
) -> ir::Tree {

    let index_location = frames.last_mut()
        .expect("Internal error: missing frame")
        .allocate(*name, escape);

    let t_label = Label::from_str("TRUE_BRANCH");
    let e_label = Label::from_str("EXIT_FOR");

    ir::Stm::Seq(vec![

        // Initialize index variable
        ir::Stm::Move(
            lo_exp.into(),
            index_location.clone(),
        ),

        // Invariant: all labels must be proceeded by a jump
        ir::Stm::Jump(
            ir::Exp::Name(s_label),
            vec![s_label],
        ),

        // Loop header
        ir::Stm::Label(s_label),
        ir::Stm::CJump(
            index_location.clone(),
            ir::Relop::Gt,
            hi_exp.into(),
            e_label,
            t_label,
        ),

        // True branch: execute body and then increment index
        ir::Stm::Label(t_label),
        body_exp.into(),
        ir::Stm::Move(
            ir::Exp::Binop(
                Box::new(index_location.clone()),
                ir::Binop::Add,
                Box::new(ir::Exp::Const(1)),
            ),
            index_location,
        ),
        ir::Stm::Jump(
            ir::Exp::Name(s_label),
            vec![s_label],
        ),

        // Exit label
        ir::Stm::Label(e_label),

    ]).into()
}

pub fn translate_let(dec_exps: Vec<ir::Tree>, body_exp: ir::Tree) -> ir::Tree {
    
    let dec_stms = dec_exps.into_iter()
        .map(|exp| exp.into())
        .collect::<Vec<_>>();

    ir::Exp::ESeq(
        Box::new(ir::Stm::Seq(dec_stms)),
        Box::new(body_exp.into()),
    ).into()

}

pub fn translate_arr(size_exp: ir::Tree, init_exp: ir::Tree) -> ir::Tree {

    let init_array = Label::from_fixed("init_array");

    ir::Exp::Call(
        Box::new(ir::Exp::Name(init_array)),
        vec![
            size_exp.into(),
            init_exp.into()
        ],
    ).into()
}

fn translate_binop(op: &Binop) -> Option<ir::Binop> {
    match op {
    | Binop::Add  => Some(ir::Binop::Add),
    | Binop::Sub  => Some(ir::Binop::Sub),
    | Binop::Mul  => Some(ir::Binop::Mul),
    | Binop::Div  => Some(ir::Binop::Div),
    | Binop::LAnd => Some(ir::Binop::And),
    | Binop::LOr  => Some(ir::Binop::Or),
    _ => None,
    }
}

fn translate_relop(op: &Binop) -> Option<ir::Relop> {
    match op {
    | Binop::Eq  => Some(ir::Relop::Eq),
    | Binop::Neq => Some(ir::Relop::Ne),
    | Binop::Lt  => Some(ir::Relop::Lt),
    | Binop::Le  => Some(ir::Relop::Le),
    | Binop::Gt  => Some(ir::Relop::Gt),
    | Binop::Ge  => Some(ir::Relop::Ge),
    _ => None,
    }
}

pub fn translate_fun_dec(frames: &mut [Frame], label: Label, args: &[FieldDec], body_exp: ir::Tree) -> Option<ir::Tree> {

    // Set up static link as first argument
    let mut all_args = vec![
        (store("STATIC_LINK"), true)
    ];

    // Collect arg names and escapes
    all_args.extend(
        args.iter().map(|arg| (arg.name, arg.escape))
    );

    // Create new frame
    let frame = Frame::new(label, all_args);

    // Push finished function to done pile
    self.done.push(frame.wrap(body_exp));

}
//     | Dec::Var{name, escape, init, ..} => {

//         let init_exp = self.translate_exp(init);
//         let name_exp = self.frames.last_mut()
//             .expect("Internal error: missing frame")
//             .allocate(*name, *escape);

//         Some(
//             ir::Stm::Move(
//                 init_exp.into(),
//                 name_exp.into(),
//             ).into()
//         )
//     }
//     }
// }
