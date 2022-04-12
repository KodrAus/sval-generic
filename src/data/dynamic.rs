use crate::data::Tag;
use crate::{data::Position, Receiver, Result, Resume, Source, Value};

pub fn dynamic<'src, T: Source<'src>>(value: T) -> Dynamic<T> {
    Dynamic::new(value)
}

pub struct Dynamic<T: ?Sized> {
    is_wrapped: bool,
    position: Position,
    value: T,
}

impl<T> Dynamic<T> {
    pub fn new(value: T) -> Self {
        Dynamic {
            value,
            is_wrapped: false,
            position: Position::Begin,
        }
    }

    pub fn into_inner(self) -> T {
        self.value
    }
}

impl<T: ?Sized> Dynamic<T> {
    pub fn by_ref(&self) -> Dynamic<&T> {
        Dynamic {
            is_wrapped: self.is_wrapped,
            position: self.position,
            value: &self.value,
        }
    }

    pub fn by_mut(&mut self) -> Dynamic<&mut T> {
        Dynamic {
            is_wrapped: self.is_wrapped,
            position: self.position,
            value: &mut self.value,
        }
    }
}

impl<'src, T: Source<'src>> Source<'src> for Dynamic<T> {
    fn stream_resume<'data, R: Receiver<'data>>(&mut self, mut receiver: R) -> Result<Resume>
    where
        'src: 'data,
    {
        loop {
            match self.position {
                Position::Begin => {
                    let mut wrap = MaybeWrap {
                        inner: &mut receiver,
                        already_dynamic: false,
                        is_wrapped: false,
                    };

                    // This first call will detect whether or not the inner value
                    // is already dynamic. If it isn't, it'll call `dynamic_begin`.
                    let r = self.value.stream_resume(&mut wrap)?;
                    self.is_wrapped = wrap.is_wrapped;

                    match r {
                        Resume::Continue => {
                            self.position = Position::Value;
                            return Ok(Resume::Continue);
                        }
                        Resume::Done => {
                            self.position = Position::End;
                        }
                    }
                }
                Position::Value => match self.value.stream_resume(&mut receiver)? {
                    Resume::Continue => return Ok(Resume::Continue),
                    Resume::Done => self.position = Position::End,
                },
                Position::End => {
                    // If the inner value was wrapped as dynamic then close it
                    // If it wasn't wrapped then the last call to `value.stream_resume` will
                    // have closed it
                    if self.is_wrapped {
                        receiver.dynamic_end()?;
                    }

                    self.position = Position::Done;
                }
                Position::Done => return Ok(Resume::Done),
            }
        }
    }

    fn maybe_dynamic(&self) -> Option<bool> {
        Some(true)
    }
}

struct MaybeWrap<R> {
    inner: R,
    already_dynamic: bool,
    is_wrapped: bool,
}

impl<'data, R: Receiver<'data>> MaybeWrap<R> {
    fn maybe_already_dynamic(&mut self) {
        if !self.already_dynamic && !self.is_wrapped {
            self.already_dynamic = true;
        }
    }

    fn maybe_dynamic_begin(&mut self) -> Result {
        if !self.already_dynamic && !self.is_wrapped {
            self.is_wrapped = true;
            self.inner.dynamic_begin()
        } else {
            Ok(())
        }
    }
}

impl<'data, R: Receiver<'data>> Receiver<'data> for MaybeWrap<R> {
    fn is_text_based(&self) -> bool {
        self.inner.is_text_based()
    }

    fn value<V: Value + ?Sized + 'data>(&mut self, value: &'data V) -> Result {
        self.maybe_dynamic_begin()?;
        self.inner.value(value)
    }

    fn unit(&mut self) -> Result {
        self.maybe_dynamic_begin()?;
        self.inner.unit()
    }

    fn null(&mut self) -> Result {
        self.maybe_dynamic_begin()?;
        self.inner.null()
    }

    fn bool(&mut self, value: bool) -> Result {
        self.maybe_dynamic_begin()?;
        self.inner.bool(value)
    }

    fn text_begin(&mut self, num_bytes_hint: Option<usize>) -> Result {
        self.maybe_dynamic_begin()?;
        self.inner.text_begin(num_bytes_hint)
    }

    fn text_fragment(&mut self, fragment: &'data str) -> Result {
        self.inner.text_fragment(fragment)
    }

    fn text_fragment_computed(&mut self, fragment: &str) -> Result {
        self.inner.text_fragment_computed(fragment)
    }

    fn text_end(&mut self) -> Result {
        self.inner.text_end()
    }

    fn text(&mut self, value: &'data str) -> Result {
        self.maybe_dynamic_begin()?;
        self.inner.text(value)
    }

    fn binary_begin(&mut self, num_bytes_hint: Option<usize>) -> Result {
        self.maybe_dynamic_begin()?;
        self.inner.binary_begin(num_bytes_hint)
    }

    fn binary_fragment(&mut self, fragment: &'data [u8]) -> Result {
        self.inner.binary_fragment(fragment)
    }

    fn binary_fragment_computed(&mut self, fragment: &[u8]) -> Result {
        self.inner.binary_fragment_computed(fragment)
    }

    fn binary_end(&mut self) -> Result {
        self.inner.binary_end()
    }

    fn binary(&mut self, value: &'data [u8]) -> Result {
        self.maybe_dynamic_begin()?;
        self.inner.binary(value)
    }

    fn u8(&mut self, value: u8) -> Result {
        self.maybe_dynamic_begin()?;
        self.inner.u8(value)
    }

    fn u16(&mut self, value: u16) -> Result {
        self.maybe_dynamic_begin()?;
        self.inner.u16(value)
    }

    fn u32(&mut self, value: u32) -> Result {
        self.maybe_dynamic_begin()?;
        self.inner.u32(value)
    }

    fn u64(&mut self, value: u64) -> Result {
        self.maybe_dynamic_begin()?;
        self.inner.u64(value)
    }

    fn u128(&mut self, value: u128) -> Result {
        self.maybe_dynamic_begin()?;
        self.inner.u128(value)
    }

    fn i8(&mut self, value: i8) -> Result {
        self.maybe_dynamic_begin()?;
        self.inner.i8(value)
    }

    fn i16(&mut self, value: i16) -> Result {
        self.maybe_dynamic_begin()?;
        self.inner.i16(value)
    }

    fn i32(&mut self, value: i32) -> Result {
        self.maybe_dynamic_begin()?;
        self.inner.i32(value)
    }

    fn i64(&mut self, value: i64) -> Result {
        self.maybe_dynamic_begin()?;
        self.inner.i64(value)
    }

    fn i128(&mut self, value: i128) -> Result {
        self.maybe_dynamic_begin()?;
        self.inner.i128(value)
    }

    fn f32(&mut self, value: f32) -> Result {
        self.maybe_dynamic_begin()?;
        self.inner.f32(value)
    }

    fn f64(&mut self, value: f64) -> Result {
        self.maybe_dynamic_begin()?;
        self.inner.f64(value)
    }

    fn map_begin(&mut self, num_entries_hint: Option<usize>) -> Result {
        self.maybe_dynamic_begin()?;
        self.inner.map_begin(num_entries_hint)
    }

    fn map_key_begin(&mut self) -> Result {
        self.inner.map_key_begin()
    }

    fn map_key_end(&mut self) -> Result {
        self.inner.map_key_end()
    }

    fn map_value_begin(&mut self) -> Result {
        self.inner.map_value_begin()
    }

    fn map_value_end(&mut self) -> Result {
        self.inner.map_value_end()
    }

    fn map_end(&mut self) -> Result {
        self.inner.map_end()
    }

    fn map_key<'k: 'data, K: Source<'k>>(&mut self, key: K) -> Result {
        self.inner.map_key(key)
    }

    fn map_value<'v: 'data, V: Source<'v>>(&mut self, value: V) -> Result {
        self.inner.map_value(value)
    }

    fn seq_begin(&mut self, num_entries_hint: Option<usize>) -> Result {
        self.maybe_dynamic_begin()?;
        self.inner.seq_begin(num_entries_hint)
    }

    fn seq_value_begin(&mut self) -> Result {
        self.inner.seq_value_begin()
    }

    fn seq_value_end(&mut self) -> Result {
        self.inner.seq_value_end()
    }

    fn seq_end(&mut self) -> Result {
        self.inner.seq_end()
    }

    fn seq_value<'e: 'data, V: Source<'e>>(&mut self, value: V) -> Result {
        self.inner.seq_value(value)
    }

    fn dynamic_begin(&mut self) -> Result {
        self.maybe_already_dynamic();
        self.inner.dynamic_begin()
    }

    fn dynamic_end(&mut self) -> Result {
        self.inner.dynamic_end()
    }

    fn fixed_size_begin(&mut self) -> Result {
        self.maybe_dynamic_begin()?;
        self.inner.fixed_size_begin()
    }

    fn fixed_size_end(&mut self) -> Result {
        self.inner.fixed_size_end()
    }

    fn tagged_begin(&mut self, tag: Tag) -> Result {
        self.maybe_dynamic_begin()?;
        self.inner.tagged_begin(tag)
    }

    fn tagged_end(&mut self) -> Result {
        self.inner.tagged_end()
    }

    fn constant_begin(&mut self, tag: Tag) -> Result {
        self.maybe_dynamic_begin()?;
        self.inner.constant_begin(tag)
    }

    fn constant_end(&mut self) -> Result {
        self.inner.constant_end()
    }

    fn struct_map_begin(&mut self, tag: Tag, num_entries_hint: Option<usize>) -> Result {
        self.maybe_dynamic_begin()?;
        self.inner.struct_map_begin(tag, num_entries_hint)
    }

    fn struct_map_key_begin(&mut self, tag: Tag) -> Result {
        self.inner.struct_map_key_begin(tag)
    }

    fn struct_map_key_end(&mut self) -> Result {
        self.inner.struct_map_key_end()
    }

    fn struct_map_value_begin(&mut self, tag: Tag) -> Result {
        self.inner.struct_map_value_begin(tag)
    }

    fn struct_map_value_end(&mut self) -> Result {
        self.inner.struct_map_value_end()
    }

    fn struct_map_end(&mut self) -> Result {
        self.inner.struct_map_end()
    }

    fn struct_map_key<'k: 'data, K: Source<'k>>(&mut self, tag: Tag, key: K) -> Result {
        self.inner.struct_map_key(tag, key)
    }

    fn struct_map_value<'v: 'data, V: Source<'v>>(&mut self, tag: Tag, value: V) -> Result {
        self.inner.struct_map_value(tag, value)
    }

    fn struct_seq_begin(&mut self, tag: Tag, num_entries_hint: Option<usize>) -> Result {
        self.maybe_dynamic_begin()?;
        self.inner.struct_seq_begin(tag, num_entries_hint)
    }

    fn struct_seq_value_begin(&mut self, tag: Tag) -> Result {
        self.inner.struct_seq_value_begin(tag)
    }

    fn struct_seq_value_end(&mut self) -> Result {
        self.inner.struct_seq_value_end()
    }

    fn struct_seq_end(&mut self) -> Result {
        self.inner.struct_seq_end()
    }

    fn struct_seq_value<'v: 'data, V: Source<'v>>(&mut self, tag: Tag, value: V) -> Result {
        self.inner.struct_seq_value(tag, value)
    }

    fn enum_begin(&mut self, tag: Tag) -> Result {
        self.maybe_dynamic_begin()?;
        self.inner.enum_begin(tag)
    }

    fn enum_end(&mut self) -> Result {
        self.inner.enum_end()
    }

    fn nullable_begin(&mut self) -> Result {
        self.maybe_dynamic_begin()?;
        self.inner.nullable_begin()
    }

    fn nullable_end(&mut self) -> Result {
        self.inner.nullable_end()
    }

    fn int_begin(&mut self) -> Result {
        self.maybe_dynamic_begin()?;
        self.inner.int_begin()
    }

    fn int_end(&mut self) -> Result {
        self.inner.int_end()
    }

    fn binfloat_begin(&mut self) -> Result {
        self.maybe_dynamic_begin()?;
        self.inner.binfloat_begin()
    }

    fn binfloat_end(&mut self) -> Result {
        self.inner.binfloat_end()
    }

    fn decfloat_begin(&mut self) -> Result {
        self.maybe_dynamic_begin()?;
        self.inner.decfloat_begin()
    }

    fn decfloat_end(&mut self) -> Result {
        self.inner.decfloat_end()
    }
}
