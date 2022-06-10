/*!
An implementation of `sval`'s type system.

This library is a playground for validating the rules of `sval`'s type system. The goal is to
implement the system using as few special rules as possible, so it can be described in something of
a complete and concise way.
*/

mod eval;

use crate::eval::Evaluator;
use std::borrow::Cow;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Simple(SimpleType),
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
    Enum {
        variants: HashMap<Id, Option<Type>>,
    },
}

pub use sval::Id;

// TODO: Consider killing this off and adding `Cow` support to `sval::Label`
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Label(Cow<'static, str>);

impl Label {
    pub fn new(value: impl Into<Cow<'static, str>>) -> Self {
        Label(value.into())
    }
}

impl<'a> From<sval::Label<'a>> for Label {
    fn from(label: sval::Label<'a>) -> Label {
        if let Some(label) = label.try_get_static() {
            Label(Cow::Borrowed(label))
        } else {
            Label(Cow::Owned((*label).to_owned()))
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SimpleType {
    Dynamic,
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

    pub fn dynamic() -> Self {
        Type::Simple(SimpleType::Dynamic)
    }

    pub fn record(
        id: Option<Id>,
        label: Option<Label>,
        values: impl IntoIterator<Item = (Label, Type)>,
    ) -> Self {
        Type::Record {
            id,
            label,
            values: values
                .into_iter()
                .map(|(label, ty)| (label, Some(ty)))
                .collect(),
        }
    }

    pub fn id(&self) -> Option<Id> {
        match self {
            Type::Record { id, .. } => *id,
            _ => None,
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
    evaluator: Option<Evaluator>,
}

impl Context {
    pub fn new() -> Self {
        Context {
            evaluator: Some(Evaluator::empty()),
        }
    }

    pub fn eval(&mut self, v: impl sval::Value) -> &Type {
        let evaluator = self.evaluator.take().expect("evaluator was poisoned");

        self.evaluator = Some(evaluator.eval(v).expect("failed to evaluate"));

        self.get_root().expect("evaluation didn't produce a type")
    }

    pub fn get_root(&self) -> Option<&Type> {
        self.evaluator.as_ref().and_then(|eval| eval.get_type())
    }

    pub fn clear(&mut self) {
        if let Some(ref mut evaluator) = self.evaluator {
            evaluator.clear();
        } else {
            self.evaluator = Some(Evaluator::empty())
        }
    }
}

pub fn type_of_val(v: impl sval::Value) -> Type {
    Evaluator::type_of_val(v)
}

#[cfg(test)]
mod tests;
