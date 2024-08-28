#[derive(Debug, PartialEq)]
struct JustClone(&'static str);
impl Clone for JustClone {
    fn clone(&self) -> Self {
        eprintln!("Cloning {:?}", self.0);
        JustClone(self.0)
    }
}

#[derive(Debug, PartialEq)]
struct CloneAndCopy(&'static str);
impl Clone for CloneAndCopy {
    fn clone(&self) -> Self {
        eprintln!("Cloning {:?}", self.0);
        CloneAndCopy(self.0)
    }
}
impl Copy for CloneAndCopy {}

fn main() {
    {
        eprintln!("JustClone");
        let x1 = JustClone("just Clone");
        let x2 = JustClone("just Clone");
        let x3 = x1.clone();
        assert_eq!(x1, x2);
        assert_eq!(x1, x3);
    }
    {
        eprintln!("CloneAndCopy");
        let x1 = CloneAndCopy("Clone and Copy");
        let x2 = CloneAndCopy("Clone and Copy");
        let x3 = x1;
        assert_eq!(x1, x2);
        assert_eq!(x1, x3);
    }
}
