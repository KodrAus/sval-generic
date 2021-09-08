use crate::{
    stream::{Ref, Stream, UnknownRef},
    value::Value,
    Result,
};

#[derive(Clone, Copy)]
pub struct ForAll<T>(pub(crate) T);

impl<T> Value for ForAll<T>
where
    T: Value,
{
    fn stream<'a, S>(&'a self, stream: S) -> Result
    where
        S: Stream<'a>,
    {
        self.0.stream(stream)
    }
}

impl<'a, 'b, T> UnknownRef<'a> for ForAll<T>
where
    T: UnknownRef<'b>,
{
    fn stream<'c, S>(self, stream: S) -> Result
    where
        'a: 'c,
        S: Stream<'c>,
    {
        self.0.stream(ForAll(stream))
    }
}

impl<'a, 'b, T, U: ?Sized> Ref<'a, U> for ForAll<T>
where
    T: Ref<'b, U>,
    U: Value,
{
    fn get(&self) -> &U {
        self.0.get()
    }

    fn try_unwrap(self) -> Option<&'a U> {
        None
    }
}

impl<'a, 'b, S> Stream<'a> for ForAll<S>
where
    S: Stream<'b>,
{
    fn u64(&mut self, v: u64) -> Result {
        self.0.u64(v)
    }

    fn i64(&mut self, v: i64) -> Result {
        self.0.i64(v)
    }

    fn u128(&mut self, v: u128) -> Result {
        self.0.u128(v)
    }

    fn i128(&mut self, v: i128) -> Result {
        self.0.i128(v)
    }

    fn f64(&mut self, v: f64) -> Result {
        self.0.f64(v)
    }

    fn bool(&mut self, v: bool) -> Result {
        self.0.bool(v)
    }

    fn none(&mut self) -> Result {
        self.0.none()
    }

    fn str<'v, V: Ref<'v, str>>(&mut self, v: V) -> Result
    where
        'v: 'a,
    {
        self.0.str(ForAll(v))
    }

    fn map_begin(&mut self, len: Option<usize>) -> Result {
        self.0.map_begin(len)
    }

    fn map_key_begin(&mut self) -> Result {
        self.0.map_key_begin()
    }

    fn map_value_begin(&mut self) -> Result {
        self.0.map_value_begin()
    }

    fn map_end(&mut self) -> Result {
        self.0.map_end()
    }

    fn map_key<'k, K: UnknownRef<'k>>(&mut self, k: K) -> Result
    where
        'k: 'a,
    {
        self.0.map_key(ForAll(k))
    }

    fn map_value<'v, V: UnknownRef<'v>>(&mut self, v: V) -> Result
    where
        'v: 'a,
    {
        self.0.map_value(ForAll(v))
    }

    fn map_entry<'k, 'v, K: UnknownRef<'k>, V: UnknownRef<'v>>(&mut self, k: K, v: V) -> Result
    where
        'k: 'a,
        'v: 'a,
    {
        self.0.map_entry(ForAll(k), ForAll(v))
    }

    fn map_field<'v, F: Ref<'static, str>, V: UnknownRef<'v>>(&mut self, f: F, v: V) -> Result
    where
        'v: 'a,
    {
        self.0.map_field(f, ForAll(v))
    }

    fn seq_begin(&mut self, len: Option<usize>) -> Result {
        self.0.seq_begin(len)
    }

    fn seq_elem_begin(&mut self) -> Result {
        self.0.seq_elem_begin()
    }

    fn seq_end(&mut self) -> Result {
        self.0.seq_end()
    }

    fn seq_elem<'e, E: UnknownRef<'e>>(&mut self, e: E) -> Result
    where
        'e: 'a,
    {
        self.0.seq_elem(ForAll(e))
    }
}
