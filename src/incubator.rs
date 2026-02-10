use std::marker::PhantomData;

use crate::{
    breeding::Breeding,
    chromosome::Chromosome,
    fitness_calculator::FitnessCalculator,
    generation::{
        Generation, find_best_fitness, find_best_individual, find_worst_fitness,
        find_worst_individual,
    },
    individual::Individual,
    random_utils::{ChoosingProbability, RandomUtils},
};

pub struct Incubator<
    RU: RandomUtils,
    CP: ChoosingProbability,
    B: Breeding<RU>,
    FC: FitnessCalculator,
> {
    new_generation: Generation<CP>,
    old_generation: Generation<CP>,
    tmp_chromosome: Chromosome,
    _phantom_b: PhantomData<B>,
    _phantom_fc: PhantomData<FC>,
    _phantom_ru: PhantomData<RU>,
}

impl<RU: RandomUtils, CP: ChoosingProbability, B: Breeding<RU>, FC: FitnessCalculator>
    Incubator<RU, CP, B, FC>
{
    pub fn new(chromosomes_amount: usize) -> Self {
        let individuals = (0..chromosomes_amount)
            .map(|_| Incubator::<RU, CP, B, FC>::generate_individual())
            .collect::<Vec<Individual>>();
        Self {
            new_generation: Generation::new(individuals.clone()),
            old_generation: Generation::new(individuals),
            tmp_chromosome: B::generate_chromosome(),
            _phantom_b: PhantomData,
            _phantom_fc: PhantomData,
            _phantom_ru: PhantomData,
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
            new_individual
                .chromosome
                .overwrite(&self.old_generation.individuals[pair.first].chromosome);
            self.tmp_chromosome
                .overwrite(&self.old_generation.individuals[pair.second].chromosome);
            B::conception(&mut new_individual.chromosome, &self.tmp_chromosome);
            new_individual.fitness = FC::calc_fitness(&new_individual.chromosome.decode_genotype());
        }
    }

    pub fn make_next_generation(&mut self) {
        self.create_individuals();
        std::mem::swap(&mut self.new_generation, &mut self.old_generation);
        self.old_generation.max_fitness = find_best_fitness(&self.old_generation.individuals);
        self.old_generation.min_fitness = find_worst_fitness(&self.old_generation.individuals);
    }

    fn generate_individual() -> Individual {
        let chromosome = B::generate_chromosome();
        Individual {
            fitness: FC::calc_fitness(&chromosome.decode_genotype()),
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
