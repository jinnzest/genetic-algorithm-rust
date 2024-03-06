use std::rc::Rc;

use crate::{
    breeding::Breeding,
    chromosome::Chromosome,
    fitness_calculator::FitnessCalculator,
    generation::{Generation, Parents, find_best_individual, find_worst_individual},
    individual::Individual,
    random_utils::ChoosingProbability,
};

pub struct Incubator {
    generation: Generation,
    breeding: Rc<dyn Breeding>,
    fitness_calculator: Rc<dyn FitnessCalculator>,
    choosing_probability: Rc<dyn ChoosingProbability>,
}

impl Incubator {
    pub fn new(
        chromosomes_amount: usize,
        breeding: Rc<dyn Breeding>,
        fitness_calculator: Rc<dyn FitnessCalculator>,
        choosing_probability: Rc<dyn ChoosingProbability>,
    ) -> Incubator {
        let individuals = (0..chromosomes_amount)
            .map(|_| Incubator::generate_individual(breeding.clone(), fitness_calculator.clone()))
            .collect::<Vec<Individual>>();
        Incubator {
            generation: Generation::new(individuals, choosing_probability.clone()),
            breeding: breeding.clone(),
            fitness_calculator: fitness_calculator.clone(),
            choosing_probability: choosing_probability.clone(),
        }
    }
    pub fn get_best_individual(&self) -> &Individual {
        find_best_individual(&self.generation.individuals)
    }

    pub fn get_worst_individual(&self) -> &Individual {
        find_worst_individual(&self.generation.individuals)
    }

    pub fn create_individuals(&mut self) -> Vec<Individual> {
        self.generation
            .select_parent_pairs()
            .iter()
            .map(|Parents { first, second }| {
                let new_chromosome = self
                    .breeding
                    .conception(&first.chromosome, &second.chromosome);
                Individual {
                    fitness: self
                        .fitness_calculator
                        .calc_fitness(&new_chromosome.decode_genotype()),
                    chromosome: new_chromosome,
                }
            })
            .collect()
    }

    pub fn make_next_generation(&mut self) {
        self.generation =
            Generation::new(self.create_individuals(), self.choosing_probability.clone());
    }

    fn generate_individual(
        breeding: Rc<dyn Breeding>,
        fitness_calculator: Rc<dyn FitnessCalculator>,
    ) -> Individual {
        let chromosome = breeding.generate_chromosome();
        Individual {
            fitness: fitness_calculator.calc_fitness(&chromosome.decode_genotype()),
            chromosome,
        }
    }

    pub fn get_chromosomes(&mut self) -> Vec<Chromosome> {
        self.generation
            .individuals
            .iter()
            .map(|i| i.chromosome.clone())
            .collect()
    }
}
