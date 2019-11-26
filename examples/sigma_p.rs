#![feature(try_from)]
#![feature(custom_attribute)]

extern crate rusty_dt;

use std::convert::TryFrom;

#[derive(Debug)]
#[sigma_p(derive = "PartialEqA, Add")]
struct CalendarMonth(u32, u8);

impl std::convert::TryFrom<u32> for CalendarMonth {
    type Error = u32;

    fn try_from(day: u32) -> std::result::Result<CalendarMonth, Self::Error> {
        let m = (day / 31) + 1;
        if m <= 12 {
            if let Ok(m) = u8::try_from(m) {
                return Ok(CalendarMonth(day, m));
            }
        }

        Err(day)
    }
}

fn main() {
    let day: u32 = std::env::args()
        .skip(1)
        .next()
        .unwrap()
        .parse::<u32>()
        .unwrap();
    if let Ok(month) = CalendarMonth::try_from(day) {
        println!("{:?}", month);
        let one_month = CalendarMonth::try_from(30).unwrap();
        let r = month + one_month;

        println!("{:?}", r);
    } else {
        eprintln!("Not a valid day in the calendar year")
    }
}

// ----------------------- Generated --------------------------
impl rusty_dt::p::Singleton<u32> for CalendarMonth {}

impl rusty_dt::p::Sigma<u32, u8> for CalendarMonth {
    fn a(&self) -> &u32 {
        &self.0
    }

    fn b(&self) -> &u8 {
        &self.1
    }
}

impl std::ops::Deref for CalendarMonth {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::convert::Into<u32> for CalendarMonth {
    fn into(self) -> u32 {
        self.0
    }
}

// Derived & Promoted Traits
impl rusty_dt::p::PartialEqA<u32, u8> for CalendarMonth {}

impl std::cmp::PartialEq<Self> for CalendarMonth {
    fn eq(&self, other: &Self) -> bool {
        use rusty_dt::p::Sigma;

        *self.a() == *other.a()
    }
}

impl rusty_dt::p::Add<u32> for CalendarMonth {}

impl std::ops::Add for CalendarMonth {
    type Output = CalendarMonth;

    fn add(self, other: Self) -> Self {
        use rusty_dt::p::Sigma;
        
        use std::convert::TryFrom;

        match Self::try_from(*self.a() + *other.a()) {
            Ok(s) => s,
            Err(p) => panic!(format!("Result of add operation refuted: {:?}", p)),
        }
    }
}
