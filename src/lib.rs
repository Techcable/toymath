#![feature(plugin, const_fn, proc_macro, proc_macro_non_items)]
#![plugin(quickcheck_macros)]
#[cfg(test)]
extern crate quickcheck;

#[macro_use]
extern crate lazy_static;
extern crate ordered_float;
extern crate num_traits;
extern crate rug;

#[macro_use]
mod utils;
mod sqrt;
mod trig;
mod pow;
mod log;

pub use self::trig::{sin, sin_cos, cos};
pub use self::sqrt::{approximate_sqrt, sqrt};
pub use self::log::{log, log2, log10, ln};