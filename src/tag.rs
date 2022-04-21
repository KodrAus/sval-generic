#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tag {
    Labeled { label: &'static str, id: u64 },
    Unlabeled { id: u64 },
}

// NOTE: `Tag` doesn't implement `Value` because it's not expected
// to have any general representation in the data model. It's up to receivers to
// decide if/how they want to encode tags.
