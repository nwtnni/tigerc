use codespan::{ByteIndex, ByteSpan};
use fnv::FnvHashMap;

use error::{Error, TypeError};
use ty::Ty;

macro_rules! hashmap {
    ( $( $key:expr => $value:expr ),* ) => {
        {
            let mut map = FnvHashMap::default();
            $(
                map.insert($key, $value);
            )*
            map
        }
    }
}

pub type Context<T> = Vec<FnvHashMap<String, T>>;

#[derive(Debug)]
pub struct VarContext(Context<Binding>);

#[derive(Debug, Clone)]
pub enum Binding {
    Var(Ty, bool),
    Fun(Vec<Ty>, Ty),
}

impl Default for VarContext {
    fn default() -> Self {
        VarContext(vec![
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
        ])
    }
}

impl VarContext {

    pub fn insert(&mut self, name: String, binding: Binding) {
        self.0.last_mut().unwrap().insert(name, binding);
    }

    pub fn push(&mut self) {
        self.0.push(FnvHashMap::default());
    }

    pub fn pop(&mut self) {
        self.0.pop();
    }

    pub fn get_var(&self, span: &ByteSpan, name: &str) -> Result<Ty, Error> {
        for env in self.0.iter().rev() {
            match env.get(name) {
            | Some(Binding::Fun(_, _))  => return Err(Error::semantic(span.clone(), TypeError::NotVar)),
            | Some(Binding::Var(ty, _)) => return Ok(ty.clone()),
            | None                      => (),
            };
        }
        Err(Error::semantic(span.clone(), TypeError::UnboundVar))
    }

    pub fn get_fun(&self, span: &ByteSpan, name: &str) -> Result<(Vec<Ty>, Ty), Error> {
        for env in self.0.iter().rev() {
            match env.get(name) {
            | Some(Binding::Var(_, _))      => return Err(Error::semantic(span.clone(), TypeError::NotFun)),
            | Some(Binding::Fun(args, ret)) => return Ok((args.clone(), ret.clone())),
            | None                          => (),
            }
        }
        Err(Error::semantic(span.clone(), TypeError::UnboundFun))
    }
}

#[derive(Debug)]
pub struct TypeContext(Context<Ty>);

impl Default for TypeContext {
    fn default() -> Self {
        TypeContext(vec![
            hashmap! {
                "int".to_string()    => Ty::Int,
                "string".to_string() => Ty::Str
            }
        ])
    }
}

impl TypeContext {

    pub fn insert(&mut self, name: String, ty: Ty) {
        self.0.last_mut().unwrap().insert(name, ty);
    }

    pub fn push(&mut self) {
        self.0.push(FnvHashMap::default());
    }

    pub fn pop(&mut self) {
        self.0.pop();
    }

    fn trace_partial(&self, ty: &Ty) -> Ty {
        match ty {
        | Ty::Arr(elem, id) => Ty::Arr(Box::new(self.trace_partial(&*elem)), id.clone()),
        | _                 => ty.clone(),
        }
    }

    pub fn get_partial(&self, span: &ByteSpan, name: &str) -> Result<Ty, Error> {
        for env in self.0.iter().rev() {
            if let Some(ty) = env.get(name) { return Ok(self.trace_partial(&*ty)) }
        }
        Err(Error::semantic(span.clone(), TypeError::UnboundType))
    }

    fn dummy_span() -> ByteSpan { ByteSpan::new(ByteIndex(0), ByteIndex(0)) }

    pub fn trace_full(&self, span: &ByteSpan, ty: &Ty) -> Result<Ty, Error> {
        match ty {
        | Ty::Name(name, opt) => {
            match opt {
            | Some(box Ty::Name(_, _)) => Err(Error::semantic(span.clone(), TypeError::NotIndirect)),
            | Some(box ty)             => self.trace_full(span, &ty),
            | _                        => Ok(self.get_full(&Self::dummy_span(), name).unwrap()),
            }
        },
        | Ty::Arr(elem, id) => Ok(Ty::Arr(Box::new(self.trace_full(span, &*elem)?), id.clone())),
        | _                 => Ok(ty.clone()),
        }
    }

    pub fn get_full(&self, span: &ByteSpan, name: &str) -> Result<Ty, Error> {
        for env in self.0.iter().rev() {
            if let Some(ty) = env.get(name) { return Ok(self.trace_full(span, &*ty)?) }
        }
        Err(Error::semantic(span.clone(), TypeError::UnboundType))
    }
}
