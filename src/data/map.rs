#[cfg(feature = "alloc")]
mod alloc_support {
    use crate::{std::collections::BTreeMap, Result, Stream, Value};

    impl<K: Value, V: Value> Value for BTreeMap<K, V> {
        fn stream<'sval, S: Stream<'sval> + ?Sized>(&'sval self, stream: &mut S) -> Result {
            stream.map_begin(Some(self.len()))?;

            for (k, v) in self {
                stream.map_key_begin()?;
                k.stream(stream)?;
                stream.map_key_end()?;

                stream.map_value_begin()?;
                v.stream(stream)?;
                stream.map_value_end()?;
            }

            stream.map_end()
        }

        fn is_dynamic(&self) -> bool {
            false
        }
    }
}

#[cfg(feature = "std")]
mod std_support {
    use crate::{
        std::{collections::HashMap, hash::BuildHasher},
        Result, Stream, Value,
    };

    impl<K: Value, V: Value, H: BuildHasher> Value for HashMap<K, V, H> {
        fn stream<'sval, S: Stream<'sval> + ?Sized>(&'sval self, stream: &mut S) -> Result {
            stream.map_begin(Some(self.len()))?;

            for (k, v) in self {
                stream.map_key_begin()?;
                k.stream(stream)?;
                stream.map_key_end()?;

                stream.map_value_begin()?;
                v.stream(stream)?;
                stream.map_value_end()?;
            }

            stream.map_end()
        }

        fn is_dynamic(&self) -> bool {
            false
        }
    }
}
