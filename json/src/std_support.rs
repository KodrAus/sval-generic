use sval::Source;

pub fn to_string<'a>(v: impl Source<'a>) -> Result<String, sval::Error> {
    let mut out = String::new();
    crate::to_fmt(&mut out, v)?;

    Ok(out)
}
