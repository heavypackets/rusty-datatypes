#![feature(try_from)]

pub mod p;

pub struct Proven<T>(T);
pub struct Unproven<T>(T);
pub struct Refuted<T>(T);

pub trait Singleton<I>: std::convert::From<I> + Into<I> + std::ops::Deref {}

pub trait Sigma<A, B>: Singleton<A> {
    fn a(&self) -> &A;
    fn b(&self) -> &B;
}

pub trait PartialEqA<A, B>: std::cmp::PartialEq<Self>
where
    Self: Sigma<A, B>,
    A: std::cmp::PartialEq<A>,
{
    // fn eq(&self, other: &Self) -> bool {
    //     *self.a() == *other.a()
    // }
}

pub trait PartialEqB<A, B>: std::cmp::PartialEq<Self>
where
    Self: Sigma<A, B>,
    B: std::cmp::PartialEq<B>,
{
    // fn eq(&self, other: &Self) -> bool {
    //     *self.b() == *other.b()
    // }
}

pub trait Add<A>: std::ops::Add<Self>
where
    Self: Singleton<A>,
    A: std::ops::Add<A>,
{
    // fn add(self, other: Self) -> Self {
    //     use std::convert::TryFrom;

    //     match Self::try_from(*self.a() + *other.a()) {
    //         Ok(s) => s,
    //         Err(p) => panic!(format!("Result of add operation refuted: {:?}", p)),
    //     }
    // }

    // fn add(self, other: Self) -> Self {
    //     Self::from(*self + *other)
    // }
}

pub trait Newtype<I>: Singleton<I> {}

pub trait PartialEq<T>: std::cmp::PartialEq<Self>
where
    Self: Newtype<T>,
    T: std::cmp::PartialEq<T>,
{
    // fn eq(&self, other: &Self) -> bool {
    //     *self == *other
    // }
}
