use std::borrow::Borrow;

#[derive(Debug)]
struct XStr([u8]);

impl XStr {
    fn from_str(s: &str) -> &Self {
        unsafe { std::mem::transmute(s.as_bytes()) }
    }

    fn from_bytes(s: &[u8]) -> &Self {
        unsafe { std::mem::transmute(s) }
    }
}

impl ToOwned for XStr {
    type Owned = XString;

    fn to_owned(&self) -> Self::Owned {
        XString(self.0.to_vec())
    }
}

#[derive(Debug)]
struct XString(Vec<u8>);

impl Borrow<XStr> for XString {
    fn borrow(&self) -> &XStr {
        XStr::from_bytes(&self.0)
    }
}

fn main() {
    let s = XStr::from_str("hello");
    let s2 = s.to_owned();
    println!("s = {:?}, s2 = {:?}", s, s2);
}
