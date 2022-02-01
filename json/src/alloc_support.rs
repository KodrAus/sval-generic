use alloc::string::String;

pub fn to_string<'a>(v: impl sval::Source<'a>) -> sval::Result<String> {
    let mut out = String::new();
    crate::to_fmt(&mut out, v)?;

    Ok(out)
}
