#![feature(custom_attribute)]

extern crate rusty_dt;

use rusty_dt::Sigma;

#[derive(Debug, Clone, Copy)]
#[sigma()]
struct Squared(u32, u32);

impl From<u32> for Squared {
    fn from(v: u32) -> Squared {
        Squared(v, v * v)
    }
}

// How do you compare squares?
impl PartialEq<Squared> for Squared {
    fn eq(&self, b: &Squared) -> bool {
        *self.a() == *b.a()
    }
}

// How do you add squares?
impl std::ops::Add for Squared {
    type Output = Squared;

    // S(A + A) -> B
    fn add(self, other: Squared) -> Squared {
        Squared::from(*self + *other)
    }
}

fn main() {
    let n: u32 = std::env::args()
        .skip(1)
        .next()
        .expect("Expected integer argument")
        .parse::<u32>()
        .unwrap();
    let squared = Squared::from(n);

    println!("{:?}", squared);
    println!("{:?}", squared + squared);
}

// ----------------------- Generated --------------------------
impl rusty_dt::Singleton<u32> for Squared {}

impl rusty_dt::Sigma<u32, u32> for Squared {
    fn a(&self) -> &u32 {
        &self.0
    }

    fn b(&self) -> &u32 {
        &self.1
    }
}

impl std::ops::Deref for Squared {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::convert::Into<u32> for Squared {
    fn into(self) -> u32 {
        self.0
    }
}
