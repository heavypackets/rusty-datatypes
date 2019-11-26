#![feature(custom_attribute)]
#![feature(try_from)]

extern crate rusty_dt;

#[derive(Debug, Clone, Copy)]
#[newtype(derive = "Add")]
struct Age(u8);

fn print_u8(v: u8) {
    println!("{:?}", v);
}

fn print_age(age: Age) {
    println!("{:?}", age);
}

fn main() {
    let tom = Age::from(18);
    print_age(tom);
    print_age(12.into());

    let sam = Age::from(90);

    // Easily deref to base type T
    print_u8(*tom);

    // Because we can Age into u8, we can use type inference
    print_u8(sam.into());
    print_u8((tom + sam).into());
}

// ----------------------- Generated --------------------------
impl rusty_dt::Singleton<u8> for Age {}

impl From<u8> for Age {
    fn from(v: u8) -> Age {
        Age(v)
    }
}

impl rusty_dt::Newtype<u8> for Age {}

impl std::ops::Deref for Age {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Into<u8> for Age {
    fn into(self) -> u8 {
        self.0
    }
}

// Derived & Promoted Traits
impl rusty_dt::Add<u8> for Age {}

impl std::ops::Add<Age> for Age {
    type Output = Age;

    fn add(self, other: Self) -> Self {
        Age::from(*self + *other)
    }
}
