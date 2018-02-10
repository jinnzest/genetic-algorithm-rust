use std::marker::PhantomData;

use crate::{individual::Individual, random_utils::ChoosingProbability, utils::normalize_fitness};

#[derive(Clone)]
pub struct Parents {
    pub first: Individual,
    pub second: Individual,
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
        let mut parents = Vec::new();
        let mut pos = 0;
        while parents.len() < self.individuals.len() {
            let first_parent_pos = self.find_parent_pos(&mut pos, None);
            let second_parent_pos = self.find_parent_pos(&mut pos, Some(first_parent_pos));
            parents.push(Parents {
                first: self.individuals[first_parent_pos].clone(),
                second: self.individuals[second_parent_pos].clone(),
            });
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
            if Some(*pos) == skip_pos {
                *pos += 1;
            }
            if *pos < self.individuals.len() - 1 {
                *pos += 1;
            } else {
                *pos = 0;
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
