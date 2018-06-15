use codespan::{ByteIndex, ByteSpan};

use ast::*;

macro_rules! impl_into_span {
    ($type:ident) => {
        impl IntoSpan for $type {
            fn into_span(&self) -> ByteSpan { self.span }
        }
    }
}

pub trait IntoSpan {
    fn into_span(&self) -> ByteSpan;
}

impl IntoSpan for (ByteIndex, ByteIndex) {
    fn into_span(&self) -> ByteSpan {
        ByteSpan::new(self.0, self.1)
    }
}

impl IntoSpan for Dec {
    fn into_span(&self) -> ByteSpan {
        match self {
        | Dec::Var{span, ..}
        | Dec::Fun(_, span)
        | Dec::Type(_, span) => *span,
        }
    }
}

impl_into_span!(FunDec);
impl_into_span!(FieldDec);
impl_into_span!(TypeDec);
impl_into_span!(Field);

impl IntoSpan for Type {
    fn into_span(&self) -> ByteSpan {
        match self {
        | Type::Name(_, span)
        | Type::Rec(_, span)
        | Type::Arr(_, _, span) => *span,
        }
    }
}

impl IntoSpan for Var {
    fn into_span(&self) -> ByteSpan {
        match self {
        | Var::Simple(_, span)
        | Var::Field(_, _, _, span)
        | Var::Index(_, _, span) => *span,
        }
    }
}

impl IntoSpan for Exp {
    fn into_span(&self) -> ByteSpan {
        match self {
        | Exp::Break(span)
        | Exp::Nil(span)
        | Exp::Var(_, span)
        | Exp::Int(_, span)
        | Exp::Str(_, span)
        | Exp::Call{span, ..}
        | Exp::Neg(_, span)
        | Exp::Bin{span, ..}
        | Exp::Rec{span, ..}
        | Exp::Seq(_, span)
        | Exp::Ass{span, ..}
        | Exp::If{span, ..}
        | Exp::While{span, ..}
        | Exp::For{span, ..}
        | Exp::Let{span, ..}
        | Exp::Arr{span, ..} => *span,
        }
    }
}
