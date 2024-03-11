#[cfg(test)]
extern crate quickcheck;

extern crate rand;
extern crate rand_distr;

pub mod breeding;
pub mod chromosome;
pub mod fitness_calculator;
pub mod gene;
pub mod generation;
pub mod global_constants;
pub mod incubator;
pub mod individual;
pub mod random_utils;
pub mod u64s;
pub mod utils;
pub mod zygote;
