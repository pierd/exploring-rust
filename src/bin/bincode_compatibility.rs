#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
enum Foo {
    Bar,
    Baz,
}

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
enum FooExtended {
    Bar,
    Baz,
    Qux,
}

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
enum NotFoo {
    Very,
    Different,
    Enum(#[serde(default)] usize),
}

fn main() {
    for x in &[Foo::Bar, Foo::Baz] {
        let serialized = bincode::serialize(x).unwrap();
        println!("Foo::{:?} -> {:?}", x, &serialized);
        println!("-> Foo {:?}", bincode::deserialize::<Foo>(&serialized));
        println!(
            "-> FooExtended {:?}",
            bincode::deserialize::<FooExtended>(&serialized)
        );
        println!(
            "-> NotFoo {:?}",
            bincode::deserialize::<NotFoo>(&serialized)
        );
        println!();
    }
    println!();

    for x in &[FooExtended::Bar, FooExtended::Baz, FooExtended::Qux] {
        let serialized = bincode::serialize(x).unwrap();
        println!("FooExtended::{:?} -> {:?}", x, &serialized);
        println!("-> Foo {:?}", bincode::deserialize::<Foo>(&serialized));
        println!(
            "-> FooExtended {:?}",
            bincode::deserialize::<FooExtended>(&serialized)
        );
        println!(
            "-> NotFoo {:?}",
            bincode::deserialize::<NotFoo>(&serialized)
        );
        println!();
    }
    println!();

    for x in &[NotFoo::Very, NotFoo::Different, NotFoo::Enum(42)] {
        let serialized = bincode::serialize(x).unwrap();
        println!("NotFoo::{:?} -> {:?}", x, &serialized);
        println!("-> Foo {:?}", bincode::deserialize::<Foo>(&serialized));
        println!(
            "-> FooExtended {:?}",
            bincode::deserialize::<FooExtended>(&serialized)
        );
        println!(
            "-> NotFoo {:?}",
            bincode::deserialize::<NotFoo>(&serialized)
        );
        println!();
    }
}
