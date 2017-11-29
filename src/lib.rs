#![allow(dead_code)]
#![cfg_attr(test, feature(plugin))]
#![cfg_attr(test, plugin(quickcheck_macros))]

#[cfg(test)]
extern crate quickcheck;
extern crate rand;
#[macro_use]
extern crate rand_derive;
extern crate comp;
#[macro_use]
extern crate lazy_static;

pub mod breeding;
pub mod chromosome;
pub mod fitness_calculator;
pub mod gen;
pub mod generation;
pub mod global_constants;
pub mod incubator;
pub mod individual;
pub mod random_utils;
pub mod utils;
pub mod zygote;
