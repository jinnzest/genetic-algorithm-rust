pub trait FitnessCalculator {
    fn calc_fitness(&self, decoded_genotype: &[bool]) -> f64;
}

pub struct FitnessCalculatorStruct;
