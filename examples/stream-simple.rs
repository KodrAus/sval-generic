fn main() -> sval::Result {
    stream(42);
    stream(true);

    stream(Some(42));
    stream(None::<i32>);

    stream({
        use std::collections::BTreeMap;

        let mut map = BTreeMap::new();

        map.insert("a", 1);
        map.insert("b", 2);
        map.insert("c", 3);

        map
    });

    stream(vec![vec!["Hello", "world"], vec!["Hello", "world"]]);

    Ok(())
}

fn stream(v: impl sval::Value) {
    use sval::Stream;

    MyStream.value(&v).expect("failed to stream");
    println!();
}

pub struct MyStream;

impl<'sval> sval::Stream<'sval> for MyStream {
    fn null(&mut self) -> sval::Result {
        print!("null");
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

    fn binary_begin(&mut self, _: Option<usize>) -> sval::Result {
        Ok(())
    }

    fn binary_fragment_computed(&mut self, fragment: &[u8]) -> sval::Result {
        for b in fragment {
            print!("{:x}", b);
        }
        Ok(())
    }

    fn binary_end(&mut self) -> sval::Result {
        Ok(())
    }

    fn map_begin(&mut self, _: Option<usize>) -> sval::Result {
        print!("{{ ");
        Ok(())
    }

    fn map_key_begin(&mut self) -> sval::Result {
        Ok(())
    }

    fn map_key_end(&mut self) -> sval::Result {
        Ok(())
    }

    fn map_value_begin(&mut self) -> sval::Result {
        print!(": ");
        Ok(())
    }

    fn map_value_end(&mut self) -> sval::Result {
        print!(", ");
        Ok(())
    }

    fn map_end(&mut self) -> sval::Result {
        print!("}}");
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

#[cfg(not(feature = "alloc"))]
compile_error!("this sample requires the `alloc` feature to be enabled");
