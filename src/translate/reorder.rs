use fnv::FnvHashSet;

use analyze::flow::Flow;
use ir::*;
use operand::Label;

pub fn reorder(unit: Unit) -> Unit {
    unit.map(|function| {
            Function {
                label: function.label,
                body: Flow::new(function.label, function.body).linearize(),
                escapes: function.escapes,
            }
        })
        .map(condense)
        .map(clean)
}

fn condense(function: Function) -> Function {

    let body_len = function.body.len();
    let mut condensed = Vec::new();

    for i in 0..body_len {
        if i == body_len - 1 {
            condensed.push(function.body[i].clone()); 
            break
        }

        match (&function.body[i], &function.body[i + 1]) {
        | (Stm::Jump(Exp::Name(j_label), _), Stm::Label(label)) if j_label == label => (),
        | (Stm::CJump(_, _, _, _, f_label), Stm::Label(label)) if f_label == label => {
            condensed.push(function.body[i].clone())  
        }
        | (Stm::CJump(l, op, r, t_label, f_label), Stm::Label(label)) if t_label == label => {
            condensed.push(Stm::CJump(l.clone(), op.negate(), r.clone(), *f_label, *t_label));
        }
        | (Stm::CJump(l, op, r, t_label, f_label), _) => {
            let label = Label::from_str("CONDENSE_CJUMP");
            condensed.push(Stm::CJump(l.clone(), *op, r.clone(), *t_label, label));
            condensed.push(Stm::Label(label));
            condensed.push(Stm::Jump(Exp::Name(*f_label), vec![*f_label]));
        }
        | _ => {
            condensed.push(function.body[i].clone())  
        },
        }
    }

    Function {
        label: function.label,
        body: condensed,
        escapes: function.escapes,
    }
}

fn clean(function: Function) -> Function {

    let mut used = FnvHashSet::default();

    for stm in &function.body {
        match stm {
        | Stm::Jump(Exp::Name(label), _) => { used.insert(*label); },
        | Stm::CJump(_, _, _, label, _)  => { used.insert(*label); },
        | _ => (),
        }
    }

    function.map(|body| {
        body.into_iter()
            .filter(|stm| {
                match stm {
                | Stm::Label(label) => used.contains(&label),
                | _ => true,
                }
            })
            .collect()
    })
}
