#![feature(plugin)]
#![plugin(quickcheck_macros)]
extern crate quickcheck;

mod sqrt;
mod utils;

pub use self::sqrt::sqrt;