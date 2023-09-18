#![doc = include_str!("../README.md")]

pub mod money;

pub use money::{Money, MoneySign, MoneyErrorCents, MoneyErrorString, MoneyErrorOverflow};
pub use money::options::NegativeView;