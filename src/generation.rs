use std::marker::PhantomData;

use crate::{individual::Individual, random_utils::ChoosingProbability, utils::normalize_fitness};

#[derive(Clone)]
pub struct Parents {
    pub first: usize,
    pub second: usize,
}

pub struct Generation<CP: ChoosingProbability> {
    pub individuals: Vec<Individual>,
    pub min_fitness: f64,
    pub max_fitness: f64,
    phantom: PhantomData<CP>,
}

impl<CP: ChoosingProbability> Generation<CP> {
    pub fn new(individuals: Vec<Individual>) -> Self {
        let min_fitness = find_worst_fitness(&individuals);
        let max_fitness = find_best_fitness(&individuals);
        Self {
            min_fitness,
            max_fitness,
            individuals,
            phantom: PhantomData,
        }
    }

    pub fn select_parent_pairs(&self) -> Vec<Parents> {
        let mut parents = Vec::with_capacity(self.individuals.len());
        let mut pos = 0;
        while parents.len() < self.individuals.len() {
            let first = self.find_parent_pos(&mut pos, None);
            let second = self.find_parent_pos(&mut pos, Some(first));
            parents.push(Parents { first, second });
        }
        parents
    }

    fn find_parent_pos(&self, pos: &mut usize, skip_pos: Option<usize>) -> usize {
        loop {
            let candidate = &self.individuals[*pos];
            if CP::select_individual_with_probability(normalize_fitness(
                candidate.fitness,
                self.min_fitness,
                self.max_fitness,
            )) {
                return *pos;
            }
            loop {
                if *pos < self.individuals.len() - 1 {
                    *pos += 1;
                } else {
                    *pos = 0;
                }
                if Some(*pos) != skip_pos {
                    break;
                }
            }
        }
    }
}

pub fn find_individual_by(
    individuals: &[Individual],
    comparator: impl Fn(f64, f64) -> bool,
) -> &Individual {
    individuals
        .iter()
        .fold(individuals.first().unwrap(), |acc, i| {
            if comparator(acc.fitness, i.fitness) {
                i
            } else {
                acc
            }
        })
}

pub fn find_best_individual(individuals: &[Individual]) -> &Individual {
    find_individual_by(individuals, |acc, i| acc < i)
}

pub fn find_worst_individual(individuals: &[Individual]) -> &Individual {
    find_individual_by(individuals, |acc, i| acc > i)
}

pub fn find_best_fitness(individuals: &[Individual]) -> f64 {
    find_individual_by(individuals, |acc, i| acc < i).fitness
}

pub fn find_worst_fitness(individuals: &[Individual]) -> f64 {
    find_individual_by(individuals, |acc, i| acc > i).fitness
}
