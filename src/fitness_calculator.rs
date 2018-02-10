pub trait FitnessCalculator {
    fn calc_fitness(decoded_genotype: &[bool]) -> f64;
}

pub struct FitnessCalculatorStruct;
