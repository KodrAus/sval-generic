use std::fmt;

use sval::{
    data::Display,
    receiver::{Receiver, Size},
    Value,
};

struct MyFormat<W>(W);

impl<'a, W> Receiver<'a> for MyFormat<W>
where
    W: fmt::Write,
{
    fn unstructured<D: Display>(&mut self, fmt: D) -> sval::Result {
        write!(&mut self.0, "\"{}\"", fmt)?;

        Ok(())
    }

    fn null(&mut self) -> sval::Result {
        write!(&mut self.0, "null")?;

        Ok(())
    }

    fn map_begin(&mut self, _size: Size) -> sval::Result {
        write!(&mut self.0, "{{ ")?;

        Ok(())
    }

    fn map_end(&mut self) -> sval::Result {
        write!(&mut self.0, "}}")?;

        Ok(())
    }

    fn map_key_begin(&mut self) -> sval::Result {
        Ok(())
    }

    fn map_key_end(&mut self) -> sval::Result {
        write!(&mut self.0, ": ")?;

        Ok(())
    }

    fn map_value_begin(&mut self) -> sval::Result {
        Ok(())
    }

    fn map_value_end(&mut self) -> sval::Result {
        write!(&mut self.0, ", ")?;

        Ok(())
    }

    fn seq_begin(&mut self, _size: Size) -> sval::Result {
        write!(&mut self.0, "[ ")?;

        Ok(())
    }

    fn seq_end(&mut self) -> sval::Result {
        write!(&mut self.0, "]")?;

        Ok(())
    }

    fn seq_elem_begin(&mut self) -> sval::Result {
        Ok(())
    }

    fn seq_elem_end(&mut self) -> sval::Result {
        write!(&mut self.0, ", ")?;

        Ok(())
    }
}

struct MyStruct;

impl Value for MyStruct {
    fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> sval::Result {
        receiver.map_begin(Size::Unknown)?;

        receiver.map_field_entry("a", 42)?;
        receiver.map_field_entry("b", true)?;
        receiver.map_field_entry("c", "A string!")?;

        receiver.map_end()
    }
}

struct MySeq;

impl Value for MySeq {
    fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> sval::Result {
        receiver.seq_begin(Size::Unknown)?;

        receiver.seq_elem(42)?;
        receiver.seq_elem(true)?;
        receiver.seq_elem("A string!")?;

        receiver.seq_end()
    }
}

fn stream(v: impl Value) {
    let mut fmt = MyFormat(String::new());

    v.stream(&mut fmt).expect("failed to stream");

    println!("{}", fmt.0);
}

fn main() {
    stream(MyStruct);
    stream(MySeq);
}
