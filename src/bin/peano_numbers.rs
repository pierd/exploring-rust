#![recursion_limit = "256"]

use std::marker::PhantomData;

struct Zero;
struct Succ<N>(PhantomData<N>);

trait Value {
    const VALUE: usize;
}
impl Value for Zero {
    const VALUE: usize = 0;
}
impl<N> Value for Succ<N>
where
    N: Value,
{
    const VALUE: usize = 1 + <N as Value>::VALUE;
}

trait CalculateAdd<Num> {
    type Output;
}
impl<N> CalculateAdd<N> for Zero {
    type Output = N;
}
impl<N, M> CalculateAdd<N> for Succ<M>
where
    M: CalculateAdd<N>,
{
    type Output = Succ<<M as CalculateAdd<N>>::Output>;
}
type Add<A, B> = <A as CalculateAdd<B>>::Output;
type Double<A> = Add<A, A>;

/// Macro to create a type from a list of `x` and `o` tokens representing the number in binary.
macro_rules! num {
    ($($b:ident) *) => {
        num!(Zero; $($b) *)
    };
    ($prev:ty ; o $($tail:ident) *) => {
        num!(Double<$prev>; $($tail) *)
    };
    ($prev:ty ; x $($tail:ident) *) => {
        num!(Add<Double<$prev>, Succ<Zero>>; $($tail) *)
    };
    ($prev:ty ; ) => {
        $prev
    };
}

fn main() {
    println!("2 + 3 = {}", <Add<num!(x o), num!(x x)> as Value>::VALUE);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        type One = Succ<Zero>;
        type Two = Succ<One>;
        type Three = Succ<Two>;
        type Four = Succ<Three>;

        let two_plus_three: usize = <Add<Two, Three> as Value>::VALUE;
        assert_eq!(two_plus_three, 5);
        let one_plus_four: usize = <Add<One, Four> as Value>::VALUE;
        assert_eq!(one_plus_four, 5);
    }

    #[test]
    fn test_num_macro() {
        assert_eq!(<num!(x) as Value>::VALUE, 1);
        assert_eq!(<num!(x o) as Value>::VALUE, 2);
        assert_eq!(<num!(x o x o) as Value>::VALUE, 10);
        assert_eq!(<num!(x x x x x x x x) as Value>::VALUE, 255);
    }
}
