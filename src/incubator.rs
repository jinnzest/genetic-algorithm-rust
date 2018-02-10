use generation::Generation;
use individual::Individual;
use breeding::Breeding;
use random_utils::{RandomUtils, ChoosingProbability};
use fitness_calculator::FitnessCalculator;
use std::marker::PhantomData;
use generation;
use chromosome::Chromosome;
use generation::Parents;

pub struct Incubator<RU: RandomUtils, CP: ChoosingProbability, B: Breeding<RU>, FC: FitnessCalculator> {
    generation: Generation<CP>,
    _phantom_b: PhantomData<B>,
    _phantom_fc: PhantomData<FC>,
    _phantom_ru: PhantomData<RU>,
}

impl<RU: RandomUtils, CP: ChoosingProbability, B: Breeding<RU>, FC: FitnessCalculator>
    Incubator<RU, CP, B, FC> {
    pub fn new(chromosomes_amount: usize) -> Self {
        let individuals = (0..chromosomes_amount)
            .map(|_| Incubator::<RU, CP, B, FC>::generate_individual())
            .collect::<Vec<Individual>>();
        Self {
            generation: Generation::new(individuals),
            _phantom_b: PhantomData,
            _phantom_fc: PhantomData,
            _phantom_ru: PhantomData,
        }
    }

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
                let new_chromosome = B::conception(&first.chromosome, &second.chromosome);
                Individual {
                    fitness: FC::calc_fitness(&new_chromosome.decode_genotype()),
                    chromosome: new_chromosome,
                }
            })
            .collect()
    }

    pub fn make_next_generation(&mut self) {
        self.generation = Generation::new(self.create_individuals());
    }

    fn generate_individual() -> Individual {
        let chromosome = B::generate_chromosome();
        Individual {
            fitness: FC::calc_fitness(&chromosome.decode_genotype()),
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
