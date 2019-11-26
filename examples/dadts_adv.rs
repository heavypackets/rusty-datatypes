#![feature(custom_attribute)]
#![feature(try_from)]

extern crate rusty_dt;
extern crate uuid;

use rusty_dt::Singleton;
use uuid::Uuid;

type Raw = f64;

#[derive(Debug)]
struct Cash(f64);

#[derive(Debug)]
struct Online();

#[derive(Debug)]
struct CreditCard();

#[derive(Debug)]
#[singleton(deref = "transaction")]
struct Receipt<T> {
    id: String,
    transaction: T,
}

// dadts!{
//     Receipt<T> {
//         Raw, // => () -- We cannot prove if Raw reciept if refundable or not
//         Cash => NoneRefundable,
//         Online | CreditCard => noimpl: Refundable, // noimpl alplows dev to specify advanced traits and impl manually
//     }
// }

trait Refundable {
    fn refund(self) -> f64;
}

impl Refundable for Receipt<CreditCard> {
    fn refund(self) -> f64 {
        0.0
    }
}

impl Refundable for Receipt<Online> {
    fn refund(self) -> f64 {
        0.0
    }
}

fn generate_uuid() -> String {
    Uuid::new_v4().to_string()
}

// If these are refutable, you must use singleton_p and dadts_p
impl rusty_dt::Singleton<Raw> for Receipt<Raw> {
    fn p(n: Raw) -> Result<Self, Raw> {
        Ok(Receipt::<Raw> {
            id: generate_uuid(),
            transaction: n,
        })
    }
}

impl rusty_dt::Singleton<CreditCard> for Receipt<CreditCard> {
    fn p(n: CreditCard) -> Result<Self, CreditCard> {
        Ok(Receipt::<CreditCard> {
            id: generate_uuid(),
            transaction: n,
        })
    }
}

impl rusty_dt::Singleton<Cash> for Receipt<Cash> {
    fn p(n: Cash) -> Result<Self, Cash> {
        Ok(Receipt::<Cash> {
            id: generate_uuid(),
            transaction: n,
        })
    }
}

impl rusty_dt::Singleton<Online> for Receipt<Online> {
    fn p(n: Online) -> Result<Self, Online> {
        Ok(Receipt::<Online> {
            id: generate_uuid(),
            transaction: n,
        })
    }
}

/* Demonstrate how we can do:
 * - type level matches (impl specialization based on type constructor)
 * - trait matches, either in where clauses or type constraints, based on triats generated by macro
 * - */

fn only_redundable<R: Refundable + std::fmt::Debug>(r: R) {
    println!("{:?}", r);
}

fn main() {
    let trans1: Raw = 6.00;
    let r = Receipt::from(trans1); // Type inference is good!
    // only_redundable(r);

    let trans2 = 6;
    //let r = Receipt::from(trans2);

    let trans3 = CreditCard();
    let r = Receipt::from(trans3);
    only_redundable(r);
}

// ----------------------- Generated --------------------------
// You can implement specialized deref for specific variants if need be
impl<T> std::ops::Deref for Receipt<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.transaction
    }
}

// Trait Variants
trait NonRefundable {}

// Kinds
enum ReceiptKind {
    Raw(Receipt<Raw>),
    Cash(Receipt<Cash>),
    Online(Receipt<Online>),
    CreditCard(Receipt<CreditCard>),
}

impl std::convert::From<Receipt<Raw>> for ReceiptKind {
    fn from(value: Receipt<Raw>) -> ReceiptKind {
        ReceiptKind::Raw(value)
    }
}

impl std::convert::From<Receipt<Cash>> for ReceiptKind {
    fn from(value: Receipt<Cash>) -> ReceiptKind {
        ReceiptKind::Cash(value)
    }
}

impl std::convert::From<Receipt<Online>> for ReceiptKind {
    fn from(value: Receipt<Online>) -> ReceiptKind {
        ReceiptKind::Online(value)
    }
}

impl std::convert::From<Receipt<CreditCard>> for ReceiptKind {
    fn from(value: Receipt<CreditCard>) -> ReceiptKind {
        ReceiptKind::CreditCard(value)
    }
}

impl std::convert::TryInto<Receipt<Raw>> for ReceiptKind {
    type Error = ReceiptKind;

    fn try_into(self) -> Result<Receipt<Raw>, ReceiptKind> {
        match self {
            ReceiptKind::Raw(v) => Ok(v),
            _ => Err(self),
        }
    }
}

impl std::convert::TryInto<Receipt<Cash>> for ReceiptKind {
    type Error = ReceiptKind;

    fn try_into(self) -> Result<Receipt<Cash>, ReceiptKind> {
        match self {
            ReceiptKind::Cash(v) => Ok(v),
            _ => Err(self),
        }
    }
}

impl std::convert::TryInto<Receipt<Online>> for ReceiptKind {
    type Error = ReceiptKind;

    fn try_into(self) -> Result<Receipt<Online>, ReceiptKind> {
        match self {
            ReceiptKind::Online(v) => Ok(v),
            _ => Err(self),
        }
    }
}

impl std::convert::TryInto<Receipt<CreditCard>> for ReceiptKind {
    type Error = ReceiptKind;

    fn try_into(self) -> Result<Receipt<CreditCard>, ReceiptKind> {
        match self {
            ReceiptKind::CreditCard(v) => Ok(v),
            _ => Err(self),
        }
    }
}

// Variant implementations
// Reciept<Raw>
impl From<Raw> for Receipt<Raw> {
    fn from(v: Raw) -> Receipt<Raw> {
        match Self::p(v) {
            Ok(s) => s,
            Err(p) => panic!(format!("Error value encountered: {:?}", p)),
        }
    }
}

impl Into<Raw> for Receipt<Raw> {
    fn into(self) -> Raw {
        self.transaction
    }
}

// Receipt<CreditCard>
// impl Refundable for Receipt<CreditCard> {}

impl From<CreditCard> for Receipt<CreditCard> {
    fn from(v: CreditCard) -> Receipt<CreditCard> {
        match Self::p(v) {
            Ok(s) => s,
            Err(p) => panic!(format!("Error value encountered: {:?}", p)),
        }
    }
}

impl Into<CreditCard> for Receipt<CreditCard> {
    fn into(self) -> CreditCard {
        self.transaction
    }
}

// Receipt<Online>
// impl Refundable for Receipt<Online> {}

impl From<Online> for Receipt<Online> {
    fn from(v: Online) -> Receipt<Online> {
        match Self::p(v) {
            Ok(s) => s,
            Err(p) => panic!(format!("Error value encountered: {:?}", p)),
        }
    }
}

impl Into<Online> for Receipt<Online> {
    fn into(self) -> Online {
        self.transaction
    }
}

// Receipt<Cash>
impl NonRefundable for Receipt<Cash> {}

impl From<Cash> for Receipt<Cash> {
    fn from(v: Cash) -> Receipt<Cash> {
        match Self::p(v) {
            Ok(s) => s,
            Err(p) => panic!(format!("Error value encountered: {:?}", p)),
        }
    }
}

impl Into<Cash> for Receipt<Cash> {
    fn into(self) -> Cash {
        self.transaction
    }
}