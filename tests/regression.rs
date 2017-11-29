extern crate genetic_algorithm;

use genetic_algorithm::gen::Gen;
use genetic_algorithm::zygote::Zygote;
use genetic_algorithm::random_utils::{RandomUtils, ChoosingProbability};
use genetic_algorithm::fitness_calculator::FitnessCalculator;
use genetic_algorithm::chromosome::Chromosome;
use std::str::FromStr;
use genetic_algorithm::global_constants::*;
use genetic_algorithm::incubator;
use genetic_algorithm::breeding;
use std::rc::Rc;

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
    fn select_individual_probability(&self, fitness: f64) -> bool {
        fitness >= 0.5f64
    }
}


impl RandomUtils for RandomUtilsMock {
    fn mutation_pos(&self) -> usize {
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

    fn crossing_chromosome_pos(&self) -> usize {
        CHROMOSOME_GENES_AMOUNT / 2
    }

    fn crossing_zygote_pos(&self) -> usize {
        unimplemented!()
    }

    fn should_cross_zygotes(&self) -> bool {
        false
    }

    fn should_mutate(&self) -> bool {
        true
    }

    fn rand_gen(&self) -> Gen {
        unsafe { GEN_TO.clone() }
    }

    fn generate_zygote(&self) -> Zygote {
        unsafe {
            let s = (0..CHROMOSOME_GENES_AMOUNT)
                .map(|_| GEN_FROM.to_char())
                .collect::<String>();
            Zygote::from_str(&s).unwrap()
        }
    }
}

pub struct FitnessCalculatorMock;

impl FitnessCalculator for FitnessCalculatorMock {
    fn calc_fitness(&self, decoded_genotype: &[bool]) -> f64 {
        let sum = decoded_genotype.iter().fold(0.0, |acc, &b| {
            acc + (if b { 1.0 } else { 0.0 })
        });
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
    let random_utils: Rc<RandomUtils> = Rc::new(RandomUtilsMock);
    let fitness_calculator: Rc<FitnessCalculator> = Rc::new(FitnessCalculatorMock);
    let rc: Rc<ChoosingProbability> = Rc::new(ChoosingProbabilityMock);

    let mut incubator = incubator::make_incubator(
        CHROMOSOMES_AMOUNT,
        &breeding::make_breeding(Rc::clone(&random_utils)),
        &fitness_calculator,
        &random_utils,
        &rc,
    );

    let mut gen_count: usize = 0;
    while !all_chromosomes_are_degenerated(&incubator.get_chromosomes()) {
        incubator.make_next_generation();
        gen_count += 1;
    }
    println!("{}", gen_count);
    gen_count
}
