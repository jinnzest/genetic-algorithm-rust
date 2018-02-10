extern crate genetic_algorithm;
extern crate conv;

use genetic_algorithm::gen::Gen;
use genetic_algorithm::zygote::Zygote;
use genetic_algorithm::random_utils::{RandomUtils, ChoosingProbability};
use genetic_algorithm::fitness_calculator::FitnessCalculator;
use genetic_algorithm::breeding::BreedingStruct;
use genetic_algorithm::chromosome::Chromosome;
use std::str::FromStr;
use genetic_algorithm::incubator::Incubator;
use genetic_algorithm::global_constants::*;
use conv::ApproxFrom;

const CHROMOSOMES_AMOUNT: usize = 5;
static mut MUTATED_CHROMOSOMES: usize = 0;
static mut POS: usize = 0;
static mut GEN_FROM: Gen = Gen::D0;
static mut GEN_TO: Gen = Gen::D0;
static mut SIGN: f64 = 1.0;
const CHROMOSOME_GENES_AMOUNT: usize = 3 * U64_BITS_AMOUNT;

#[test]
fn breeding_new_generation_should_replace_all_genes_by_defined_ones_during_a_few_generations() {
    unsafe {
        GEN_FROM = Gen::D0;
        GEN_TO = Gen::D1;
        MUTATED_CHROMOSOMES = 0;
    };
    assert_eq!(run_generations(), 230);
    unsafe {
        GEN_FROM = Gen::R0;
        GEN_TO = Gen::R1;
        MUTATED_CHROMOSOMES = 0;
    };
    assert_eq!(run_generations(), 230);
    unsafe {
        GEN_FROM = Gen::R0;
        GEN_TO = Gen::D1;
        MUTATED_CHROMOSOMES = 0;
    };
    assert_eq!(run_generations(), 230);
    unsafe {
        GEN_FROM = Gen::R1;
        GEN_TO = Gen::D0;
        SIGN = -1.0;
        MUTATED_CHROMOSOMES = 0;
    };
    assert_eq!(run_generations(), 230);
}

pub struct RandomUtilsMock;


pub struct ChoosingProbabilityMock;

impl ChoosingProbability for ChoosingProbabilityMock {
    fn select_individual_probability(fitness: f64) -> bool {
        fitness >= 0.5f64
    }
}


impl RandomUtils for RandomUtilsMock {
    fn mutation_pos() -> usize {
        unsafe {
            if MUTATED_CHROMOSOMES == CHROMOSOMES_AMOUNT {
                MUTATED_CHROMOSOMES = 0;
                if POS == (CHROMOSOME_GENES_AMOUNT - 1) {
                    POS = 0;
                } else {
                    POS += 1;
                }
            } else {
                MUTATED_CHROMOSOMES += 1;
            }
            POS
        }
    }

    fn crossing_chromosome_pos() -> usize {
        CHROMOSOME_GENES_AMOUNT / 2
    }

    fn crossing_zygote_pos() -> usize {
        unimplemented!()
    }

    fn should_cross_zygotes() -> bool {
        false
    }

    fn should_mutate() -> bool {
        true
    }

    fn rand_gen() -> Gen {
        unsafe { GEN_TO.clone() }
    }

    fn generate_zygote() -> Zygote {
        unsafe {
            let s = (0..CHROMOSOME_GENES_AMOUNT)
                .map(|_| GEN_FROM.to_char())
                .collect::<String>();
            Zygote::from_str(&s).unwrap()
        }
    }
}

pub struct FitnessCalculatorStruct;

impl FitnessCalculator for FitnessCalculatorStruct {
    fn calc_fitness(decoded_genotype: &[u64]) -> f64 {
        let sum = decoded_genotype.iter().map(|l| f64::approx_from(*l)).fold(
            0.0,
            |acc, d| acc + d.unwrap(),
        );
        unsafe { sum * SIGN }
    }
}

fn all_chromosomes_are_degenerated(chromosomes: &[Chromosome]) -> bool {
    chromosomes.iter().all(|chr| {
        let chr_str = format!("{}", chr);
        let mut chr_str = chr_str.split('\n').next().unwrap().chars().filter(|c| {
            !c.is_whitespace()
        });
        unsafe { chr_str.all(|c| c == GEN_TO.to_char()) }
    })
}

fn run_generations() -> usize {
    let mut incubator: Incubator<
        RandomUtilsMock,
        ChoosingProbabilityMock,
        BreedingStruct<RandomUtilsMock>,
        FitnessCalculatorStruct,
    > = Incubator::new(CHROMOSOMES_AMOUNT);

    let mut gen_count: usize = 0;
    while !all_chromosomes_are_degenerated(&incubator.get_chromosomes()) {
        incubator.make_next_generation();
        gen_count += 1;
    }
    println!("{}", gen_count);
    gen_count
}
