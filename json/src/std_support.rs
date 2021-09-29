use sval_generic_api::stream;

pub fn to_string<'a>(v: impl stream::SourceRef<'a>) -> Result<String, stream::Error> {
    let mut out = String::new();
    crate::to_fmt(&mut out, v)?;

    Ok(out)
}
