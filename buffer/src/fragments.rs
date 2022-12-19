use crate::std::ops::Deref;

#[cfg(feature = "alloc")]
use crate::std::borrow::{Cow, ToOwned};

pub struct TextBuf<'sval>(FragmentBuf<'sval, str>);

impl<'sval> TextBuf<'sval> {
    pub fn new() -> Self {
        TextBuf(FragmentBuf::new(""))
    }

    pub fn push_fragment(&mut self, fragment: &'sval str) -> sval::Result {
        self.0.push(fragment)
    }

    pub fn push_fragment_computed(&mut self, fragment: &str) -> sval::Result {
        self.0.push_computed(fragment)
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
        self.0.push(fragment)
    }

    pub fn push_fragment_computed(&mut self, fragment: &[u8]) -> sval::Result {
        self.0.push_computed(fragment)
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

#[cfg(not(feature = "alloc"))]
trait Fragment {
    fn to_fragment<'sval>(&'sval self) -> &'sval self {
        self
    }

    fn can_replace(&self) -> bool;
}

#[cfg(feature = "alloc")]
trait Fragment: ToOwned {
    fn to_fragment<'sval>(&'sval self) -> Cow<'sval, Self> {
        Cow::Borrowed(self)
    }

    fn extend(buf: &mut Cow<Self>, fragment: &Self);

    fn can_replace(&self) -> bool;
}

impl Fragment for str {
    #[cfg(feature = "alloc")]
    fn extend(buf: &mut Cow<Self>, fragment: &Self) {
        buf.to_mut().push_str(fragment);
    }

    fn can_replace(&self) -> bool {
        self.len() == 0
    }
}

impl Fragment for [u8] {
    #[cfg(feature = "alloc")]
    fn extend(buf: &mut Cow<Self>, fragment: &Self) {
        buf.to_mut().extend(fragment);
    }

    fn can_replace(&self) -> bool {
        self.len() == 0
    }
}

struct FragmentBuf<'sval, T: ?Sized + Fragment> {
    #[cfg(not(feature = "alloc"))]
    value: &'sval T,
    #[cfg(feature = "alloc")]
    value: Cow<'sval, T>,
}

impl<'sval, T: ?Sized + Fragment> FragmentBuf<'sval, T> {
    fn new(value: &'sval T) -> Self {
        FragmentBuf {
            value: value.to_fragment(),
        }
    }
}

impl<'sval, T: ?Sized + Fragment> FragmentBuf<'sval, T> {
    fn push(&mut self, fragment: &'sval T) -> sval::Result {
        if self.value.can_replace() {
            self.value = fragment.to_fragment();

            Ok(())
        } else {
            self.push_computed(fragment)
        }
    }

    fn push_computed(&mut self, fragment: &T) -> sval::Result {
        #[cfg(feature = "alloc")]
        {
            Fragment::extend(&mut self.value, fragment);

            Ok(())
        }

        #[cfg(not(feature = "alloc"))]
        {
            Err(sval::Error::unsupported())
        }
    }

    fn try_get(&self) -> Option<&'sval T> {
        #[cfg(feature = "alloc")]
        {
            match self.value {
                Cow::Borrowed(value) => Some(value),
                Cow::Owned(_) => None,
            }
        }

        #[cfg(not(feature = "alloc"))]
        {
            Some(self.value)
        }
    }

    fn get(&self) -> &T {
        #[cfg(feature = "alloc")]
        {
            &*self.value
        }

        #[cfg(not(feature = "alloc"))]
        {
            self.value
        }
    }
}
