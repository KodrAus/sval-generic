/**
A data tag that may be treated as either in-band or out-of-band.

Tags identify distinct data types and fragments.
Tags nest in a value to form a context, which is the scope of their canonicalization.
For example, the scope of a tag on a top-level value is the set of all values, whereas
the scope of that tag in an enum is the set of its variants, and the scope of a tag
on some field in a struct is the set of fields on that struct.

The canonical id field is large enough to fit a UUID.
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tag {
    /**
    A tag that carries a name and optional canonical id.
    */
    Named {
        /**
        The name of the tagged data.

        Names are not canonical.
        */
        name: &'static str,
        /**
        A canonical id for the tagged data.

        Ids are unique to each type, but shared by all instances.
        If a type can't guarantee uniqueness this field can be left `None`.
        */
        id: Option<u128>,
    },
    /**
    A tag that carries a canonical id.
    */
    Unnamed {
        /**
        A canonical id for the tagged data.

        Ids are unique to each type, but shared by all instances.
        */
        id: u128,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TagNamed {
    pub name: &'static str,
    pub id: Option<u128>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TagUnnamed {
    pub id: u128,
}

// NOTE: `Tag` doesn't implement `Value` because it's not expected
// to have any general representation in the data model. It's up to receivers to
// decide if/how they want to encode tags.
