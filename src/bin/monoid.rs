use std::rc::Rc;

trait Semigroup {
    type T;
    fn append(a: Self::T, b: Self::T) -> Self::T;
}

trait Monoid: Semigroup {
    fn identity() -> Self::T;

    fn times(a: Self::T, n: usize) -> Self::T
    where
        Self::T: Clone,
    {
        let mut res = Self::identity();
        for _ in 0..n {
            res = Self::append(res, a.clone());
        }
        res
    }
}

struct Add;
impl Semigroup for Add {
    type T = usize;

    fn append(a: usize, b: usize) -> usize {
        // saturating so we don't have to worry about overflows
        a.saturating_add(b)
    }
}
impl Monoid for Add {
    fn identity() -> usize {
        0
    }
}

struct Mul;
impl Semigroup for Mul {
    type T = usize;

    fn append(a: usize, b: usize) -> usize {
        // saturating so we don't have to worry about overflows
        a.saturating_mul(b)
    }
}
impl Monoid for Mul {
    fn identity() -> usize {
        1
    }
}

struct Max;
impl Semigroup for Max {
    type T = usize;

    fn append(a: usize, b: usize) -> usize {
        a.max(b)
    }
}
impl Monoid for Max {
    fn identity() -> usize {
        usize::MIN
    }
}

#[derive(Clone, Debug, Default)]
struct VecAppend<T>(std::marker::PhantomData<T>);
impl<T> Semigroup for VecAppend<T> {
    type T = Vec<T>;

    fn append(mut a: Vec<T>, mut b: Vec<T>) -> Vec<T> {
        a.append(&mut b);
        a
    }
}
impl<T> Monoid for VecAppend<T> {
    fn identity() -> Vec<T> {
        Vec::new()
    }
}

struct StringAppend;
impl Semigroup for StringAppend {
    type T = String;

    fn append(mut a: String, b: String) -> String {
        a.push_str(&b);
        a
    }
}
impl Monoid for StringAppend {
    fn identity() -> String {
        String::new()
    }
}

impl<A: Semigroup> Semigroup for (A,) {
    type T = (A::T,);

    fn append(a: Self::T, b: Self::T) -> Self::T {
        (A::append(a.0, b.0),)
    }
}
impl<A: Monoid> Monoid for (A,) {
    fn identity() -> Self::T {
        (A::identity(),)
    }
}

impl<A, B> Semigroup for (A, B)
where
    A: Semigroup,
    B: Semigroup,
{
    type T = (A::T, B::T);

    fn append(a: Self::T, b: Self::T) -> Self::T {
        (A::append(a.0, b.0), B::append(a.1, b.1))
    }
}
impl<A: Monoid, B: Monoid> Monoid for (A, B) {
    fn identity() -> Self::T {
        (A::identity(), B::identity())
    }
}

impl<A, B, C> Semigroup for (A, B, C)
where
    A: Semigroup,
    B: Semigroup,
    C: Semigroup,
{
    type T = (A::T, B::T, C::T);

    fn append(a: Self::T, b: Self::T) -> Self::T {
        (
            A::append(a.0, b.0),
            B::append(a.1, b.1),
            C::append(a.2, b.2),
        )
    }
}
impl<A, B, C> Monoid for (A, B, C)
where
    A: Monoid,
    B: Monoid,
    C: Monoid,
{
    fn identity() -> Self::T {
        (A::identity(), B::identity(), C::identity())
    }
}

type Fun<Args, Return> = Rc<dyn Fn(Args) -> Return>;

struct TraverseFn<A, B>(std::marker::PhantomData<(A, B)>);
impl<A, B> Semigroup for TraverseFn<A, B>
where
    A: Clone + 'static,
    B: Monoid,
    B::T: 'static,
{
    type T = Fun<A, B::T>;

    fn append(a: Self::T, b: Self::T) -> Self::T {
        Rc::new(move |x| B::append(a(x.clone()), b(x)))
    }
}
impl<A, B> Monoid for TraverseFn<A, B>
where
    A: Clone + 'static,
    B: Monoid,
    B::T: 'static,
{
    fn identity() -> Self::T {
        Rc::new(|_| B::identity())
    }
}

type Endomorphism<A> = Fun<A, A>;

struct ComposeEndomorphism<A>(std::marker::PhantomData<A>);
impl<A> Semigroup for ComposeEndomorphism<A>
where
    A: 'static,
{
    type T = Endomorphism<A>;

    fn append(a: Self::T, b: Self::T) -> Self::T {
        Rc::new(move |x| a(b(x)))
    }
}
impl<A> Monoid for ComposeEndomorphism<A>
where
    A: 'static,
{
    fn identity() -> Self::T {
        Rc::new(|x| x)
    }
}

impl<A: Semigroup> Semigroup for Option<A> {
    type T = Option<A::T>;

    fn append(a: Self::T, b: Self::T) -> Self::T {
        match (a, b) {
            (Some(a), Some(b)) => Some(A::append(a, b)),
            (Some(a), None) | (None, Some(a)) => Some(a),
            (None, None) => None,
        }
    }
}
impl<A: Semigroup> Monoid for Option<A> {
    fn identity() -> Self::T {
        None
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use quickcheck::{Arbitrary, Gen, TestResult, Testable};
    use quickcheck_macros::quickcheck;

    use super::*;

    fn check_identity<A: Monoid>(a: A::T)
    where
        A::T: Clone + std::fmt::Debug + PartialEq + Eq,
    {
        assert_eq!(A::append(a.clone(), A::identity()), a);
        assert_eq!(A::append(A::identity(), a.clone()), a);
    }

    fn check_associative<A: Semigroup>(a: A::T, b: A::T, c: A::T)
    where
        A::T: Clone + std::fmt::Debug + PartialEq + Eq,
    {
        assert_eq!(
            A::append(A::append(a.clone(), b.clone()), c.clone()),
            A::append(a, A::append(b, c))
        );
    }

    struct EqualFunTestable<Args, Return> {
        f: Fun<Args, Return>,
        g: Fun<Args, Return>,
    }
    impl<Args, Return> EqualFunTestable<Args, Return>
    where
        Args: Arbitrary + Clone,
        Return: PartialEq,
    {
        fn check(&self, x: Args) -> bool {
            (self.f)(x.clone()) == (self.g)(x)
        }

        fn shrink_failure(
            &self,
            g: &mut Gen,
            args: Args,
        ) -> Option<TestResult> {
            for t in args.shrink() {
                let new_args = t.clone();
                let r = self.check(new_args).result(g);
                if r.is_failure() {
                    // The shrunk value *does* witness a failure, so keep
                    // trying to shrink it.
                    let shrunk = self.shrink_failure(g, t);

                    // If we couldn't witness a failure on any shrunk value,
                    // then return the failure we already have.
                    return Some(shrunk.unwrap_or(r))
                }
            }
            None
        }
    }
    impl<Args, Return> Testable for EqualFunTestable<Args, Return>
    where
        Args: Arbitrary + std::fmt::Debug + 'static,
        Return: PartialEq + std::fmt::Debug + 'static,
    {
        fn result(&self, g: &mut Gen) -> TestResult {
            let args: Args = Arbitrary::arbitrary(g);
            let r = self.check(args.clone()).result(g);
            if r.is_failure() {
                return self.shrink_failure(g, args).unwrap_or(r);
            }
            r
        }
    }

    fn check_fun_equality(f: Fun<usize, usize>, g: Fun<usize, usize>) {
        ::quickcheck::quickcheck(EqualFunTestable { f, g });
    }

    #[quickcheck]
    fn test_add(a: usize, b: usize, c: usize) {
        check_identity::<Add>(a);
        check_associative::<Add>(a, b, c);
    }

    #[quickcheck]
    fn test_mul(a: usize, b: usize, c: usize) {
        check_identity::<Mul>(a);
        check_associative::<Mul>(a, b, c);
    }

    #[quickcheck]
    fn test_vec_append(a: Vec<usize>, b: Vec<usize>, c: Vec<usize>) {
        check_identity::<VecAppend<usize>>(a.clone());
        check_associative::<VecAppend<usize>>(a, b, c);
    }

    #[quickcheck]
    fn test_string_append(a: String, b: String, c: String) {
        check_identity::<StringAppend>(a.clone());
        check_associative::<StringAppend>(a, b, c);
    }

    #[quickcheck]
    fn test_tuple(a: usize, b: usize, c: usize, d: usize, e: usize, f: usize) {
        check_identity::<(Add, Mul)>((a, b));
        check_associative::<(Add, Mul)>((a, b), (c, d), (e, f));
    }

    #[quickcheck]
    fn test_tuple3(a: usize, b: usize, c: usize, x: Vec<usize>, y: Vec<usize>, z: Vec<usize>) {
        check_identity::<(Add, Mul, VecAppend<usize>)>((a, a, x.clone()));
        check_associative::<(Add, Mul, VecAppend<usize>)>(
            (a, a, x.clone()),
            (b, b, y.clone()),
            (c, c, z.clone()),
        );
    }

    #[test]
    fn test_traverse_fn() {
        type M = TraverseFn<usize, Add>;
        // saturating so we don't have to worry about overflows
        let a = Rc::new(|x: usize| x.saturating_add(1));
        let b = Rc::new(|x: usize| x.saturating_mul(5));
        let c = Rc::new(|x: usize| x.saturating_mul(x));

        // checking by hand because we can't really check functions for equality
        let x = M::append(a.clone(), M::identity());
        let y = M::append(M::identity(), a.clone());
        check_fun_equality(x, y);

        // checking by hand because we can't really check functions for equality
        let x = M::append(M::append(a.clone(), b.clone()), c.clone());
        let y = M::append(a.clone(), M::append(b.clone(), c.clone()));
        check_fun_equality(x, y)
    }

    #[test]
    fn test_compose_endo() {
        type M = ComposeEndomorphism<usize>;
        // saturating so we don't have to worry about overflows
        let a = Rc::new(|x: usize| x.saturating_add(1));
        let b = Rc::new(|x: usize| x.saturating_mul(5));
        let c = Rc::new(|x: usize| x.saturating_mul(x));

        // checking by hand because we can't really check functions for equality
        let x = M::append(a.clone(), M::identity());
        let y = M::append(M::identity(), a.clone());
        check_fun_equality(x, y);

        // checking by hand because we can't really check functions for equality
        let x = M::append(M::append(a.clone(), b.clone()), c.clone());
        let y = M::append(a.clone(), M::append(b.clone(), c.clone()));
        check_fun_equality(x, y)
    }

    #[quickcheck]
    fn test_option(a: Option<usize>, b: Option<usize>, c: Option<usize>) {
        check_identity::<Option<Max>>(a.clone());
        check_associative::<Option<Max>>(a, b, c);
    }
}
