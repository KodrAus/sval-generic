use std::collections::HashMap;
use sval::{Id, Label};

fn main() {
    Context::new().eval(());

    Context::new().eval({
        let mut map = HashMap::new();
        map.insert((), ());
        map
    });

    Context::new().eval({
        let mut map = HashMap::new();
        map.insert((), {
            let mut map = HashMap::new();
            map.insert((), ());
            map
        });
        map
    });

    let mut ctxt = Context::new();

    ctxt.eval(());
    ctxt.eval(());

    ctxt.eval({
        let mut map = HashMap::new();
        map.insert((), ());
        map
    });
}

#[derive(Debug)]
pub struct Context {
    result: Option<EvalType>,
    stack: Vec<Option<EvalType>>,
}

#[derive(Debug)]
struct EvalType {
    state: State,
    ty: Type,
}

#[derive(Debug)]
pub enum Type {
    Simple(SimpleType),
    Map {
        key: Box<Option<Type>>,
        value: Box<Option<Type>>,
    },
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SimpleType {
    Unit,
}

#[derive(Debug)]
enum TypeBuilder {
    Simple(SimpleType),
    Map,
}

impl From<SimpleType> for TypeBuilder {
    fn from(ty: SimpleType) -> Self {
        TypeBuilder::Simple(ty)
    }
}

impl TypeBuilder {
    fn build(self) -> Type {
        match self {
            TypeBuilder::Simple(ty) => Type::Simple(ty),
            TypeBuilder::Map => Type::Map {
                key: Box::new(None),
                value: Box::new(None),
            },
        }
    }
}

impl Type {
    fn compatible_with(&self, builder: &TypeBuilder) -> bool {
        match (self, builder) {
            (Type::Map { .. }, TypeBuilder::Map) => true,
            (Type::Simple(a), TypeBuilder::Simple(b)) => a == b,
            _ => false,
        }
    }
}

#[derive(Debug, PartialEq)]
enum State {
    Ready,
    MapKey,
    MapValue,
}

impl Context {
    pub fn new() -> Self {
        Context {
            result: None,
            stack: Vec::new(),
        }
    }

    pub fn eval(&mut self, v: impl sval::Value) {
        v.stream(self).expect("failed to stream");
        assert!(self.stack.is_empty(), "unexpected end of input");

        println!("{:?}", self);
    }

    fn value_mut(&mut self) -> &mut Option<EvalType> {
        if let Some(current) = self.stack.last_mut() {
            current
        } else {
            &mut self.result
        }
    }

    fn pop_eval_value(&mut self) -> Option<EvalType> {
        self.stack
            .pop()
            .and_then(|v| v)
            .or_else(|| self.result.take())
    }

    fn pop_value(&mut self) -> Type {
        let value = self.pop_eval_value();

        match value {
            Some(EvalType {
                state: State::Ready,
                ty,
            }) => ty,
            invalid => panic!("cannot pop {:?}", invalid),
        }
    }

    fn infer(&mut self, ty: impl Into<TypeBuilder>) -> &mut Option<EvalType> {
        let ty = ty.into();

        match self.value_mut() {
            empty @ None => {
                *empty = Some(EvalType {
                    state: State::Ready,
                    ty: ty.build(),
                });

                empty
            }
            inferred @ Some(EvalType {
                state: State::Ready,
                ..
            }) => {
                let check = &inferred.as_mut().expect("missing type").ty;

                assert!(
                    check.compatible_with(&ty),
                    "expected {:?}, got {:?}",
                    check,
                    ty,
                );

                inferred
            }
            inferred => panic!("expected {:?}, got {:?}", inferred, ty),
        }
    }

    fn pop_infer(&mut self, ty: impl Into<TypeBuilder>) -> Option<EvalType> {
        self.infer(ty);
        self.pop_eval_value()
    }

    fn push_map(&mut self) {
        let map = self.pop_infer(TypeBuilder::Map);

        self.stack.push(map);
    }

    fn push_map_key(&mut self) {
        match self.value_mut() {
            Some(EvalType {
                state,
                ty: Type::Map { key, .. },
            }) => {
                assert_eq!(*state, State::Ready, "unexpected map key");
                *state = State::MapKey;

                let key = key.take().map(|ty| EvalType {
                    state: State::Ready,
                    ty,
                });

                self.stack.push(key);
            }
            v => panic!("expected map, got {:?}", v),
        }
    }

    fn pop_map_key(&mut self) {
        let restore = self.pop_value();

        match self.value_mut() {
            Some(EvalType {
                state,
                ty: Type::Map { key, .. },
            }) => {
                assert_eq!(*state, State::MapKey, "unexpected map key");

                **key = Some(restore);
            }
            v => panic!("expected map, got {:?}", v),
        }
    }

    fn push_map_value(&mut self) {
        match self.value_mut() {
            Some(EvalType {
                state,
                ty: Type::Map { value, .. },
            }) => {
                assert_eq!(*state, State::MapKey, "unexpected map value");
                *state = State::MapValue;

                let value = value.take().map(|ty| EvalType {
                    state: State::Ready,
                    ty,
                });

                self.stack.push(value);
            }
            v => panic!("expected map, got {:?}", v),
        }
    }

    fn pop_map_value(&mut self) {
        let restore = self.pop_value();

        match self.value_mut() {
            Some(EvalType {
                state,
                ty: Type::Map { value, .. },
            }) => {
                assert_eq!(
                    *state,
                    State::MapValue,
                    "failed to restore {:?}: unexpected map value",
                    restore
                );
                *state = State::Ready;

                **value = Some(restore);
            }
            v => panic!("failed to restore {:?}: expected map, got {:?}", restore, v),
        }
    }

    fn pop_map(&mut self) {
        let depth = self.stack.len();

        match self.value_mut() {
            Some(EvalType {
                state,
                ty: Type::Map { .. },
            }) if depth > 1 => {
                assert_eq!(*state, State::Ready, "unexpected end of map");
            }
            _ => {
                let restore = self.pop_value();

                match self.value_mut() {
                    empty @ None => {
                        *empty = Some(EvalType {
                            state: State::Ready,
                            ty: restore,
                        });
                    }
                    v => panic!("attempt to restore map into {:?}", v),
                }
            }
        }
    }
}

impl<'sval> sval::Stream<'sval> for Context {
    fn is_text_based(&self) -> bool {
        todo!()
    }

    fn unit(&mut self) -> sval::Result {
        self.infer(SimpleType::Unit);

        Ok(())
    }

    fn null(&mut self) -> sval::Result {
        todo!()
    }

    fn bool(&mut self, value: bool) -> sval::Result {
        todo!()
    }

    fn text_begin(&mut self, num_bytes_hint: Option<usize>) -> sval::Result {
        todo!()
    }

    fn text_fragment(&mut self, fragment: &'sval str) -> sval::Result {
        todo!()
    }

    fn text_fragment_computed(&mut self, fragment: &str) -> sval::Result {
        todo!()
    }

    fn text_end(&mut self) -> sval::Result {
        todo!()
    }

    fn binary_begin(&mut self, num_bytes_hint: Option<usize>) -> sval::Result {
        todo!()
    }

    fn binary_fragment(&mut self, fragment: &'sval [u8]) -> sval::Result {
        todo!()
    }

    fn binary_fragment_computed(&mut self, fragment: &[u8]) -> sval::Result {
        todo!()
    }

    fn binary_end(&mut self) -> sval::Result {
        todo!()
    }

    fn u8(&mut self, value: u8) -> sval::Result {
        todo!()
    }

    fn u16(&mut self, value: u16) -> sval::Result {
        todo!()
    }

    fn u32(&mut self, value: u32) -> sval::Result {
        todo!()
    }

    fn u64(&mut self, value: u64) -> sval::Result {
        todo!()
    }

    fn u128(&mut self, value: u128) -> sval::Result {
        todo!()
    }

    fn i8(&mut self, value: i8) -> sval::Result {
        todo!()
    }

    fn i16(&mut self, value: i16) -> sval::Result {
        todo!()
    }

    fn i32(&mut self, value: i32) -> sval::Result {
        todo!()
    }

    fn i64(&mut self, value: i64) -> sval::Result {
        todo!()
    }

    fn i128(&mut self, value: i128) -> sval::Result {
        todo!()
    }

    fn f32(&mut self, value: f32) -> sval::Result {
        todo!()
    }

    fn f64(&mut self, value: f64) -> sval::Result {
        todo!()
    }

    fn map_begin(&mut self, _: Option<usize>) -> sval::Result {
        self.push_map();
        Ok(())
    }

    fn map_key_begin(&mut self) -> sval::Result {
        self.push_map_key();
        Ok(())
    }

    fn map_key_end(&mut self) -> sval::Result {
        self.pop_map_key();
        Ok(())
    }

    fn map_value_begin(&mut self) -> sval::Result {
        self.push_map_value();
        Ok(())
    }

    fn map_value_end(&mut self) -> sval::Result {
        self.pop_map_value();
        Ok(())
    }

    fn map_end(&mut self) -> sval::Result {
        self.pop_map();
        Ok(())
    }

    fn seq_begin(&mut self, num_entries_hint: Option<usize>) -> sval::Result {
        todo!()
    }

    fn seq_value_begin(&mut self) -> sval::Result {
        todo!()
    }

    fn seq_value_end(&mut self) -> sval::Result {
        todo!()
    }

    fn seq_end(&mut self) -> sval::Result {
        todo!()
    }

    fn dynamic_begin(&mut self) -> sval::Result {
        todo!()
    }

    fn dynamic_end(&mut self) -> sval::Result {
        todo!()
    }

    fn enum_begin(&mut self, id: Option<Id>, label: Option<Label>) -> sval::Result {
        todo!()
    }

    fn enum_end(&mut self, id: Option<Id>, label: Option<Label>) -> sval::Result {
        todo!()
    }

    fn tagged_begin(&mut self, id: Option<Id>, label: Option<Label>) -> sval::Result {
        todo!()
    }

    fn tagged_end(&mut self, id: Option<Id>, label: Option<Label>) -> sval::Result {
        todo!()
    }

    fn constant_begin(&mut self, id: Option<Id>, label: Option<Label>) -> sval::Result {
        todo!()
    }

    fn constant_end(&mut self, id: Option<Id>, label: Option<Label>) -> sval::Result {
        todo!()
    }

    fn record_begin(
        &mut self,
        id: Option<Id>,
        label: Option<Label>,
        num_entries_hint: Option<usize>,
    ) -> sval::Result {
        todo!()
    }

    fn record_value_begin(&mut self, label: Label) -> sval::Result {
        todo!()
    }

    fn record_value_end(&mut self, label: Label) -> sval::Result {
        todo!()
    }

    fn record_end(&mut self, id: Option<Id>, label: Option<Label>) -> sval::Result {
        todo!()
    }

    fn tuple_begin(
        &mut self,
        id: Option<Id>,
        label: Option<Label>,
        num_entries_hint: Option<usize>,
    ) -> sval::Result {
        todo!()
    }

    fn tuple_value_begin(&mut self, id: Id) -> sval::Result {
        todo!()
    }

    fn tuple_value_end(&mut self, id: Id) -> sval::Result {
        todo!()
    }

    fn tuple_end(&mut self, id: Option<Id>, label: Option<Label>) -> sval::Result {
        todo!()
    }

    fn optional_some_begin(&mut self) -> sval::Result {
        todo!()
    }

    fn optional_some_end(&mut self) -> sval::Result {
        todo!()
    }

    fn optional_none(&mut self) -> sval::Result {
        todo!()
    }

    fn fixed_size_begin(&mut self) -> sval::Result {
        todo!()
    }

    fn fixed_size_end(&mut self) -> sval::Result {
        todo!()
    }

    fn int_begin(&mut self) -> sval::Result {
        todo!()
    }

    fn int_end(&mut self) -> sval::Result {
        todo!()
    }

    fn binfloat_begin(&mut self) -> sval::Result {
        todo!()
    }

    fn binfloat_end(&mut self) -> sval::Result {
        todo!()
    }

    fn decfloat_begin(&mut self) -> sval::Result {
        todo!()
    }

    fn decfloat_end(&mut self) -> sval::Result {
        todo!()
    }
}
