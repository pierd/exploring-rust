use std::{rc::Rc, sync::Arc};

#[derive(Debug)]
struct X {
    a: i32,
}
impl PartialEq for X {
    fn eq(&self, other: &Self) -> bool {
        eprintln!("Comparing X");
        self.a == other.a
    }
}

#[derive(Debug, Eq)]
struct Y {
    a: i32,
}
impl PartialEq for Y {
    fn eq(&self, other: &Self) -> bool {
        eprintln!("Comparing Y");
        self.a == other.a
    }
}

fn main() {
    {
        eprintln!("Rc X");
        let x1 = Rc::new(X { a: 1 });
        let x2 = Rc::new(X { a: 1 });
        let x3 = x1.clone();
        assert_eq!(x1, x2);
        assert_eq!(x1, x3);
    }
    {
        eprintln!("Rc Y");
        let x1 = Rc::new(Y { a: 1 });
        let x2 = Rc::new(Y { a: 1 });
        let x3 = x1.clone();
        assert_eq!(x1, x2);
        assert_eq!(x1, x3);
    }
    {
        eprintln!("Arc X");
        let x1 = Arc::new(X { a: 1 });
        let x2 = Arc::new(X { a: 1 });
        let x3 = x1.clone();
        assert_eq!(x1, x2);
        assert_eq!(x1, x3);
    }
    {
        eprintln!("Arc Y");
        let x1 = Arc::new(Y { a: 1 });
        let x2 = Arc::new(Y { a: 1 });
        let x3 = x1.clone();
        assert_eq!(x1, x2);
        assert_eq!(x1, x3);
    }
}
