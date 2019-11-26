#![feature(custom_attribute)]

extern crate rusty_dt;

#[derive(Debug, Clone)]
#[sigma(derive = "PartialEqA")]
struct Count(u32, Vec<u32>);

impl From<u32> for Count {
    fn from(n: u32) -> Count {
        Count(n, (0..=n).collect::<Vec<u32>>())
    }
}

fn main() {
    let n: u32 = std::env::args()
        .skip(1)
        .next()
        .expect("Expected integer argument")
        .parse::<u32>()
        .unwrap();
    let count = Count::from(n);

    println!("{:?}", count);
}

// ----------------------- Generated --------------------------
impl rusty_dt::Singleton<u32> for Count {}

impl rusty_dt::Sigma<u32, Vec<u32>> for Count {
    fn a(&self) -> &u32 {
        &self.0
    }

    fn b(&self) -> &Vec<u32> {
        &self.1
    }
}

impl std::ops::Deref for Count {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::convert::Into<u32> for Count {
    fn into(self) -> u32 {
        self.0
    }
}

impl rusty_dt::PartialEqA<u32, Vec<u32>> for Count {}

impl std::cmp::PartialEq<Self> for Count
{
    fn eq(&self, other: &Self) -> bool {
        use rusty_dt::Sigma;
        
        *self.a() == *other.a()
    }
}
