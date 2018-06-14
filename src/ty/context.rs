use codespan::{ByteIndex, ByteSpan};
use im::HashMap;

use error::{Error, TypeError};
use ty::Ty;

pub type Context<T> = HashMap<String, T>;

#[derive(Debug, Clone)]
pub struct VarContext(Context<Binding>);

#[derive(Debug, Clone)]
pub enum Binding {
    Var(Ty, bool),
    Fun(Vec<Ty>, Ty),
}

impl Default for VarContext {
    fn default() -> Self {
        VarContext(
            hashmap! {
                "print".to_string()     => Binding::Fun(vec![Ty::Str], Ty::Unit),
                "flush".to_string()     => Binding::Fun(vec![], Ty::Unit),
                "getchar".to_string()   => Binding::Fun(vec![], Ty::Str),
                "ord".to_string()       => Binding::Fun(vec![Ty::Str], Ty::Int),
                "chr".to_string()       => Binding::Fun(vec![Ty::Int], Ty::Str),
                "size".to_string()      => Binding::Fun(vec![Ty::Str], Ty::Int),
                "substring".to_string() => Binding::Fun(vec![Ty::Str, Ty::Int, Ty::Int], Ty::Str),
                "concat".to_string()    => Binding::Fun(vec![Ty::Str, Ty::Str], Ty::Str),
                "not".to_string()       => Binding::Fun(vec![Ty::Int], Ty::Int),
                "exit".to_string()      => Binding::Fun(vec![Ty::Int], Ty::Unit)
            }
        )
    }
}

impl VarContext {

    pub fn new() -> Self {
        VarContext(HashMap::default())
    }

    pub fn get_var(&self, span: &ByteSpan, name: &str) -> Result<Ty, Error> {
        match self.0.get(name) {
        | None          => Err(Error::semantic(span.clone(), TypeError::UnboundVar)),
        | Some(binding) => match &*binding {
            | Binding::Fun(_, _) => Err(Error::semantic(span.clone(), TypeError::NotVar)),
            | Binding::Var(ty, _) => Ok(ty.clone()),
        },
        }
    }

    pub fn get_fun(&self, span: &ByteSpan, name: &str) -> Result<(Vec<Ty>, Ty), Error> {
        match self.0.get(name) {
        | None          => Err(Error::semantic(span.clone(), TypeError::UnboundFun)),
        | Some(binding) => match &*binding {
            | Binding::Var(_, _)      => Err(Error::semantic(span.clone(), TypeError::NotFun)),
            | Binding::Fun(args, ret) => Ok((args.clone(), ret.clone())),
        },
        }
    }

    pub fn insert(&self, name: String, binding: Binding) -> Self {
        VarContext(self.0.insert(name, binding))
    }

    pub fn insert_mut(&mut self, name: String, binding: Binding) {
        self.0.insert_mut(name, binding)
    }
}

#[derive(Debug, Clone)]
pub struct TypeContext(Context<Ty>);

impl Default for TypeContext {
    fn default() -> Self {
        TypeContext(
            hashmap! {
                "int".to_string()    => Ty::Int,
                "string".to_string() => Ty::Str
            }
        )
    }
}

impl TypeContext {

    pub fn insert(&self, name: String, ty: Ty) -> Self {
        TypeContext(self.0.insert(name, ty))
    }

    pub fn insert_mut(&mut self, name: String, ty: Ty) {
        self.0.insert_mut(name, ty)
    }

    fn trace_partial(&self, ty: &Ty) -> Ty {
        match ty {
        | Ty::Arr(elem, id) => Ty::Arr(Box::new(self.trace_partial(&*elem)), id.clone()),
        | _                 => ty.clone(),
        }
    }

    pub fn get_partial(&self, span: &ByteSpan, name: &str) -> Result<Ty, Error> {
        match self.0.get(name) {
        | None     => Err(Error::semantic(span.clone(), TypeError::UnboundType)),
        | Some(ty) => Ok(self.trace_partial(&*ty)),
        }
    }

    fn dummy_span() -> ByteSpan { ByteSpan::new(ByteIndex(0), ByteIndex(0)) }

    pub fn trace_full(&self, ty: &Ty) -> Ty {
        match ty {
        | Ty::Name(name, opt) => if let Some(ty) = opt { self.trace_full(&*ty) } else { self.get_full(&Self::dummy_span(), name).unwrap() },
        | Ty::Arr(elem, id)  => Ty::Arr(Box::new(self.trace_full(&*elem)), id.clone()),
        | _                  => ty.clone(),
        }
    }

    pub fn get_full(&self, span: &ByteSpan, name: &str) -> Result<Ty, Error> {
        match self.0.get(name) {
        | None => Err(Error::semantic(span.clone(), TypeError::UnboundType)),
        | Some(ty) => Ok(self.trace_full(&*ty)),
        }
    }
}
