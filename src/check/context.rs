use fnv::FnvHashMap;
use simple_symbol::{store, Symbol};

use ty::Ty;
use operand::Label;
use error::{Error, TypeError};
use span::Span;

pub type Context<T> = Vec<FnvHashMap<Symbol, T>>;

#[derive(Debug)]
pub struct VarContext(Context<Binding>);

#[derive(Debug, Clone)]
pub enum Binding {
    Var(Ty),
    Fun(Vec<Ty>, Ty, Label),
    Ext(Vec<Ty>, Ty, Label),
}

impl Default for VarContext {
    fn default() -> Self {
        VarContext(vec![
            hashmap! {
                store("prints")     => Binding::Ext(vec![Ty::Str], Ty::Unit, Label::from_fixed("__prints__")),
                store("printi")    => Binding::Ext(vec![Ty::Int], Ty::Unit, Label::from_fixed("__printi__")),
                store("flush")     => Binding::Ext(vec![], Ty::Unit, Label::from_fixed("__flush__")),
                store("getchar")   => Binding::Ext(vec![], Ty::Str, Label::from_fixed("__getchar__")),
                store("ord")       => Binding::Ext(vec![Ty::Str], Ty::Int, Label::from_fixed("__ord__")),
                store("chr")       => Binding::Ext(vec![Ty::Int], Ty::Str, Label::from_fixed("__chr__")),
                store("size")      => Binding::Ext(vec![Ty::Str], Ty::Int, Label::from_fixed("__size__")),
                store("substring") => Binding::Ext(vec![Ty::Str, Ty::Int, Ty::Int], Ty::Str, Label::from_fixed("__substring__")),
                store("concat")    => Binding::Ext(vec![Ty::Str, Ty::Str], Ty::Str, Label::from_fixed("__concat__")),
                store("not")       => Binding::Ext(vec![Ty::Int], Ty::Int, Label::from_fixed("__not__")),
                store("exit")      => Binding::Ext(vec![Ty::Int], Ty::Unit, Label::from_fixed("__exit__"))
            }
        ])
    }
}

impl VarContext {

    pub fn insert(&mut self, name: Symbol, binding: Binding) {
        self.0.last_mut().unwrap().insert(name, binding);
    }

    pub fn push(&mut self) {
        self.0.push(FnvHashMap::default());
    }

    pub fn pop(&mut self) {
        self.0.pop().expect("Internal error: no variable context");
    }

    pub fn get_var(&self, span: &Span, name: &Symbol) -> Result<Ty, Error> {
        for env in self.0.iter().rev() {
            match env.get(name) {
            | Some(Binding::Var(ty))   => return Ok(ty.clone()),
            | Some(_)                  => return Err(Error::semantic(*span, TypeError::NotVar)),
            | None                     => (),
            };
        }
        Err(Error::semantic(*span, TypeError::UnboundVar))
    }

    pub fn get_fun(&self, span: &Span, name: &Symbol) -> Result<Binding, Error> {
        for env in self.0.iter().rev() {
            match env.get(name) {
            | Some(Binding::Var(_)) => return Err(Error::semantic(*span, TypeError::NotFun)),
            | Some(binding)         => return Ok(binding.clone()),
            | _                     => (),
            }
        }
        Err(Error::semantic(*span, TypeError::UnboundFun))
    }
}

#[derive(Debug)]
pub struct TypeContext(Context<Ty>);

impl Default for TypeContext {
    fn default() -> Self {
        TypeContext(vec![
            hashmap! {
                store("int")    => Ty::Int,
                store("string") => Ty::Str
            }
        ])
    }
}

impl TypeContext {

    pub fn insert(&mut self, name: Symbol, ty: Ty) {
        self.0.last_mut().unwrap().insert(name, ty);
    }

    pub fn push(&mut self) {
        self.0.push(FnvHashMap::default());
    }

    pub fn pop(&mut self) {
        self.0.pop().expect("Internal error: no type context");
    }

    fn trace_partial(&self, ty: &Ty) -> Ty {
        match ty {
        | Ty::Arr(elem, id) => Ty::Arr(Box::new(self.trace_partial(&*elem)), *id),
        | _                 => ty.clone(),
        }
    }

    pub fn get_partial(&self, span: &Span, name: &Symbol) -> Result<Ty, Error> {
        for env in self.0.iter().rev() {
            if let Some(ty) = env.get(name) { return Ok(self.trace_partial(&*ty)) }
        }
        Err(Error::semantic(*span, TypeError::UnboundType))
    }

    pub fn trace_full(&self, span: &Span, ty: &Ty) -> Result<Ty, Error> {
        match ty {
        | Ty::Name(name, opt) => {
            match opt {
            | Some(ty) => self.trace_full(span, &*ty),
            | _        => Ok(self.get_full(span, name).unwrap()),
            }
        },
        | Ty::Arr(elem, id) => Ok(Ty::Arr(Box::new(self.trace_full(span, &*elem)?), *id)),
        | _                 => Ok(ty.clone()),
        }
    }

    pub fn get_full(&self, span: &Span, name: &Symbol) -> Result<Ty, Error> {
        for env in self.0.iter().rev() {
            if let Some(ty) = env.get(name) { return Ok(self.trace_full(span, &*ty)?) }
        }
        Err(Error::semantic(*span, TypeError::UnboundType))
    }
}
