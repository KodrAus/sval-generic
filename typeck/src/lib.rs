mod eval;

use crate::eval::Evaluator;
use std::borrow::Cow;

#[derive(Debug, PartialEq)]
pub enum Type {
    Simple(SimpleType),
    // TODO: Consider an `EmptyMap` type
    Map {
        key: Box<Option<Type>>,
        value: Box<Option<Type>>,
    },
    Seq {
        value: Box<Option<Type>>,
    },
    Record {
        id: Option<Id>,
        label: Option<Label>,
        values: Vec<(Label, Option<Type>)>,
    },
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Id([u8; 16]);

impl From<sval::Id> for Id {
    fn from(id: sval::Id) -> Id {
        Id(id.get())
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Label(Cow<'static, str>);

impl<'a> From<sval::Label<'a>> for Label {
    fn from(label: sval::Label<'a>) -> Label {
        if let Some(label) = label.try_get_static() {
            Label(Cow::Borrowed(label))
        } else {
            Label(Cow::Owned(label.get().to_owned()))
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SimpleType {
    Unit,
    Null,
    Text,
    Binary,
    U8,
    U16,
    U32,
    U64,
    U128,
    I8,
    I16,
    I32,
    I64,
    I128,
    F32,
    F64,
    Bool,
}

impl Type {
    pub fn unit() -> Self {
        Type::Simple(SimpleType::Unit)
    }

    pub fn null() -> Self {
        Type::Simple(SimpleType::Null)
    }

    pub fn text() -> Self {
        Type::Simple(SimpleType::Text)
    }

    pub fn binary() -> Self {
        Type::Simple(SimpleType::Binary)
    }

    pub fn u8() -> Self {
        Type::Simple(SimpleType::U8)
    }

    pub fn u16() -> Self {
        Type::Simple(SimpleType::U16)
    }

    pub fn u32() -> Self {
        Type::Simple(SimpleType::U32)
    }

    pub fn u64() -> Self {
        Type::Simple(SimpleType::U64)
    }

    pub fn u128() -> Self {
        Type::Simple(SimpleType::U128)
    }

    pub fn i8() -> Self {
        Type::Simple(SimpleType::I8)
    }

    pub fn i16() -> Self {
        Type::Simple(SimpleType::I16)
    }

    pub fn i32() -> Self {
        Type::Simple(SimpleType::I32)
    }

    pub fn i64() -> Self {
        Type::Simple(SimpleType::I64)
    }

    pub fn i128() -> Self {
        Type::Simple(SimpleType::I128)
    }

    pub fn f32() -> Self {
        Type::Simple(SimpleType::F32)
    }

    pub fn f64() -> Self {
        Type::Simple(SimpleType::F64)
    }

    pub fn bool() -> Self {
        Type::Simple(SimpleType::Bool)
    }

    pub fn empty_map() -> Self {
        Type::Map {
            key: Box::new(None),
            value: Box::new(None),
        }
    }

    pub fn map(key: Type, value: Type) -> Self {
        Type::Map {
            key: Box::new(Some(key)),
            value: Box::new(Some(value)),
        }
    }

    pub fn empty_seq() -> Self {
        Type::Seq {
            value: Box::new(None),
        }
    }

    pub fn seq(value: Type) -> Self {
        Type::Seq {
            value: Box::new(Some(value)),
        }
    }

    pub fn is_complete(&self) -> bool {
        match self {
            Type::Seq { value } if value.is_some() => true,
            Type::Map { key, value } if key.is_some() && value.is_some() => true,
            Type::Simple(_) => true,
            _ => false,
        }
    }
}

#[derive(Debug)]
pub struct Context {
    evaluator: Evaluator,
}

impl Context {
    pub fn new() -> Self {
        Context {
            evaluator: Evaluator::new(),
        }
    }

    pub fn eval(&mut self, v: impl sval::Value) -> &Type {
        self.evaluator.eval(v)
    }

    pub fn get_root(&self) -> Option<&Type> {
        self.evaluator.get_root()
    }

    pub fn clear(&mut self) {
        self.evaluator.clear()
    }
}

#[cfg(test)]
mod tests;
