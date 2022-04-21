use alloc::string::String;

pub fn to_string(v: impl sval::Value) -> sval::Result<String> {
    let mut out = String::new();
    crate::to_fmt(&mut out, v)?;

    Ok(out)
}
