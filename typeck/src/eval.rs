use std::collections::HashMap;

use crate::{Id, SimpleType, Type};

/**
Infer and check the types of values.

The evaluator works on the theory that it should only ever see correctly typed data. If it encounters
invalid data then it doesn't attempt to restore itself to a working state, it simply blows up with
the error, taking its state with it.
*/
#[derive(Debug)]
pub struct Evaluator {
    root: Option<StackType>,
    globals: HashMap<Id, Type>,
    stack: Vec<Option<StackType>>,
}

impl Evaluator {
    /**
    Create an empty evaluator that hasn't seen any data yet.
    */
    pub fn empty() -> Self {
        Evaluator {
            root: None,
            globals: HashMap::new(),
            stack: Vec::new(),
        }
    }

    /**
    Evaluate the type of an individual value.
    */
    pub fn type_of_val(v: impl sval::Value) -> Type {
        let evaluator = Self::empty().eval(v).expect("failed to evaluate");

        evaluator
            .root
            .map(|result| result.ty)
            .expect("value didn't produce a type")
    }

    /**
    Extend the type known to the evaluator using the given value.

    If the value isn't compatible with whatever type the evaluator has already seen then it will fail.
    */
    pub fn eval(mut self, v: impl sval::Value) -> Result<Self, sval::Error> {
        v.stream(&mut self)?;
        assert!(self.stack.is_empty(), "unexpected end of input");

        Ok(self)
    }

    /**
    Check whether the type of a value is compatible with what's been evaluated.

    If the evaluator hasn't seen a type yet this method will panic.
    */
    pub fn check(&self, v: impl sval::Value) -> bool {
        let typeof_self = self
            .get_type()
            .expect("attempt to check the type of an empty context");
        let typeof_v = Evaluator::type_of_val(v);

        typeof_self == &typeof_v
    }

    /**
    Get the type as it's currently known to the evaluator.
    */
    pub fn get_type(&self) -> Option<&Type> {
        self.root.as_ref().map(|result| &result.ty)
    }

    /**
    Clear all state from the evaluator so it can be re-used.
    */
    pub fn clear(&mut self) {
        let Evaluator {
            root,
            globals,
            stack,
        } = self;

        *root = None;
        globals.clear();
        stack.clear();
    }

    fn current_mut(&mut self) -> &mut Option<StackType> {
        if let Some(current) = self.stack.last_mut() {
            current
        } else {
            &mut self.root
        }
    }

    fn push(&mut self, ty: Option<StackType>) {
        self.stack.push(ty);
    }

    fn pop(&mut self) -> Option<StackType> {
        self.stack
            .pop()
            .and_then(|v| v)
            .or_else(|| self.root.take())
    }

    fn infer_begin<'a>(&mut self, builder: impl Into<TypeBuilder<'a>>) {
        let builder = builder.into();
        let is_chunked = builder.is_chunked();

        /*
        Ids follow a particular rule:

        1. Ids in the same Scope must always match the same Structural Type.

        The Scope of an Id can be either:

        1. The set of Variants in a particular Enum.
        2. The set of all Ids that can appear in the Context, including any Enums.

        Ids that appear on Values immediately following an Enum are in that Enum's Scope.
        Ids that appear anywhere else are in the Context's Scope.

        We know we're looking at a particular Enum because either:

        1. It has the same Path from the root of the Context.
        2. It has the same Id.

        The Variants that can appear in an Enum are distinguished by their Type.
        Given the rules for Ids, that means Variants may be either purely Structural (without an Id)
        or Identified (with an Id, which must always match that same structural type).
        */

        match self.current_mut() {
            empty @ None => {
                *empty = Some(StackType {
                    state: State::Valid,
                    index: 0,
                    scope: builder.scope(),
                    ty: builder.build(),
                });
            }
            inferred @ Some(StackType {
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

        if is_chunked {
            let ty = self.pop();
            self.push(ty);
        }
    }

    fn infer_end<'a>(&mut self, builder: impl Into<TypeBuilder<'a>>) {
        let builder = builder.into();
        let depth = self.stack.len();

        // TODO: Check the Scope of our context: if there's an id then ensure it also matches

        match self.current_mut() {
            Some(StackType { ty, .. }) if builder.is_chunked() && depth > 1 => {
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
            Some(StackType {
                ty: Type::Simple(SimpleType::Text),
                scope: _,
                state: _,
                index: _,
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
            Some(StackType {
                ty: Type::Simple(SimpleType::Binary),
                scope: _,
                state: _,
                index: _,
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
            Some(StackType {
                state,
                scope,
                index: _,
                ty: Type::Map { key, .. },
            }) => {
                assert_eq!(*state, State::Valid, "unexpected map key");
                *state = State::MapKey;

                let key = key.take().map(|ty| StackType {
                    state: State::Valid,
                    scope: *scope,
                    index: 0,
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
            Some(StackType {
                state,
                scope: _,
                index: _,
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
            Some(StackType {
                state,
                scope,
                index: _,
                ty: Type::Map { value, .. },
            }) => {
                assert_eq!(*state, State::MapKey, "unexpected map value");
                *state = State::MapValue;

                let value = value.take().map(|ty| StackType {
                    state: State::Valid,
                    scope: *scope,
                    index: 0,
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
            Some(StackType {
                state,
                scope: _,
                index,
                ty: Type::Map { value, .. },
            }) => {
                assert_eq!(
                    *state,
                    State::MapValue,
                    "failed to restore {:?}: unexpected map value",
                    restore
                );
                *state = State::Valid;
                *index += 1;

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
            Some(StackType {
                state,
                scope,
                index: _,
                ty: Type::Seq { value, .. },
            }) => {
                assert_eq!(*state, State::Valid, "unexpected seq value");
                *state = State::SeqValue;

                let value = value.take().map(|ty| StackType {
                    state: State::Valid,
                    scope: *scope,
                    index: 0,
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
            Some(StackType {
                state,
                scope: _,
                index,
                ty: Type::Seq { value, .. },
            }) => {
                assert_eq!(
                    *state,
                    State::SeqValue,
                    "failed to restore {:?}: unexpected seq value",
                    restore
                );
                *state = State::Valid;
                *index += 1;

                **value = Some(restore.ty);
            }
            v => panic!("failed to restore {:?}: expected seq, got {:?}", restore, v),
        }
    }

    fn pop_seq(&mut self) {
        self.infer_end(TypeBuilder::Seq);
    }

    fn push_record(&mut self, tag: sval::Tag) {
        self.infer_begin(TypeBuilder::Record(tag));
    }

    fn push_record_value(&mut self, label: sval::Label) {
        match self.current_mut() {
            Some(StackType {
                state,
                scope,
                index,
                ty: Type::Record { values, .. },
            }) => {
                assert_eq!(*state, State::Valid, "unexpected record value");
                *state = State::RecordValue;

                let value = match values.get_mut(*index) {
                    Some((existing_label, value)) => {
                        assert_eq!(&*existing_label, &label.into(), "values out-of-order");

                        value.take().map(|ty| StackType {
                            state: State::Valid,
                            scope: *scope,
                            index: 0,
                            ty,
                        })
                    }
                    None => {
                        assert_eq!(*index, values.len(), "attempt to push values out-of-order");
                        values.push((label.into(), None));

                        None
                    }
                };

                self.push(value);
            }
            v => panic!("expected record, got {:?}", v),
        }
    }

    fn pop_record_value(&mut self, label: sval::Label) {
        let restore = self.pop().expect("missing value to restore");

        assert!(
            restore.is_valid(),
            "attempt to restore invalid value {:?}",
            restore
        );

        match self.current_mut() {
            Some(StackType {
                state,
                scope: _,
                index,
                ty: Type::Record { values, .. },
            }) => {
                assert_eq!(
                    *state,
                    State::RecordValue,
                    "failed to restore {:?}: unexpected record value",
                    restore
                );

                assert_eq!(&values[*index].0, &label.into(), "values out-of-order");
                values[*index].1 = Some(restore.ty);

                *state = State::Valid;
                *index += 1;
            }
            v => panic!("failed to restore {:?}: expected map, got {:?}", restore, v),
        }
    }

    fn pop_record(&mut self, tag: sval::Tag) {
        self.infer_end(TypeBuilder::Record(tag));
    }
}

#[derive(Debug, Clone)]
enum TypeBuilder<'a> {
    Simple(SimpleType),
    Map,
    Seq,
    Record(sval::Tag<'a>),
}

#[derive(Debug)]
struct StackType {
    state: State,
    scope: Scope,
    index: usize,
    ty: Type,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Scope {
    Local,
    Global,
}

#[derive(Debug, PartialEq)]
enum State {
    Valid,
    MapKey,
    MapValue,
    SeqValue,
    RecordValue,
}

impl StackType {
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
            TypeBuilder::Record(tag) => {
                let (id, label) = tag.split();

                Type::Record {
                    id: id.map(Into::into),
                    label: label.map(Into::into),
                    values: Vec::new(),
                }
            }
        }
    }

    fn is_compatible_with(&self, ty: &Type) -> bool {
        match (ty, self) {
            (Type::Map { .. }, TypeBuilder::Map) => true,
            (Type::Seq { .. }, TypeBuilder::Seq) => true,
            (Type::Simple(a), TypeBuilder::Simple(b)) => a == b,
            (Type::Record { id, .. }, TypeBuilder::Record(tag)) => tag.id() == id.as_ref(),
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

    fn scope(&self) -> Scope {
        Scope::Global
    }

    fn id(&self) -> Option<Id> {
        match self {
            TypeBuilder::Record(sval::Tag::Identified(id, _)) => Some(*id),
            _ => None,
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
        self.push_record(tag);
        Ok(())
    }

    fn record_value_begin(&mut self, label: sval::Label) -> sval::Result {
        self.push_record_value(label);
        Ok(())
    }

    fn record_value_end(&mut self, label: sval::Label) -> sval::Result {
        self.pop_record_value(label);
        Ok(())
    }

    fn record_end(&mut self, tag: sval::Tag) -> sval::Result {
        self.pop_record(tag);
        Ok(())
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
