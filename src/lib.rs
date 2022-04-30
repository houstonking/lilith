#![recursion_limit = "512"]
// #![feature(generic_associated_types)]

#[macro_use]
extern crate shrinkwraprs;

pub mod database_snapshot;
pub mod datom;
pub mod pull;
mod indexes;

pub type EntityId = i64;
pub type AttributeId = i64;
pub type TransactionId = i64;

const SIZE: usize = 512;

#[derive(Shrinkwrap, Clone, Debug)]
pub struct Key(pub String);

pub trait Minimum {
    fn minimum() -> Self;
}

pub trait Maximum {
    fn maximum() -> Self;
}

// this works because entity id et al are a type alias, not a newtype.
// these'll need to be updated if that changes.
impl Minimum for i64 {
    fn minimum() -> i64 {
        std::i64::MIN
    }
}

impl Maximum for i64 {
    fn maximum() -> i64 {
        std::i64::MAX
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum V {
    MinimumValue,
    String(String),
    EntityId(EntityId),
    Uuid(uuid::Uuid),
    I64(i64),
    Key,
    MaximumValue,
}

impl Minimum for V {
    fn minimum() -> V {
        V::MinimumValue
    }
}

impl Maximum for V {
    fn maximum() -> V {
        V::MaximumValue
    }
}

pub trait Datom {
    fn e(self) -> EntityId;
    fn a(self) -> AttributeId;
    fn v(self) -> V;
    fn t(self) -> TransactionId;
    fn added(self) -> bool;
}
