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
            let mut short = |s: &str| stream.map_field("field", value::any_ref(s));

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

        fn str<V: stream::ValueRef<'a, Target = str>>(&mut self, v: V) -> stream::Result {
            match v.try_into_ref() {
                Ok(v) => println!("borrowed: {}", v),
                Err(stream::IntoRefError(v)) => println!("short: {}", &*v),
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

    MyValue.stream(MyStream(None)).unwrap();

    let my_struct = MyStruct {
        a: String::from("hello!"),
    };

    my_struct.stream(MyStream(None)).unwrap();

    let erased_value = &MyValue as &dyn erased::Value;
    let erased_stream = &mut MyStream(None) as &mut dyn erased::Stream<'_>;

    erased_value.stream(MyStream(None)).unwrap();

    MyValue.stream(&mut *erased_stream).unwrap();

    erased_value.stream(&mut *erased_stream).unwrap();
}
