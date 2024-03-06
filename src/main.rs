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
pub mod utils;
pub mod zygote;

use fitness_calculator::*;
use global_constants::*;
use random_utils::*;
use std::rc::Rc;
use std::time::{SystemTime, UNIX_EPOCH};
use utils::*;

use crate::breeding::make_breeding;
use crate::incubator::Incubator;

impl RandomParams for RandomParamsStruct {
    fn chromosome_genes_amount(&self) -> usize {
        20 * U64_BITS_AMOUNT
    }
}

impl FitnessCalculator for FitnessCalculatorStruct {
    fn calc_fitness(&self, decoded_genotype: &[bool]) -> f64 {
        let u64s = decode_bools_to_u64s(decoded_genotype);
        let bits = u64s.iter().fold(0u64, |acc, v| acc | v);
        if bits == 0 { 0f64 } else { 1f64 }
    }
}

fn main() {
    println!("starting...");
    let chromosomes_amount = 1000;
    let random_params = Rc::new(RandomParamsStruct);
    let random_utils: Rc<dyn RandomUtils> = random_utils::make_random_utils(random_params);
    let fitness_calculaltor: Rc<dyn FitnessCalculator> = Rc::new(FitnessCalculatorStruct);
    let perf_choosing_probability: Rc<dyn ChoosingProbability> = Rc::new(PerfChoosingProbability);

    let mut incubator = Incubator::new(
        chromosomes_amount,
        make_breeding(random_utils.clone()).clone(),
        fitness_calculaltor.clone(),
        perf_choosing_probability.clone(),
    );

    let duration = run_and_measure(|| {
        for _ in 0..100_000 {
            incubator.make_next_generation();
        }
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
