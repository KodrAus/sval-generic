mod private {
    use crate::{Source, ValueSource};

    pub trait DispatchReceiver<'a> {
        fn dispatch_unstructured(&mut self, fmt: &dyn sval::data::Display) -> sval::Result;

        fn dispatch_null(&mut self) -> sval::Result;

        fn dispatch_u8(&mut self, value: u8) -> sval::Result;

        fn dispatch_u16(&mut self, value: u16) -> sval::Result;

        fn dispatch_u32(&mut self, value: u32) -> sval::Result;

        fn dispatch_u64(&mut self, value: u64) -> sval::Result;

        fn dispatch_u128(&mut self, value: u128) -> sval::Result;

        fn dispatch_i8(&mut self, value: i8) -> sval::Result;

        fn dispatch_i16(&mut self, value: i16) -> sval::Result;

        fn dispatch_i32(&mut self, value: i32) -> sval::Result;

        fn dispatch_i64(&mut self, value: i64) -> sval::Result;

        fn dispatch_i128(&mut self, value: i128) -> sval::Result;

        fn dispatch_f32(&mut self, value: f32) -> sval::Result;

        fn dispatch_f64(&mut self, value: f64) -> sval::Result;

        fn dispatch_bool(&mut self, value: bool) -> sval::Result;

        fn dispatch_char(&mut self, value: char) -> sval::Result;

        fn dispatch_str<'s: 'a>(&mut self, value: &mut dyn ValueSource<'s, str>) -> sval::Result;

        fn dispatch_text<'s: 'a>(
            &mut self,
            text: &mut dyn ValueSource<'s, sval::data::Text>,
        ) -> sval::Result;

        fn dispatch_error<'e: 'a>(
            &mut self,
            error: &mut dyn ValueSource<'e, sval::data::Error>,
        ) -> sval::Result;

        fn dispatch_bytes<'s: 'a>(
            &mut self,
            bytes: &mut dyn ValueSource<'s, sval::data::Bytes>,
        ) -> sval::Result;

        fn dispatch_tag(
            &mut self,
            tag: sval::data::Tag<&mut dyn ValueSource<'static, str>>,
        ) -> sval::Result;

        fn dispatch_tagged_begin(
            &mut self,
            tag: sval::data::Tag<&mut dyn ValueSource<'static, str>>,
        ) -> sval::Result;

        fn dispatch_tagged_end(
            &mut self,
            tag: sval::data::Tag<&mut dyn ValueSource<'static, str>>,
        ) -> sval::Result;

        fn dispatch_tagged<'v: 'a>(
            &mut self,
            tagged: sval::data::Tagged<&mut dyn ValueSource<'static, str>, &mut dyn Source<'v>>,
        ) -> sval::Result;

        fn dispatch_map_begin(&mut self, size: Option<u64>) -> sval::Result;

        fn dispatch_map_end(&mut self) -> sval::Result;

        fn dispatch_map_key_begin(&mut self) -> sval::Result;

        fn dispatch_map_key_end(&mut self) -> sval::Result;

        fn dispatch_map_value_begin(&mut self) -> sval::Result;

        fn dispatch_map_value_end(&mut self) -> sval::Result;

        fn dispatch_map_entry<'k: 'a, 'v: 'a>(
            &mut self,
            key: &mut dyn Source<'k>,
            value: &mut dyn Source<'v>,
        ) -> sval::Result;

        fn dispatch_map_field_entry<'v: 'a>(
            &mut self,
            field: &mut dyn ValueSource<'static, str>,
            value: &mut dyn Source<'v>,
        ) -> sval::Result;

        fn dispatch_map_field(&mut self, field: &mut dyn ValueSource<'static, str>)
            -> sval::Result;

        fn dispatch_map_key<'k: 'a>(&mut self, key: &mut dyn Source<'k>) -> sval::Result;

        fn dispatch_map_value<'v: 'a>(&mut self, value: &mut dyn Source<'v>) -> sval::Result;

        fn dispatch_seq_begin(&mut self, size: Option<u64>) -> sval::Result;

        fn dispatch_seq_end(&mut self) -> sval::Result;

        fn dispatch_seq_elem_begin(&mut self) -> sval::Result;

        fn dispatch_seq_elem_end(&mut self) -> sval::Result;

        fn dispatch_seq_elem<'e: 'a>(&mut self, elem: &mut dyn Source<'e>) -> sval::Result;
    }

    pub trait EraseReceiver<'a> {
        fn erase_receiver(&mut self) -> crate::private::Erased<&mut dyn DispatchReceiver<'a>>;
    }
}

use crate::{Source, ValueSource};

pub trait Receiver<'a>: private::EraseReceiver<'a> {}

impl<'a, R: sval::Receiver<'a>> Receiver<'a> for R {}

impl<'a, R: sval::Receiver<'a>> private::EraseReceiver<'a> for R {
    fn erase_receiver(&mut self) -> crate::private::Erased<&mut dyn private::DispatchReceiver<'a>> {
        crate::private::Erased(self)
    }
}

impl<'a, R: sval::Receiver<'a>> private::DispatchReceiver<'a> for R {
    fn dispatch_unstructured(&mut self, fmt: &dyn sval::data::Display) -> sval::Result {
        self.unstructured(fmt)
    }

    fn dispatch_null(&mut self) -> sval::Result {
        self.null()
    }

    fn dispatch_u8(&mut self, value: u8) -> sval::Result {
        self.u8(value)
    }

    fn dispatch_u16(&mut self, value: u16) -> sval::Result {
        self.u16(value)
    }

    fn dispatch_u32(&mut self, value: u32) -> sval::Result {
        self.u32(value)
    }

    fn dispatch_u64(&mut self, value: u64) -> sval::Result {
        self.u64(value)
    }

    fn dispatch_u128(&mut self, value: u128) -> sval::Result {
        self.u128(value)
    }

    fn dispatch_i8(&mut self, value: i8) -> sval::Result {
        self.i8(value)
    }

    fn dispatch_i16(&mut self, value: i16) -> sval::Result {
        self.i16(value)
    }

    fn dispatch_i32(&mut self, value: i32) -> sval::Result {
        self.i32(value)
    }

    fn dispatch_i64(&mut self, value: i64) -> sval::Result {
        self.i64(value)
    }

    fn dispatch_i128(&mut self, value: i128) -> sval::Result {
        self.i128(value)
    }

    fn dispatch_f32(&mut self, value: f32) -> sval::Result {
        self.f32(value)
    }

    fn dispatch_f64(&mut self, value: f64) -> sval::Result {
        self.f64(value)
    }

    fn dispatch_bool(&mut self, value: bool) -> sval::Result {
        self.bool(value)
    }

    fn dispatch_char(&mut self, value: char) -> sval::Result {
        self.char(value)
    }

    fn dispatch_str<'s: 'a>(&mut self, value: &mut dyn ValueSource<'s, str>) -> sval::Result {
        self.str(value)
    }

    fn dispatch_text<'s: 'a>(
        &mut self,
        text: &mut dyn ValueSource<'s, sval::data::Text>,
    ) -> sval::Result {
        self.text(text)
    }

    fn dispatch_error<'e: 'a>(
        &mut self,
        error: &mut dyn ValueSource<'e, sval::data::Error>,
    ) -> sval::Result {
        self.error(error)
    }

    fn dispatch_bytes<'s: 'a>(
        &mut self,
        bytes: &mut dyn ValueSource<'s, sval::data::Bytes>,
    ) -> sval::Result {
        self.bytes(bytes)
    }

    fn dispatch_tag(
        &mut self,
        tag: sval::data::Tag<&mut dyn ValueSource<'static, str>>,
    ) -> sval::Result {
        self.tag(tag)
    }

    fn dispatch_tagged_begin(
        &mut self,
        tag: sval::data::Tag<&mut dyn ValueSource<'static, str>>,
    ) -> sval::Result {
        self.tagged_begin(tag)
    }

    fn dispatch_tagged_end(
        &mut self,
        tag: sval::data::Tag<&mut dyn ValueSource<'static, str>>,
    ) -> sval::Result {
        self.tagged_end(tag)
    }

    fn dispatch_tagged<'v: 'a>(
        &mut self,
        tagged: sval::data::Tagged<&mut dyn ValueSource<'static, str>, &mut dyn Source<'v>>,
    ) -> sval::Result {
        self.tagged(tagged)
    }

    fn dispatch_map_begin(&mut self, size: Option<u64>) -> sval::Result {
        self.map_begin(size)
    }

    fn dispatch_map_end(&mut self) -> sval::Result {
        self.map_end()
    }

    fn dispatch_map_key_begin(&mut self) -> sval::Result {
        self.map_key_begin()
    }

    fn dispatch_map_key_end(&mut self) -> sval::Result {
        self.map_key_end()
    }

    fn dispatch_map_value_begin(&mut self) -> sval::Result {
        self.map_value_begin()
    }

    fn dispatch_map_value_end(&mut self) -> sval::Result {
        self.map_value_end()
    }

    fn dispatch_map_entry<'k: 'a, 'v: 'a>(
        &mut self,
        key: &mut dyn Source<'k>,
        value: &mut dyn Source<'v>,
    ) -> sval::Result {
        self.map_entry(key, value)
    }

    fn dispatch_map_field_entry<'v: 'a>(
        &mut self,
        field: &mut dyn ValueSource<'static, str>,
        value: &mut dyn Source<'v>,
    ) -> sval::Result {
        self.map_field_entry(field, value)
    }

    fn dispatch_map_field(&mut self, field: &mut dyn ValueSource<'static, str>) -> sval::Result {
        self.map_field(field)
    }

    fn dispatch_map_key<'k: 'a>(&mut self, key: &mut dyn Source<'k>) -> sval::Result {
        self.map_key(key)
    }

    fn dispatch_map_value<'v: 'a>(&mut self, value: &mut dyn Source<'v>) -> sval::Result {
        self.map_value(value)
    }

    fn dispatch_seq_begin(&mut self, size: Option<u64>) -> sval::Result {
        self.seq_begin(size)
    }

    fn dispatch_seq_end(&mut self) -> sval::Result {
        self.seq_end()
    }

    fn dispatch_seq_elem_begin(&mut self) -> sval::Result {
        self.seq_elem_begin()
    }

    fn dispatch_seq_elem_end(&mut self) -> sval::Result {
        self.seq_elem_end()
    }

    fn dispatch_seq_elem<'e: 'a>(&mut self, elem: &mut dyn Source<'e>) -> sval::Result {
        self.seq_elem(elem)
    }
}

impl<'a, 'd> sval::Receiver<'a> for dyn Receiver<'a> + 'd {
    fn unstructured<D: sval::data::Display>(&mut self, fmt: D) -> sval::Result {
        self.erase_receiver().0.dispatch_unstructured(&fmt)
    }

    fn null(&mut self) -> sval::Result {
        self.erase_receiver().0.dispatch_null()
    }

    fn u8(&mut self, value: u8) -> sval::Result {
        self.erase_receiver().0.dispatch_u8(value)
    }

    fn u16(&mut self, value: u16) -> sval::Result {
        self.erase_receiver().0.dispatch_u16(value)
    }

    fn u32(&mut self, value: u32) -> sval::Result {
        self.erase_receiver().0.dispatch_u32(value)
    }

    fn u64(&mut self, value: u64) -> sval::Result {
        self.erase_receiver().0.dispatch_u64(value)
    }

    fn u128(&mut self, value: u128) -> sval::Result {
        self.erase_receiver().0.dispatch_u128(value)
    }

    fn i8(&mut self, value: i8) -> sval::Result {
        self.erase_receiver().0.dispatch_i8(value)
    }

    fn i16(&mut self, value: i16) -> sval::Result {
        self.erase_receiver().0.dispatch_i16(value)
    }

    fn i32(&mut self, value: i32) -> sval::Result {
        self.erase_receiver().0.dispatch_i32(value)
    }

    fn i64(&mut self, value: i64) -> sval::Result {
        self.erase_receiver().0.dispatch_i64(value)
    }

    fn i128(&mut self, value: i128) -> sval::Result {
        self.erase_receiver().0.dispatch_i128(value)
    }

    fn f32(&mut self, value: f32) -> sval::Result {
        self.erase_receiver().0.dispatch_f32(value)
    }

    fn f64(&mut self, value: f64) -> sval::Result {
        self.erase_receiver().0.dispatch_f64(value)
    }

    fn bool(&mut self, value: bool) -> sval::Result {
        self.erase_receiver().0.dispatch_bool(value)
    }

    fn char(&mut self, value: char) -> sval::Result {
        self.erase_receiver().0.dispatch_char(value)
    }

    fn str<'s: 'a, S: sval::ValueSource<'s, str>>(&mut self, mut value: S) -> sval::Result {
        self.erase_receiver().0.dispatch_str(&mut value)
    }

    fn text<'s: 'a, S: sval::ValueSource<'s, sval::data::Text>>(
        &mut self,
        mut text: S,
    ) -> sval::Result {
        self.erase_receiver().0.dispatch_text(&mut text)
    }

    fn error<'e: 'a, E: sval::ValueSource<'e, sval::data::Error>>(
        &mut self,
        mut error: E,
    ) -> sval::Result {
        self.erase_receiver().0.dispatch_error(&mut error)
    }

    fn bytes<'s: 'a, B: sval::ValueSource<'s, sval::data::Bytes>>(
        &mut self,
        mut bytes: B,
    ) -> sval::Result {
        self.erase_receiver().0.dispatch_bytes(&mut bytes)
    }

    fn tag<T: sval::ValueSource<'static, str>>(
        &mut self,
        mut tag: sval::data::Tag<T>,
    ) -> sval::Result {
        self.erase_receiver().0.dispatch_tag(
            tag.by_mut()
                .map_label(|l| l as &mut dyn ValueSource<'static, str>),
        )
    }

    fn tagged_begin<T: sval::ValueSource<'static, str>>(
        &mut self,
        mut tag: sval::data::Tag<T>,
    ) -> sval::Result {
        self.erase_receiver().0.dispatch_tagged_begin(
            tag.by_mut()
                .map_label(|l| l as &mut dyn ValueSource<'static, str>),
        )
    }

    fn tagged_end<T: sval::ValueSource<'static, str>>(
        &mut self,
        mut tag: sval::data::Tag<T>,
    ) -> sval::Result {
        self.erase_receiver().0.dispatch_tagged_end(
            tag.by_mut()
                .map_label(|l| l as &mut dyn ValueSource<'static, str>),
        )
    }

    fn tagged<'v: 'a, T: sval::ValueSource<'static, str>, V: sval::Source<'v>>(
        &mut self,
        mut tagged: sval::data::Tagged<T, V>,
    ) -> sval::Result {
        self.erase_receiver().0.dispatch_tagged(
            tagged
                .by_mut()
                .map_label(|l| l as &mut dyn ValueSource<'static, str>)
                .map_value(|v| v as &mut dyn Source<'v>),
        )
    }

    fn map_begin(&mut self, size: Option<u64>) -> sval::Result {
        self.erase_receiver().0.dispatch_map_begin(size)
    }

    fn map_end(&mut self) -> sval::Result {
        self.erase_receiver().0.dispatch_map_end()
    }

    fn map_key_begin(&mut self) -> sval::Result {
        self.erase_receiver().0.dispatch_map_key_begin()
    }

    fn map_key_end(&mut self) -> sval::Result {
        self.erase_receiver().0.dispatch_map_key_end()
    }

    fn map_value_begin(&mut self) -> sval::Result {
        self.erase_receiver().0.dispatch_map_value_begin()
    }

    fn map_value_end(&mut self) -> sval::Result {
        self.erase_receiver().0.dispatch_map_value_end()
    }

    fn map_entry<'k: 'a, 'v: 'a, K: sval::Source<'k>, V: sval::Source<'v>>(
        &mut self,
        mut key: K,
        mut value: V,
    ) -> sval::Result {
        self.erase_receiver()
            .0
            .dispatch_map_entry(&mut key, &mut value)
    }

    fn map_field_entry<'v: 'a, F: sval::ValueSource<'static, str>, V: sval::Source<'v>>(
        &mut self,
        mut field: F,
        mut value: V,
    ) -> sval::Result {
        self.erase_receiver()
            .0
            .dispatch_map_field_entry(&mut field, &mut value)
    }

    fn map_field<F: sval::ValueSource<'static, str>>(&mut self, mut field: F) -> sval::Result {
        self.erase_receiver().0.dispatch_map_field(&mut field)
    }

    fn map_key<'k: 'a, K: sval::Source<'k>>(&mut self, mut key: K) -> sval::Result {
        self.erase_receiver().0.dispatch_map_key(&mut key)
    }

    fn map_value<'v: 'a, V: sval::Source<'v>>(&mut self, mut value: V) -> sval::Result {
        self.erase_receiver().0.dispatch_map_value(&mut value)
    }

    fn seq_begin(&mut self, size: Option<u64>) -> sval::Result {
        self.erase_receiver().0.dispatch_seq_begin(size)
    }

    fn seq_end(&mut self) -> sval::Result {
        self.erase_receiver().0.dispatch_seq_end()
    }

    fn seq_elem_begin(&mut self) -> sval::Result {
        self.erase_receiver().0.dispatch_seq_elem_begin()
    }

    fn seq_elem_end(&mut self) -> sval::Result {
        self.erase_receiver().0.dispatch_seq_elem_end()
    }

    fn seq_elem<'e: 'a, E: sval::Source<'e>>(&mut self, mut elem: E) -> sval::Result {
        self.erase_receiver().0.dispatch_seq_elem(&mut elem)
    }
}
