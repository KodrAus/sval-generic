use syn::{Attribute, Field, Lit, Meta, MetaList, NestedMeta};

pub(crate) fn name_of_field(field: &Field) -> String {
    let mut rename = None;

    for list in field.attrs.iter().filter_map(sval_attr) {
        for meta in list.nested {
            if let NestedMeta::Meta(Meta::NameValue(value)) = meta {
                if value.path.is_ident("rename") && rename.is_none() {
                    if let Lit::Str(s) = value.lit {
                        rename = Some(s.value());
                        continue;
                    }
                }
            }
            panic!("unsupported attribute");
        }
    }

    rename.unwrap_or_else(|| field.ident.as_ref().unwrap().to_string())
}

fn sval_attr(attr: &Attribute) -> Option<MetaList> {
    let segments = &attr.path.segments;
    if !(segments.len() == 1 && segments[0].ident == "sval") {
        return None;
    }

    match attr.parse_meta().ok() {
        Some(Meta::List(list)) => Some(list),
        _ => panic!("unsupported attribute"),
    }
}
