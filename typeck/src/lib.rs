use sval::{Id, Label};

#[derive(Debug, PartialEq)]
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

    pub fn is_complete(&self) -> bool {
        match self {
            Type::Map { key, value } if key.is_some() && value.is_some() => true,
            Type::Simple(_) => true,
            _ => false,
        }
    }
}

#[derive(Debug)]
pub struct Context {
    root_type: Option<ContextType>,
    eval_stack: Vec<Option<ContextType>>,
}

#[derive(Debug)]
struct ContextType {
    state: State,
    ty: Type,
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
            root_type: None,
            eval_stack: Vec::new(),
        }
    }

    pub fn eval(&mut self, v: impl sval::Value) -> &Type {
        v.stream(self).expect("failed to stream");
        assert!(self.eval_stack.is_empty(), "unexpected end of input");

        self.root_type().expect("value didn't produce a type")
    }

    pub fn root_type(&self) -> Option<&Type> {
        self.root_type.as_ref().map(|result| &result.ty)
    }

    pub fn clear(&mut self) {
        self.root_type = None;
        self.eval_stack.clear();
    }

    fn current_mut(&mut self) -> &mut Option<ContextType> {
        if let Some(current) = self.eval_stack.last_mut() {
            current
        } else {
            &mut self.root_type
        }
    }

    fn push(&mut self, ty: Option<ContextType>) {
        self.eval_stack.push(ty);
    }

    fn pop(&mut self) -> Option<ContextType> {
        self.eval_stack
            .pop()
            .and_then(|v| v)
            .or_else(|| self.root_type.take())
    }

    fn infer_begin(&mut self, builder: impl Into<TypeBuilder>) {
        let builder = builder.into();

        match self.current_mut() {
            empty @ None => {
                *empty = Some(ContextType {
                    state: State::Ready,
                    ty: builder.build(),
                });
            }
            inferred @ Some(ContextType {
                state: State::Ready,
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

    fn infer_end(&mut self, builder: impl Into<TypeBuilder>) {
        let builder = builder.into();
        let depth = self.eval_stack.len();

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
                let ty = self.pop().expect("unexpected end of value").unwrap();

                assert!(
                    builder.is_compatible_with(&ty),
                    "expected {:?}, got {:?}",
                    ty,
                    builder,
                );

                match self.current_mut() {
                    empty @ None => {
                        *empty = Some(ContextType {
                            state: State::Ready,
                            ty,
                        });
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
                ..
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
                ..
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
                state,
                ty: Type::Map { key, .. },
            }) => {
                assert_eq!(*state, State::Ready, "unexpected map key");
                *state = State::MapKey;

                let key = key.take().map(|ty| ContextType {
                    state: State::Ready,
                    ty,
                });

                self.push(key);
            }
            v => panic!("expected map, got {:?}", v),
        }
    }

    fn pop_map_key(&mut self) {
        let restore = self.pop().map(ContextType::unwrap);

        match self.current_mut() {
            Some(ContextType {
                state,
                ty: Type::Map { key, .. },
            }) => {
                assert_eq!(*state, State::MapKey, "unexpected map key");

                **key = restore;
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
                    state: State::Ready,
                    ty,
                });

                self.push(value);
            }
            v => panic!("expected map, got {:?}", v),
        }
    }

    fn pop_map_value(&mut self) {
        let restore = self.pop().map(ContextType::unwrap);

        match self.current_mut() {
            Some(ContextType {
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

                **value = restore;
            }
            v => panic!("failed to restore {:?}: expected map, got {:?}", restore, v),
        }
    }

    fn pop_map(&mut self) {
        self.infer_end(TypeBuilder::Map);
    }
}

impl ContextType {
    fn unwrap(self) -> Type {
        match self {
            ContextType {
                state: State::Ready,
                ty,
            } => ty,
            invalid => panic!("cannot unwrap {:?}", invalid),
        }
    }
}

#[derive(Debug, Clone, Copy)]
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

    fn is_compatible_with(&self, ty: &Type) -> bool {
        match (ty, self) {
            (Type::Map { .. }, TypeBuilder::Map) => true,
            (Type::Simple(a), TypeBuilder::Simple(b)) => a == b,
            _ => false,
        }
    }

    fn is_chunked(&self) -> bool {
        match self {
            TypeBuilder::Map => true,
            TypeBuilder::Simple(SimpleType::Text) => true,
            TypeBuilder::Simple(SimpleType::Binary) => true,
            _ => false,
        }
    }
}

impl<'sval> sval::Stream<'sval> for Context {
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

#[cfg(test)]
mod tests;
