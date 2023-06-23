macro_rules! xtry {
    ($e:expr) => {
        match $e {
            Ok(t) => t,
            Err(e) => return Err(e.into()),
        }
    };
}

fn ok<T>(x: T) -> Result<T, String> {
    Ok(x)
}

fn err<T>(x: T) -> Result<(), T> {
    Err(x)
}

fn main() -> Result<(), String> {
    let x = xtry!(ok(1));
    println!("x = {}", x);

    let _ = xtry!(err("boom!"));

    unreachable!()
}
