use sval_ref_api::{
    erased,
    stream::{self, Stream},
    value::{self, Value},
};

fn main() {
    struct MyValue;

    impl Value for MyValue {
        fn stream<'a, S>(&'a self, mut stream: S) -> value::Result
        where
            S: value::Stream<'a>,
        {
            let mut short = |s: &str| stream.map_field("field", value::for_all(s));

            short("value")
        }
    }

    struct MyStruct {
        a: String,
    }

    impl Value for MyStruct {
        fn stream<'a, S>(&'a self, mut stream: S) -> value::Result
        where
            S: value::Stream<'a>,
        {
            stream.map_field("a", &*self.a)
        }
    }

    struct MyStream<'a>(Option<&'a str>);

    impl<'a> Stream<'a> for MyStream<'a> {
        fn u128(&mut self, _: u128) -> stream::Result {
            Ok(())
        }

        fn i128(&mut self, _: i128) -> stream::Result {
            Ok(())
        }

        fn str<V: stream::TypedValue<'a, str>>(&mut self, v: V) -> stream::Result {
            if let Some(v) = v.to_ref() {
                println!("borrowed: {}", v);
            } else {
                println!("short: {}", &*v);
            }

            Ok(())
        }

        fn map_begin(&mut self, _: Option<usize>) -> stream::Result {
            Ok(())
        }

        fn map_key_begin(&mut self) -> stream::Result {
            Ok(())
        }

        fn map_value_begin(&mut self) -> stream::Result {
            Ok(())
        }

        fn map_end(&mut self) -> stream::Result {
            Ok(())
        }
    }

    println!("1");
    MyValue.stream(MyStream(None)).unwrap();

    let my_struct = MyStruct {
        a: String::from("hello!"),
    };

    println!("2");
    my_struct.stream(MyStream(None)).unwrap();

    let erased_value = &MyValue as &dyn erased::Value;
    let erased_stream = &mut MyStream(None) as &mut dyn erased::Stream<'_>;

    println!("3");
    erased_value.stream(MyStream(None)).unwrap();

    println!("4");
    MyValue.stream(&mut *erased_stream).unwrap();

    println!("5");
    erased_value.stream(&mut *erased_stream).unwrap();
}
