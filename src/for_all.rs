use std::{error, fmt};

use crate::{
    source::{Source, ToValueError, TypedSource},
    stream::Stream,
    tag::{TypeTag, VariantTag},
    value::Value,
    Error, Result,
};

#[derive(Clone, Copy)]
pub struct ForAll<T>(pub(crate) T);

impl<T: Value> Value for ForAll<T> {
    fn stream<'a, S: Stream<'a>>(&'a self, stream: S) -> Result {
        self.0.stream(stream)
    }
}

impl<'a, 'b, T: Source<'b>> Source<'a> for ForAll<T> {
    fn stream<'c, S: Stream<'c>>(&mut self, stream: S) -> Result
    where
        'a: 'c,
    {
        self.0.stream(ForAll(stream))
    }
}

impl<'a, 'b, U: Value + ?Sized + 'static, T: TypedSource<'b, U>> TypedSource<'a, U> for ForAll<T> {
    // NOTE: We can't use `T::Error` here or `'b` becomes unconstrained
    type Error = Error;

    fn stream_to_value(&mut self) -> Result<&U, ToValueError<Self::Error>> {
        self.0
            .stream_to_value()
            .map_err(|e| ToValueError::from_error(e.into_inner().into()))
    }
}

impl<'a, 'b, S: Stream<'b>> Stream<'a> for ForAll<S> {
    fn any<'v: 'a, V: Source<'v>>(&mut self, value: V) -> Result {
        self.0.any(ForAll(value))
    }

    fn display<D: fmt::Display>(&mut self, fmt: D) -> Result {
        self.0.display(fmt)
    }

    fn u64(&mut self, value: u64) -> Result {
        self.0.u64(value)
    }

    fn i64(&mut self, value: i64) -> Result {
        self.0.i64(value)
    }

    fn u128(&mut self, value: u128) -> Result {
        self.0.u128(value)
    }

    fn i128(&mut self, value: i128) -> Result {
        self.0.i128(value)
    }

    fn f64(&mut self, value: f64) -> Result {
        self.0.f64(value)
    }

    fn bool(&mut self, value: bool) -> Result {
        self.0.bool(value)
    }

    fn none(&mut self) -> Result {
        self.0.none()
    }

    fn str<'s: 'a, T: TypedSource<'s, str>>(&mut self, value: T) -> Result {
        self.0.str(ForAll(value))
    }

    fn error<'e: 'a, E: TypedSource<'e, dyn error::Error + 'static>>(
        &mut self,
        error: E,
    ) -> Result {
        self.0.error(ForAll(error))
    }

    fn type_tag<T: TypedSource<'static, str>>(&mut self, tag: TypeTag<T>) -> Result {
        self.0.type_tag(tag)
    }

    fn variant_tag<T: TypedSource<'static, str>, K: TypedSource<'static, str>>(
        &mut self,
        tag: VariantTag<T, K>,
    ) -> Result {
        self.0.variant_tag(tag)
    }

    fn type_tagged_begin<T: TypedSource<'static, str>>(&mut self, tag: TypeTag<T>) -> Result {
        self.0.type_tagged_begin(tag)
    }

    fn type_tagged_end(&mut self) -> Result {
        self.0.type_tagged_end()
    }

    fn variant_tagged_begin<T: TypedSource<'static, str>, K: TypedSource<'static, str>>(
        &mut self,
        tag: VariantTag<T, K>,
    ) -> Result {
        self.0.variant_tagged_begin(tag)
    }

    fn variant_tagged_end(&mut self) -> Result {
        self.0.variant_tagged_end()
    }

    fn type_tagged<'v: 'a, T: TypedSource<'static, str>, V: Source<'v>>(
        &mut self,
        tag: TypeTag<T>,
        value: V,
    ) -> Result {
        self.0.type_tagged(tag, ForAll(value))
    }

    fn variant_tagged<
        'v: 'a,
        T: TypedSource<'static, str>,
        K: TypedSource<'static, str>,
        V: Source<'v>,
    >(
        &mut self,
        tag: VariantTag<T, K>,
        value: V,
    ) -> Result {
        self.0.variant_tagged(tag, ForAll(value))
    }

    fn map_begin(&mut self, len: Option<usize>) -> Result {
        self.0.map_begin(len)
    }

    fn map_end(&mut self) -> Result {
        self.0.map_end()
    }

    fn map_key_begin(&mut self) -> Result {
        self.0.map_key_begin()
    }

    fn map_key_end(&mut self) -> Result {
        self.0.map_key_end()
    }

    fn map_value_begin(&mut self) -> Result {
        self.0.map_value_begin()
    }

    fn map_value_end(&mut self) -> Result {
        self.0.map_value_end()
    }

    fn type_tagged_map_begin<T: TypedSource<'static, str>>(
        &mut self,
        tag: TypeTag<T>,
        len: Option<usize>,
    ) -> Result {
        self.0.type_tagged_map_begin(tag, len)
    }

    fn type_tagged_map_end(&mut self) -> Result {
        self.0.type_tagged_map_end()
    }

    fn variant_tagged_map_begin<T: TypedSource<'static, str>, K: TypedSource<'static, str>>(
        &mut self,
        tag: VariantTag<T, K>,
        len: Option<usize>,
    ) -> Result {
        self.0.variant_tagged_map_begin(tag, len)
    }

    fn variant_tagged_map_end(&mut self) -> Result {
        self.0.variant_tagged_map_end()
    }

    fn map_entry<'k: 'a, 'v: 'a, K: Source<'k>, V: Source<'v>>(
        &mut self,
        key: K,
        value: V,
    ) -> Result {
        self.0.map_entry(ForAll(key), ForAll(value))
    }

    fn map_key<'k: 'a, K: Source<'k>>(&mut self, key: K) -> Result {
        self.0.map_key(ForAll(key))
    }

    fn map_field<F: TypedSource<'static, str>>(&mut self, field: F) -> Result {
        self.0.map_field(field)
    }

    fn map_value<'v: 'a, V: Source<'v>>(&mut self, value: V) -> Result {
        self.0.map_value(ForAll(value))
    }

    fn seq_begin(&mut self, len: Option<usize>) -> Result {
        self.0.seq_begin(len)
    }

    fn seq_end(&mut self) -> Result {
        self.0.seq_end()
    }

    fn type_tagged_seq_begin<T: TypedSource<'static, str>>(
        &mut self,
        tag: TypeTag<T>,
        len: Option<usize>,
    ) -> Result {
        self.0.type_tagged_seq_begin(tag, len)
    }

    fn type_tagged_seq_end(&mut self) -> Result {
        self.0.type_tagged_seq_end()
    }

    fn variant_tagged_seq_begin<T: TypedSource<'static, str>, K: TypedSource<'static, str>>(
        &mut self,
        tag: VariantTag<T, K>,
        len: Option<usize>,
    ) -> Result {
        self.0.variant_tagged_seq_begin(tag, len)
    }

    fn variant_tagged_seq_end(&mut self) -> Result {
        self.0.variant_tagged_seq_end()
    }

    fn seq_elem_begin(&mut self) -> Result {
        self.0.seq_elem_begin()
    }

    fn seq_elem_end(&mut self) -> Result {
        self.0.seq_elem_end()
    }

    fn seq_elem<'e: 'a, E: Source<'e>>(&mut self, elem: E) -> Result {
        self.0.seq_elem(ForAll(elem))
    }
}
