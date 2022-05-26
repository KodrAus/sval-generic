use std::collections::HashMap;

use crate::{Id, SimpleType, Type};

#[derive(Debug)]
pub struct Evaluator {
    root: Option<ContextType>,
    stack: Vec<Option<ContextType>>,
}

impl Evaluator {
    pub fn new() -> Self {
        Evaluator {
            root: None,
            stack: Vec::new(),
        }
    }

    pub fn eval(&mut self, v: impl sval::Value) -> &Type {
        v.stream(self).expect("failed to stream");
        assert!(self.stack.is_empty(), "unexpected end of input");

        self.get_root().expect("value didn't produce a type")
    }

    pub fn get_root(&self) -> Option<&Type> {
        self.root.as_ref().map(|result| &result.ty)
    }

    pub fn clear(&mut self) {
        self.root = None;
        self.stack.clear();
    }

    fn path(&self) -> String {
        let mut path = String::new();

        let parts = self
            .stack
            .iter()
            .filter_map(|ty| ty.as_ref())
            .filter_map(|ty| match ty {
                ContextType {
                    ty: Type::Map { .. },
                    ..
                } => Some("map"),
                ContextType {
                    ty: Type::Seq { .. },
                    ..
                } => Some("seq"),
                _ => None,
            });

        let mut first = true;
        for part in parts {
            if !first {
                path.push_str(".");
            }

            first = false;
            path.push_str(part);
        }

        path
    }

    fn current_mut(&mut self) -> &mut Option<ContextType> {
        if let Some(current) = self.stack.last_mut() {
            current
        } else {
            &mut self.root
        }
    }

    fn push(&mut self, ty: Option<ContextType>) {
        self.stack.push(ty);
    }

    fn pop(&mut self) -> Option<ContextType> {
        self.stack
            .pop()
            .and_then(|v| v)
            .or_else(|| self.root.take())
    }

    fn infer_begin<'a>(&mut self, builder: impl Into<TypeBuilder<'a>>) {
        let builder = builder.into();

        // For a global, the type we should end up with in the root of our context will
        // be a `Type::Global(Id)`, which we can then lookup in the global set.
        // The way we could do this is by checking whether a builder belongs to the global set
        // or not instead of whether it's chunked?

        // NOTE: How would we deal with self-referential types?
        // If a global may contain a global?
        // We'd need a way to get a copy of it. We're basically re-entering
        // the current value so we can define it. Maybe this actually works out ok?
        // We won't find it in the global list because we've moved it, so we'll start
        // building a fresh one. Once we return it we'll see that a version exists and will
        // merge ourselves into it. We need to make sure that new information we discover
        // is added to information that was already there. This should be possible because
        // we only discover one new thing at a path at a time.
        match self.current_mut() {
            empty @ None => {
                *empty = Some(ContextType {
                    state: State::Valid,
                    ty: builder.build(),
                });
            }
            inferred @ Some(ContextType {
                state: State::Valid,
                ..
            }) => {
                let check = &inferred.as_mut().expect("missing type").ty;

                assert!(
                    builder.is_compatible_with(&check),
                    "expected {:?}, got {:?}",
                    check,
                    builder,
                );
            }
            inferred => panic!("expected {:?}, got {:?}", inferred, builder),
        }

        if builder.is_chunked() {
            let ty = self.pop();
            self.push(ty);
        }
    }

    fn infer_end<'a>(&mut self, builder: impl Into<TypeBuilder<'a>>) {
        let builder = builder.into();
        let depth = self.stack.len();

        match self.current_mut() {
            Some(ContextType { ty, .. }) if builder.is_chunked() && depth > 1 => {
                assert!(
                    builder.is_compatible_with(&ty),
                    "expected {:?}, got {:?}",
                    ty,
                    builder,
                );
            }
            _ => {
                let ended = self.pop().expect("unexpected end of value");

                assert!(
                    ended.is_valid(),
                    "attempt to restore an invalid type {:?}",
                    ended
                );

                assert!(
                    builder.is_compatible_with(&ended.ty),
                    "expected {:?}, got {:?}",
                    ended.ty,
                    builder,
                );

                match self.current_mut() {
                    empty @ None => {
                        *empty = Some(ended);
                    }
                    v => panic!("attempt to restore value into {:?}", v),
                }
            }
        }
    }

    fn push_text(&mut self) {
        self.infer_begin(SimpleType::Text);
    }

    fn text_fragment(&mut self) {
        match self.current_mut() {
            Some(ContextType {
                ty: Type::Simple(SimpleType::Text),
                state: _,
            }) => (),
            v => panic!("expected text, got {:?}", v),
        }
    }

    fn pop_text(&mut self) {
        self.infer_end(SimpleType::Text);
    }

    fn push_binary(&mut self) {
        self.infer_begin(SimpleType::Binary);
    }

    fn binary_fragment(&mut self) {
        match self.current_mut() {
            Some(ContextType {
                ty: Type::Simple(SimpleType::Binary),
                state: _,
            }) => (),
            v => panic!("expected text, got {:?}", v),
        }
    }

    fn pop_binary(&mut self) {
        self.infer_end(SimpleType::Binary);
    }

    fn push_map(&mut self) {
        self.infer_begin(TypeBuilder::Map);
    }

    fn push_map_key(&mut self) {
        match self.current_mut() {
            Some(ContextType {
                state: state,
                ty: Type::Map { key, .. },
            }) => {
                assert_eq!(*state, State::Valid, "unexpected map key");
                *state = State::MapKey;

                let key = key.take().map(|ty| ContextType {
                    state: State::Valid,
                    ty,
                });

                self.push(key);
            }
            v => panic!("expected map, got {:?}", v),
        }
    }

    fn pop_map_key(&mut self) {
        let restore = self.pop().expect("missing key to restore");

        assert!(
            restore.is_valid(),
            "attempt to restore invalid key {:?}",
            restore
        );

        match self.current_mut() {
            Some(ContextType {
                state,
                ty: Type::Map { key, .. },
            }) => {
                assert_eq!(*state, State::MapKey, "unexpected map key");

                **key = Some(restore.ty);
            }
            v => panic!("expected map, got {:?}", v),
        }
    }

    fn push_map_value(&mut self) {
        match self.current_mut() {
            Some(ContextType {
                state,
                ty: Type::Map { value, .. },
            }) => {
                assert_eq!(*state, State::MapKey, "unexpected map value");
                *state = State::MapValue;

                let value = value.take().map(|ty| ContextType {
                    state: State::Valid,
                    ty,
                });

                self.push(value);
            }
            v => panic!("expected map, got {:?}", v),
        }
    }

    fn pop_map_value(&mut self) {
        let restore = self.pop().expect("missing value to restore");

        assert!(
            restore.is_valid(),
            "attempt to restore invalid value {:?}",
            restore
        );

        match self.current_mut() {
            Some(ContextType {
                state: state,
                ty: Type::Map { value, .. },
            }) => {
                assert_eq!(
                    *state,
                    State::MapValue,
                    "failed to restore {:?}: unexpected map value",
                    restore
                );
                *state = State::Valid;

                **value = Some(restore.ty);
            }
            v => panic!("failed to restore {:?}: expected map, got {:?}", restore, v),
        }
    }

    fn pop_map(&mut self) {
        self.infer_end(TypeBuilder::Map);
    }

    fn push_seq(&mut self) {
        self.infer_begin(TypeBuilder::Seq);
    }

    fn push_seq_value(&mut self) {
        match self.current_mut() {
            Some(ContextType {
                state: state,
                ty: Type::Seq { value, .. },
            }) => {
                assert_eq!(*state, State::Valid, "unexpected seq value");
                *state = State::SeqValue;

                let value = value.take().map(|ty| ContextType {
                    state: State::Valid,
                    ty,
                });

                self.push(value);
            }
            v => panic!("expected seq, got {:?}", v),
        }
    }

    fn pop_seq_value(&mut self) {
        let restore = self.pop().expect("missing value to restore");

        assert!(
            restore.is_valid(),
            "attempt to restore invalid value {:?}",
            restore
        );

        match self.current_mut() {
            Some(ContextType {
                state: state,
                ty: Type::Seq { value, .. },
            }) => {
                assert_eq!(
                    *state,
                    State::SeqValue,
                    "failed to restore {:?}: unexpected seq value",
                    restore
                );
                *state = State::Valid;

                **value = Some(restore.ty);
            }
            v => panic!("failed to restore {:?}: expected seq, got {:?}", restore, v),
        }
    }

    fn pop_seq(&mut self) {
        self.infer_end(TypeBuilder::Seq);
    }
}

#[derive(Debug, Clone, Copy)]
enum TypeBuilder<'a> {
    Simple(SimpleType),
    Map,
    Seq,
    Record(sval::Tag<'a>),
}

#[derive(Debug)]
struct ContextType {
    state: State,
    ty: Type,
}

#[derive(Debug, PartialEq)]
enum State {
    Valid,
    MapKey,
    MapValue,
    SeqValue,
}

impl ContextType {
    fn is_valid(&self) -> bool {
        matches!(self.state, State::Valid)
    }
}

impl<'a> From<SimpleType> for TypeBuilder<'a> {
    fn from(ty: SimpleType) -> Self {
        TypeBuilder::Simple(ty)
    }
}

impl<'a> TypeBuilder<'a> {
    fn build(self) -> Type {
        match self {
            TypeBuilder::Simple(ty) => Type::Simple(ty),
            TypeBuilder::Map => Type::Map {
                key: Box::new(None),
                value: Box::new(None),
            },
            TypeBuilder::Seq => Type::Seq {
                value: Box::new(None),
            },
            TypeBuilder::Record(tag) => Type::Record {
                id: tag.id().map(Into::into),
                label: tag.label().map(Into::into),
                values: Vec::new(),
            },
        }
    }

    fn is_compatible_with(&self, ty: &Type) -> bool {
        match (ty, self) {
            (Type::Map { .. }, TypeBuilder::Map) => true,
            (Type::Seq { .. }, TypeBuilder::Seq) => true,
            (Type::Simple(a), TypeBuilder::Simple(b)) => a == b,
            _ => false,
        }
    }

    fn is_chunked(&self) -> bool {
        match self {
            TypeBuilder::Map => true,
            TypeBuilder::Seq => true,
            TypeBuilder::Simple(SimpleType::Text) => true,
            TypeBuilder::Simple(SimpleType::Binary) => true,
            _ => false,
        }
    }
}

impl<'sval> sval::Stream<'sval> for Evaluator {
    fn unit(&mut self) -> sval::Result {
        self.infer_begin(SimpleType::Unit);

        Ok(())
    }

    fn null(&mut self) -> sval::Result {
        self.infer_begin(SimpleType::Null);

        Ok(())
    }

    fn bool(&mut self, _: bool) -> sval::Result {
        self.infer_begin(SimpleType::Bool);

        Ok(())
    }

    fn text_begin(&mut self, _: Option<usize>) -> sval::Result {
        self.push_text();

        Ok(())
    }

    fn text_fragment(&mut self, _: &'sval str) -> sval::Result {
        self.text_fragment();

        Ok(())
    }

    fn text_fragment_computed(&mut self, _: &str) -> sval::Result {
        self.text_fragment();

        Ok(())
    }

    fn text_end(&mut self) -> sval::Result {
        self.pop_text();

        Ok(())
    }

    fn binary_begin(&mut self, _: Option<usize>) -> sval::Result {
        self.push_binary();

        Ok(())
    }

    fn binary_fragment(&mut self, _: &'sval [u8]) -> sval::Result {
        self.binary_fragment();

        Ok(())
    }

    fn binary_fragment_computed(&mut self, _: &[u8]) -> sval::Result {
        self.binary_fragment();

        Ok(())
    }

    fn binary_end(&mut self) -> sval::Result {
        self.pop_binary();

        Ok(())
    }

    fn u8(&mut self, _: u8) -> sval::Result {
        self.infer_begin(SimpleType::U8);

        Ok(())
    }

    fn u16(&mut self, _: u16) -> sval::Result {
        self.infer_begin(SimpleType::U16);

        Ok(())
    }

    fn u32(&mut self, _: u32) -> sval::Result {
        self.infer_begin(SimpleType::U32);

        Ok(())
    }

    fn u64(&mut self, _: u64) -> sval::Result {
        self.infer_begin(SimpleType::U64);

        Ok(())
    }

    fn u128(&mut self, _: u128) -> sval::Result {
        self.infer_begin(SimpleType::U128);

        Ok(())
    }

    fn i8(&mut self, _: i8) -> sval::Result {
        self.infer_begin(SimpleType::I8);

        Ok(())
    }

    fn i16(&mut self, _: i16) -> sval::Result {
        self.infer_begin(SimpleType::I16);

        Ok(())
    }

    fn i32(&mut self, _: i32) -> sval::Result {
        self.infer_begin(SimpleType::I32);

        Ok(())
    }

    fn i64(&mut self, _: i64) -> sval::Result {
        self.infer_begin(SimpleType::I64);

        Ok(())
    }

    fn i128(&mut self, _: i128) -> sval::Result {
        self.infer_begin(SimpleType::I128);

        Ok(())
    }

    fn f32(&mut self, _: f32) -> sval::Result {
        self.infer_begin(SimpleType::F32);

        Ok(())
    }

    fn f64(&mut self, _: f64) -> sval::Result {
        self.infer_begin(SimpleType::F64);

        Ok(())
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

    fn seq_begin(&mut self, _: Option<usize>) -> sval::Result {
        self.push_seq();
        Ok(())
    }

    fn seq_value_begin(&mut self) -> sval::Result {
        self.push_seq_value();
        Ok(())
    }

    fn seq_value_end(&mut self) -> sval::Result {
        self.pop_seq_value();
        Ok(())
    }

    fn seq_end(&mut self) -> sval::Result {
        self.pop_seq();
        Ok(())
    }

    fn dynamic_begin(&mut self) -> sval::Result {
        todo!()
    }

    fn dynamic_end(&mut self) -> sval::Result {
        todo!()
    }

    fn enum_begin(&mut self, tag: sval::Tag) -> sval::Result {
        todo!()
    }

    fn enum_end(&mut self, tag: sval::Tag) -> sval::Result {
        todo!()
    }

    fn tagged_begin(&mut self, tag: sval::Tag) -> sval::Result {
        todo!()
    }

    fn tagged_end(&mut self, tag: sval::Tag) -> sval::Result {
        todo!()
    }

    fn record_begin(&mut self, tag: sval::Tag, num_entries: Option<usize>) -> sval::Result {
        todo!()
    }

    fn record_value_begin(&mut self, label: sval::Label) -> sval::Result {
        todo!()
    }

    fn record_value_end(&mut self, label: sval::Label) -> sval::Result {
        todo!()
    }

    fn record_end(&mut self, tag: sval::Tag) -> sval::Result {
        todo!()
    }

    fn tuple_begin(&mut self, tag: sval::Tag, num_entries_hint: Option<usize>) -> sval::Result {
        todo!()
    }

    fn tuple_value_begin(&mut self, index: u32) -> sval::Result {
        todo!()
    }

    fn tuple_value_end(&mut self, index: u32) -> sval::Result {
        todo!()
    }

    fn tuple_end(&mut self, tag: sval::Tag) -> sval::Result {
        todo!()
    }

    fn constant_begin(&mut self, tag: sval::Tag) -> sval::Result {
        todo!()
    }

    fn constant_end(&mut self, tag: sval::Tag) -> sval::Result {
        todo!()
    }

    fn constant_size_begin(&mut self) -> sval::Result {
        todo!()
    }

    fn constant_size_end(&mut self) -> sval::Result {
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
