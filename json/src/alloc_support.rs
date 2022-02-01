use alloc::string::String;

pub fn to_string<'a>(v: impl sval::Source<'a>) -> Result<String, sval::Error> {
    let mut out = String::new();
    crate::to_fmt(&mut out, v)?;

    Ok(out)
}
