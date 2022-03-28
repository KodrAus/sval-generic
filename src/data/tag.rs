#[inline]
pub fn tag() -> Tag {
    Tag::new()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Tag {
    // NOTE: These fields are public and `Tag` is constructable (not non-exhaustive)
    // *specifically* to limit any possible changes to its shape.
    pub label: Option<&'static str>,
    pub id: Option<u64>,
}

impl Default for Tag {
    #[inline]
    fn default() -> Self {
        Tag {
            label: Default::default(),
            id: Default::default(),
        }
    }
}

impl Tag {
    #[inline]
    pub fn new() -> Self {
        Tag::default()
    }

    #[inline]
    pub fn with_label(self, label: impl Into<Option<&'static str>>) -> Tag {
        Tag {
            label: label.into(),
            id: self.id,
        }
    }

    #[inline]
    pub fn with_id(self, id: impl Into<Option<u64>>) -> Self {
        Tag {
            label: self.label,
            id: id.into(),
        }
    }
}
