#![feature(plugin, const_fn)]
#![plugin(quickcheck_macros)]
#[cfg(test)]
extern crate quickcheck;

#[macro_use]
extern crate lazy_static;
extern crate ordered_float;

#[macro_use]
mod utils;
mod sqrt;
mod trig;

pub use self::sqrt::{approximate_sqrt, sqrt};