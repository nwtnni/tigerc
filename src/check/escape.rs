use fnv::FnvHashMap;
use sym::Symbol;

use ast::*;

type Escaped = FnvHashMap<Symbol, usize>;

pub fn trap_ast(ast: &mut Exp) {
    let mut escaped = FnvHashMap::default();
    trap_exp(0, &mut escaped, ast);
}

fn trap_name(depth: usize, escaped: &mut Escaped, name: &Symbol, escape: &mut bool) {
    if let Some(&usage) = escaped.get(name) {
        *escape = usage > depth; 
        escaped.remove(name);
    } else {
        *escape = false;
    }
}

fn trap_var(depth: usize, escaped: &mut Escaped, var: &mut Var) {
    match var {
    | Var::Simple(name, _) => {
        escaped.insert(*name, depth);
    },
    | Var::Field(rec, _, _, _) => {
        trap_var(depth, escaped, rec)
    },
    | Var::Index(arr, index, _) => {
        trap_var(depth, escaped, arr);
        trap_exp(depth, escaped, index);
    },
    }
}

fn trap_exp(depth: usize, escaped: &mut Escaped, exp: &mut Exp) {

    macro_rules! recurse {
        ($exp:expr) => { trap_exp(depth, escaped, $exp); }
    }

    match exp {
    | Exp::Nil(_)
    | Exp::Int(_, _)
    | Exp::Str(_, _)
    | Exp::Break(_) => (),
    | Exp::Var(var, _) => trap_var(depth, escaped, var),
    | Exp::Neg(neg, _) => recurse!(neg),
    | Exp::Call{args, ..} => {
        for arg in args { recurse!(arg) }
    },
    | Exp::Bin{lhs, rhs, ..} => {
        recurse!(lhs);
        recurse!(rhs)
    },
    | Exp::Rec{fields, ..} => {
        for field in fields { recurse!(&mut *field.exp) }
    },
    | Exp::Seq(statements, _) => {
        for statement in statements { recurse!(statement) }
    },
    | Exp::Ass{name, exp, ..} => {
        trap_var(depth, escaped, name);
        recurse!(exp)
    },
    | Exp::If{guard, then, or, ..} => {
        recurse!(guard);
        recurse!(then);
        if let Some(or) = or { recurse!(or) }
    },
    | Exp::While{guard, body, ..} => {
        recurse!(guard);
        recurse!(body);
    },
    | Exp::For{name, escape, lo, hi, body, ..} => {
        recurse!(lo);
        recurse!(hi);
        recurse!(body);
        trap_name(depth, escaped, name, escape);
    },
    | Exp::Let{decs, body, ..} => {
        recurse!(body);
        for dec in decs { trap_dec(depth, escaped, dec); }
    },
    | Exp::Arr{size, init, ..} => {
        recurse!(size);
        recurse!(init);
    },
    }

}

fn trap_dec(depth: usize, escaped: &mut Escaped, dec: &mut Dec) {

    match dec {
    | Dec::Fun(funs, _) => {

        // Increase static nesting depth
        let depth = depth + 1;

        // Check function bodies for usage of arguments
        for fun in funs {
            trap_exp(depth, escaped, &mut fun.body); 
            for arg in &mut fun.args {
                trap_name(depth, escaped, &arg.name, &mut arg.escape);
            }
        }
    },
    | Dec::Var{name, escape, init, ..} => {
        trap_exp(depth, escaped, init);
        trap_name(depth, escaped, name, escape);
    },
    _ => (),
    }

}
