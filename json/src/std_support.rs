use sval_generic_api::Source;

pub fn to_string<'a>(v: impl Source<'a>) -> Result<String, sval_generic_api::Error> {
    let mut out = String::new();
    crate::to_fmt(&mut out, v)?;

    Ok(out)
}
