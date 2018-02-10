pub trait FitnessCalculator {
    fn calc_fitness(decoded_genotype: &[u64]) -> f64;
}

pub struct FitnessCalculatorStruct;
