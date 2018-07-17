use std::fmt;

use simple_symbol::Symbol;

generate_counter!(TyID, usize);

#[derive(Debug, Eq, Clone)]
pub enum Ty {
    Nil,
    Int,
    Str,
    Unit,
    Arr(Box<Ty>, usize),
    Rec(Vec<(Symbol, Ty)>, usize),
    Name(Symbol, Option<Box<Ty>>),
}

impl Ty {
    pub fn subtypes(&self, rhs: &Self) -> bool {
        match (self, rhs) {
        | (Ty::Nil, Ty::Rec(_, _)) => true,
        | _                        => self == rhs,
        }
    }

    pub fn is_int(&self) -> bool {
        *self == Ty::Int
    }

    pub fn is_unit(&self) -> bool {
        *self == Ty::Unit
    }
}

impl PartialEq for Ty {
    fn eq(&self, rhs: &Self) -> bool {
        match (self, rhs) {
        | (Ty::Int, Ty::Int)
        | (Ty::Str, Ty::Str)
        | (Ty::Nil, Ty::Nil)
        | (Ty::Unit, Ty::Unit) => true,
        | (Ty::Arr(_, lid), Ty::Arr(_, rid))
        | (Ty::Rec(_, lid), Ty::Rec(_, rid)) => lid == rid,
        | (Ty::Name(_, _), _)
        | (_, Ty::Name(_, _)) => panic!("Internal error: should never compare names"),
        _ => false,
        }
    }
}

impl fmt::Display for Ty {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
        | Ty::Nil            => write!(fmt, "nil"),
        | Ty::Int            => write!(fmt, "int"),
        | Ty::Str            => write!(fmt, "string"),
        | Ty::Unit           => write!(fmt, "unit"),
        | Ty::Name(name, _)  => write!(fmt, "{}", name),
        | Ty::Arr(ty, _)     => write!(fmt, "array of {}", ty),
        | Ty::Rec(fields, _) => {
            write!(fmt, "{{ ").unwrap();
            for (name, ty) in fields {
                write!(fmt, "{} : {}, ", name, ty).unwrap();
            }
            write!(fmt, "}}")
        }
        }
    }
}
