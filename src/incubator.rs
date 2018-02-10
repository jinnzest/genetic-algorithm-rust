use generation::Generation;
use individual::Individual;
use breeding::Breeding;
use random_utils::{RandomUtils, ChoosingProbability};
use fitness_calculator::FitnessCalculator;
use std::marker::PhantomData;
use std;
use generation;
use chromosome::Chromosome;

pub struct Incubator<RU: RandomUtils, CP: ChoosingProbability, B: Breeding<RU>, FC: FitnessCalculator> {
    new_gen: Generation<CP>,
    old_gen: Generation<CP>,
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
            new_gen: Generation::new(individuals.clone()),
            old_gen: Generation::new(individuals),
            _phantom_b: PhantomData,
            _phantom_fc: PhantomData,
            _phantom_ru: PhantomData,
        }
    }

    pub fn get_best_individual(&self) -> Individual {
        generation::find_best_individual(&self.old_gen.individuals)
    }

    pub fn get_worst_individual(&self) -> Individual {
        generation::find_worst_individual(
            &self.old_gen.individuals,
            generation::calc_overage_fitness(&self.old_gen.individuals),
        )
    }

    pub fn create_individuals(&mut self) {
        let individuals = &self.old_gen.individuals;
        let parents = self.old_gen.select_parent_pairs();

        for (pos, pair) in parents.iter().enumerate() {
            let new_individual = &mut self.new_gen.individuals[pos];
            let mut child = &mut new_individual.chromosome;
            let first = &individuals[pair.first_pos].chromosome;
            let second = &individuals[pair.second_pos].chromosome;
            B::conception(first, second, &mut child);
            new_individual.fitness = FC::calc_fitness(&child.decode_genotype());
        }
    }

    pub fn make_next_generation(&mut self) {
        self.create_individuals();
        std::mem::swap(&mut self.new_gen, &mut self.old_gen);
        let overage_fitness = generation::calc_overage_fitness(&self.old_gen.individuals);
        self.old_gen.max_fitness = generation::find_best_fitness(&self.old_gen.individuals);
        self.old_gen.min_fitness =
            generation::find_worst_fitness(&self.old_gen.individuals, overage_fitness);
    }

    fn generate_individual() -> Individual {
        let chromosome = B::generate_chromosome();
        Individual {
            fitness: FC::calc_fitness(&chromosome.decode_genotype()),
            chromosome,
        }
    }

    pub fn get_chromosomes(&mut self) -> Vec<Chromosome> {
        self.old_gen
            .individuals
            .iter()
            .map(|i| i.chromosome.clone())
            .collect()
    }
}
