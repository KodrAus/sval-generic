#[cfg(feature = "alloc")]
mod alloc_support {
    use crate::{std::collections::BTreeMap, Result, Stream, Value};

    impl<K: Value, V: Value> Value for BTreeMap<K, V> {
        fn stream<'sval, S: Stream<'sval>>(&'sval self, mut stream: S) -> Result {
            stream.map_begin(Some(self.len()))?;

            for (k, v) in self {
                stream.map_key_begin()?;
                stream.value(k)?;
                stream.map_key_end()?;

                stream.map_value_begin()?;
                stream.value(v)?;
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
        fn stream<'sval, S: Stream<'sval>>(&'sval self, mut stream: S) -> Result {
            stream.map_begin(Some(self.len()))?;

            for (k, v) in self {
                stream.map_key_begin()?;
                stream.value(k)?;
                stream.map_key_end()?;

                stream.map_value_begin()?;
                stream.value(v)?;
                stream.map_value_end()?;
            }

            stream.map_end()
        }

        fn is_dynamic(&self) -> bool {
            false
        }
    }
}
