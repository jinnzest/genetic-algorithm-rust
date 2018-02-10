#![allow(dead_code)]
#![cfg_attr(test, feature(plugin))]
#![cfg_attr(test, plugin(quickcheck_macros))]

#[cfg(test)]
extern crate quickcheck;
extern crate rand;
#[macro_use]
extern crate rand_derive;
#[macro_use]
extern crate comp;
#[macro_use]
extern crate lazy_static;
extern crate conv;

pub mod global_constants;
pub mod utils;
pub mod gen;
pub mod u64s;
pub mod zygote;
pub mod random_utils;
pub mod chromosome;
pub mod individual;
pub mod generation;
pub mod fitness_calculator;
pub mod breeding;
pub mod incubator;
