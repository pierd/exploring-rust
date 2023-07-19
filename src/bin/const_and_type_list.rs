use std::{marker::PhantomData, ops::Add};

// List parts
struct Nil;
struct Cons<H, T> {
    head: H,
    tail: T,
}

const fn cons<H, T>(head: H, tail: T) -> Cons<H, T> {
    Cons { head, tail }
}

struct ConstUsize<const N: usize>;

// List operations
trait Len {
    const LEN: usize;
}
const fn len<L: Len>(_: &L) -> usize {
    L::LEN
}

impl Len for Nil {
    const LEN: usize = 0;
}
impl<H, T> Len for Cons<H, T>
where
    T: Len,
{
    const LEN: usize = 1 + T::LEN;
}

trait Sum<T> {
    fn sum(&self) -> T;
}
impl Sum<usize> for Nil {
    fn sum(&self) -> usize {
        0
    }
}
impl<T> Sum<usize> for Cons<usize, T>
where
    T: Sum<usize>,
{
    fn sum(&self) -> usize {
        self.head + self.tail.sum()
    }
}

trait ConstSum {
    const SUM: usize;
}
const fn const_sum<L: ConstSum>(_: &L) -> usize {
    L::SUM
}

impl ConstSum for Nil {
    const SUM: usize = 0;
}
impl<H, T> ConstSum for Cons<H, T>
where
    H: ConstSum,
    T: ConstSum,
{
    const SUM: usize = H::SUM + T::SUM;
}
impl<const N: usize> ConstSum for ConstUsize<N> {
    const SUM: usize = N;
}

#[allow(dead_code)]
mod compilation_stack_overflow {
    use super::*;

    trait Fold<H, T> {
        type Output;

        fn fold(cons: Cons<H, T>) -> Self::Output;
    }

    struct SumFolder<T>(PhantomData<T>);

    impl Fold<usize, Nil> for SumFolder<Nil> {
        type Output = usize;

        fn fold(cons: Cons<usize, Nil>) -> Self::Output {
            cons.head
        }
    }

    impl<T> Fold<usize, Cons<usize, T>> for SumFolder<Cons<usize, T>>
    where
        SumFolder<T>: Fold<usize, T>,
        usize: Add<<SumFolder<T> as Fold<usize, T>>::Output>,
    {
        type Output = <usize as Add<<SumFolder<T> as Fold<usize, T>>::Output>>::Output;

        fn fold(cons: Cons<usize, Cons<usize, T>>) -> Self::Output {
            cons.head + SumFolder::<T>::fold(cons.tail)
        }
    }

    fn sum<T>(list: Cons<usize, T>) -> <SumFolder<T> as Fold<usize, T>>::Output
    where
        SumFolder<T>: Fold<usize, T>,
        usize: Add<<SumFolder<T> as Fold<usize, T>>::Output>,
    {
        SumFolder::<T>::fold(list)
    }
}

fn main() {
    let list = cons(1, cons(2, cons(3, Nil)));
    println!("list.len() = {}", len(&list));
    println!("sum(list) = {}", list.sum());

    let usize_list = cons(
        ConstUsize::<1>,
        cons(ConstUsize::<2>, cons(ConstUsize::<3>, Nil)),
    );
    println!("const_sum(usize_list) = {}", const_sum(&usize_list));
}
