use std::{error, fmt};

use serde::ser::{Serialize, SerializeMap, SerializeSeq, Serializer};

use crate::{
    stream::{self, Stream},
    value::Value,
};

struct SerdeStream<S: Serializer>(Option<Serde<S>>);

enum Serde<S: Serializer> {
    Serializer(S),
    SerializeMap(S::SerializeMap),
    SerializeSeq(S::SerializeSeq),
    Ok(S::Ok),
}
