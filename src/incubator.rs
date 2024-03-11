use std::rc::Rc;

use crate::{
    breeding::Breeding,
    chromosome::Chromosome,
    fitness_calculator::FitnessCalculator,
    generation::{
        Generation, find_best_fitness, find_best_individual, find_worst_fitness,
        find_worst_individual,
    },
    individual::Individual,
    random_utils::ChoosingProbability,
};

pub struct Incubator {
    breeding: Rc<dyn Breeding>,
    fitness_calculator: Rc<dyn FitnessCalculator>,
    new_generation: Generation,
    old_generation: Generation,
    tmp_chromosome: Chromosome,
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
            breeding: breeding.clone(),
            fitness_calculator: fitness_calculator.clone(),
            new_generation: Generation::new(individuals.clone(), choosing_probability.clone()),
            old_generation: Generation::new(individuals, choosing_probability.clone()),
            tmp_chromosome: breeding.generate_chromosome(),
        }
    }
    pub fn get_best_individual(&self) -> &Individual {
        find_best_individual(&self.new_generation.individuals)
    }

    pub fn get_worst_individual(&self) -> &Individual {
        find_worst_individual(&self.new_generation.individuals)
    }

    pub fn create_individuals(&mut self) {
        let parents = self.old_generation.select_parent_pairs();

        for (pos, pair) in parents.iter().enumerate() {
            let new_individual = &mut self.new_generation.individuals[pos];
            new_individual.chromosome.overwrite(&pair.first.chromosome);
            self.tmp_chromosome.overwrite(&pair.second.chromosome);
            self.breeding
                .conception(&mut new_individual.chromosome, &self.tmp_chromosome);
            new_individual.fitness = self
                .fitness_calculator
                .calc_fitness(&new_individual.chromosome.decode_genotype());
        }
    }

    pub fn make_next_generation(&mut self) {
        self.create_individuals();
        std::mem::swap(&mut self.new_generation, &mut self.old_generation);
        self.old_generation.max_fitness = find_best_fitness(&self.old_generation.individuals);
        self.old_generation.min_fitness = find_worst_fitness(&self.old_generation.individuals);
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
        self.old_generation
            .individuals
            .iter()
            .map(|i| i.chromosome.clone())
            .collect()
    }
}
