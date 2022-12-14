use std::ops::Deref;

#[cfg(feature = "alloc")]
use std::borrow::Cow;

pub struct TextBuf<'sval>(FragmentBuf<'sval, str>);

impl<'sval> TextBuf<'sval> {
    pub fn new() -> Self {
        TextBuf(FragmentBuf::new(""))
    }

    pub fn push_fragment(&mut self, fragment: &'sval str) -> sval::Result {
        self.0.push(fragment, |value| value.len() == 0)
    }

    pub fn try_get(&self) -> Option<&'sval str> {
        self.0.try_get()
    }

    pub fn get(&self) -> &str {
        self.0.get()
    }
}

impl<'sval> From<&'sval str> for TextBuf<'sval> {
    fn from(fragment: &'sval str) -> Self {
        TextBuf(FragmentBuf::new(fragment))
    }
}

impl<'sval> AsRef<str> for TextBuf<'sval> {
    fn as_ref(&self) -> &str {
        self.get()
    }
}

impl<'sval> Deref for TextBuf<'sval> {
    type Target = str;

    fn deref(&self) -> &str {
        self.get()
    }
}

pub struct BinaryBuf<'sval>(FragmentBuf<'sval, [u8]>);

impl<'sval> BinaryBuf<'sval> {
    pub fn new() -> Self {
        BinaryBuf(FragmentBuf::new(&[]))
    }

    pub fn push_fragment(&mut self, fragment: &'sval [u8]) -> sval::Result {
        self.0.push(fragment, |value| value.len() == 0)
    }

    pub fn try_get(&self) -> Option<&'sval [u8]> {
        self.0.try_get()
    }

    pub fn get(&self) -> &[u8] {
        self.0.get()
    }
}

impl<'sval> From<&'sval [u8]> for BinaryBuf<'sval> {
    fn from(fragment: &'sval [u8]) -> Self {
        BinaryBuf(FragmentBuf::new(fragment))
    }
}

impl<'sval> AsRef<[u8]> for BinaryBuf<'sval> {
    fn as_ref(&self) -> &[u8] {
        self.get()
    }
}

impl<'sval> Deref for BinaryBuf<'sval> {
    type Target = [u8];

    fn deref(&self) -> &[u8] {
        self.get()
    }
}

struct FragmentBuf<'sval, T: ?Sized> {
    #[cfg(not(feature = "alloc"))]
    value: &'sval T,
    #[cfg(feature = "alloc")]
    value: Cow<'sval, T>,
}

impl<'sval, T: ?Sized> FragmentBuf<'sval, T> {
    fn new(value: &'sval T) -> Self {
        FragmentBuf {
            value: value.into(),
        }
    }
}

impl<'sval, T: ?Sized> FragmentBuf<'sval, T> {
    fn push(&mut self, fragment: &'sval T, can_replace: impl FnOnce(&T) -> bool) -> sval::Result {
        #[cfg(not(feature = "alloc"))]
        {
            let _ = extend;

            if can_replace(self.value) {
                self.value = fragment;

                Ok(())
            } else {
                Err(sval::Error::unsupported())
            }
        }

        #[cfg(feature = "alloc")]
        {
            if can_replace(&self.value) {
                self.value = fragment.into();
            } else {
                self.value.get_mut()
            }

            Ok(())
        }
    }

    fn try_get(&self) -> Option<&'sval T> {
        #[cfg(not(feature = "alloc"))]
        {
            Some(self.value)
        }
    }

    fn get(&self) -> &T {
        #[cfg(not(feature = "alloc"))]
        {
            self.value
        }
    }
}
