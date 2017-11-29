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
pub mod zygote;
pub mod random_utils;
pub mod chromosome;
pub mod individual;
pub mod generation;
pub mod fitness_calculator;
pub mod breeding;
pub mod incubator;

use random_utils::*;
use fitness_calculator::*;
use conv::*;
use utils::*;
use std::time::{SystemTime, UNIX_EPOCH};
use global_constants::*;
use std::rc::Rc;

impl RandomParams for RandomParamsStruct {
    fn chromosome_genes_amount(&self) -> usize {
        20 * U64_BITS_AMOUNT
    }
}

impl FitnessCalculator for FitnessCalculatorStruct {
    fn calc_fitness(&self, decoded_genotype: &[bool]) -> f64 {
        let u64s = decode_bools_to_u64s(decoded_genotype);
        u64s.iter().map(|l| f64::approx_from(*l)).fold(
            0.0,
            |acc, d| {
                acc + d.unwrap()
            },
        )
    }
}


fn main() {
    let chromosomes_amount = 1000;
    let random_params = Rc::new(RandomParamsStruct);
    let random_utils: Rc<RandomUtils> = random_utils::make_random_utils(random_params);
    let fitness_calculaltor: Rc<FitnessCalculator> = Rc::new(FitnessCalculatorStruct);
    let perf_choosing_probability: Rc<ChoosingProbability> = Rc::new(PerfChoosingProbability);

    let mut incubator = incubator::make_incubator(
        chromosomes_amount,
        &breeding::make_breeding(Rc::clone(&random_utils)),
        &fitness_calculaltor,
        &random_utils,
        &perf_choosing_probability,
    );

    let duration = run_and_measure(|| for _ in 0..100_000 {
        incubator.make_next_generation();
    });

    println!("exec time = {:?} ms", duration);
}

fn run_and_measure<F>(f: F) -> u64
where
    F: FnOnce(),
{
    let start = get_ms_now();
    f();
    get_ms_now() - start
}

fn get_ms_now() -> u64 {
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH).unwrap();
    since_the_epoch.as_secs() * 1000 + u64::from(since_the_epoch.subsec_nanos()) / 1_000_000
}
