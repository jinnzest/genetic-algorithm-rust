use generation::Generation;
use individual::Individual;
use breeding::Breeding;
use random_utils::{RandomUtils, ChoosingProbability};
use fitness_calculator::FitnessCalculator;
use generation;
use chromosome::Chromosome;
use generation::Parents;
use std::rc::Rc;

pub struct Incubator {
    generation: Rc<Generation>,
    breeding: Rc<Breeding>,
    fitness_calculator: Rc<FitnessCalculator>,
    random_utils: Rc<RandomUtils>,
    choosing_probability: Rc<ChoosingProbability>,
}

pub fn make_incubator(
    chromosomes_amount: usize,
    breeding: &Rc<Breeding>,
    fitness_calculator: &Rc<FitnessCalculator>,
    random_utils: &Rc<RandomUtils>,
    choosing_probability: &Rc<ChoosingProbability>,
) -> Incubator {
    let individuals = (0..chromosomes_amount)
        .map(|_| {
            Incubator::generate_individual(breeding, fitness_calculator)
        })
        .collect::<Vec<Individual>>();
    Incubator {
        generation: generation::make_generation(individuals, choosing_probability),
        breeding: Rc::clone(breeding),
        fitness_calculator: Rc::clone(fitness_calculator),
        random_utils: Rc::clone(random_utils),
        choosing_probability: Rc::clone(choosing_probability),
    }
}

impl Incubator {
    pub fn get_best_individual(&self) -> Individual {
        generation::find_best_individual(&self.generation.individuals)
    }

    pub fn get_worst_individual(&self) -> Individual {
        generation::find_worst_individual(
            &self.generation.individuals,
            generation::calc_overage_fitness(&self.generation.individuals),
        )
    }

    pub fn create_individuals(&mut self) -> Vec<Individual> {
        self.generation
            .select_parent_pairs()
            .iter()
            .map(|&Parents {
                 ref first,
                 ref second,
             }| {
                let new_chromosome = self.breeding.conception(
                    &first.chromosome,
                    &second.chromosome,
                );
                Individual {
                    fitness: self.fitness_calculator.calc_fitness(
                        &new_chromosome.decode_genotype(),
                    ),
                    chromosome: new_chromosome,
                }
            })
            .collect()
    }

    pub fn make_next_generation(&mut self) {
        self.generation =
            generation::make_generation(self.create_individuals(), &self.choosing_probability);
    }

    fn generate_individual(
        breeding: &Rc<Breeding>,
        fitness_calculator: &Rc<FitnessCalculator>,
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
