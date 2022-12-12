pub struct MyStream;

impl<'sval> sval::Stream<'sval> for MyStream {
    fn null(&mut self) -> sval::Result {
        print!("null");
        Ok(())
    }

    fn bool(&mut self, v: bool) -> sval::Result {
        print!("{}", v);
        Ok(())
    }

    fn i64(&mut self, v: i64) -> sval::Result {
        print!("{}", v);
        Ok(())
    }

    fn f64(&mut self, v: f64) -> sval::Result {
        print!("{}", v);
        Ok(())
    }

    fn text_begin(&mut self, _: Option<usize>) -> sval::Result {
        print!("\"");
        Ok(())
    }

    fn text_fragment_computed(&mut self, fragment: &str) -> sval::Result {
        print!("{}", fragment.escape_debug());

        Ok(())
    }

    fn text_end(&mut self) -> sval::Result {
        print!("\"");
        Ok(())
    }

    fn seq_begin(&mut self, _: Option<usize>) -> sval::Result {
        print!("[ ");
        Ok(())
    }

    fn seq_value_begin(&mut self) -> sval::Result {
        Ok(())
    }

    fn seq_value_end(&mut self) -> sval::Result {
        print!(", ");
        Ok(())
    }

    fn seq_end(&mut self) -> sval::Result {
        print!("]");
        Ok(())
    }
}

fn main() -> sval::Result {
    stream(42);
    stream(true);

    stream(Some(42));
    stream(None::<i32>);

    #[cfg(feature = "alloc")]
    stream({
        use std::collections::BTreeMap;

        let mut map = BTreeMap::new();

        map.insert("a", 1);
        map.insert("b", 2);
        map.insert("c", 3);

        map
    });

    #[cfg(feature = "alloc")]
    stream(vec![vec!["Hello", "world"], vec!["Hello", "world"]]);

    Ok(())
}

fn stream(v: impl sval::Value) {
    v.stream(&mut MyStream).expect("failed to stream");
    println!();
}
