use individual::Individual;
use conv::*;
use random_utils::ChoosingProbability;
use std::rc::Rc;
use utils;

#[derive(Clone)]
pub struct Parents {
    pub first: Individual,
    pub second: Individual,
}

pub struct Generation {
    pub individuals: Vec<Individual>,
    pub min_fitness: f64,
    pub max_fitness: f64,
    choosing_probability: Rc<ChoosingProbability>,
}

pub fn make_generation(
    individuals: Vec<Individual>,
    choosing_probability: &Rc<ChoosingProbability>,
) -> Rc<Generation> {
    let overage_fitness = calc_overage_fitness(&individuals);
    let min_fitness = find_worst_fitness(&individuals, overage_fitness);
    let max_fitness = find_best_fitness(&individuals);
    Rc::new(Generation {
        min_fitness,
        max_fitness,
        individuals,
        choosing_probability: Rc::clone(choosing_probability),
    })
}

impl Generation {
    pub fn select_parent_pairs(&self) -> Vec<Parents> {
        let mut parents = Vec::new();
        let mut pos = 0;
        while pos < self.individuals.len() {
            let first_parent_pos = self.find_parent_pos(&mut pos);
            let mut second_parent_pos = self.find_parent_pos(&mut pos);
            if first_parent_pos == second_parent_pos {
                second_parent_pos = if pos > 0 {
                    pos - 1
                } else {
                    self.individuals.len() - 1
                }
            }
            parents.push(Parents {
                first: self.individuals[first_parent_pos].clone(),
                second: self.individuals[second_parent_pos].clone(),
            });
            pos += 1;
        }
        parents
    }

    fn find_parent_pos(&self, pos: &mut usize) -> usize {
        loop {
            let candidate = &self.individuals[*pos];
            if self.choosing_probability.select_individual_probability(
                utils::normalize_fitness(
                    candidate.fitness,
                    self.min_fitness,
                    self.max_fitness,
                ),
            )
            {
                return *pos;
            }
            if *pos < self.individuals.len() {
                *pos += 1;
            } else {
                *pos = 0;
            }
        }
    }
}


pub fn calc_overage_fitness(individuals: &[Individual]) -> f64 {
    let fitnesses_sum: f64 = individuals.iter().map(|i| i.fitness).sum();
    fitnesses_sum / f64::value_from(individuals.len()).unwrap()
}

pub fn find_best_individual(individuals: &[Individual]) -> Individual {
    individuals
        .iter()
        .fold(individuals.first().unwrap(), |acc, i| if acc.fitness >
            i.fitness
        {
            i
        } else {
            acc
        })
        .clone()
}

pub fn find_worst_individual(individuals: &[Individual], overage_fitness: f64) -> Individual {
    individuals
        .iter()
        .fold(individuals.first().unwrap(), |acc, i| {
            if acc.fitness > i.fitness && filter_out_unviable_fetus(i, overage_fitness) {
                i
            } else {
                acc
            }
        })
        .clone()
}

pub fn find_best_fitness(individuals: &[Individual]) -> f64 {
    individuals
        .iter()
        .fold(individuals.first().unwrap(), |acc, i| if acc.fitness >
            i.fitness
        {
            i
        } else {
            acc
        })
        .fitness
}

fn filter_out_unviable_fetus(i: &Individual, overage_fitness: f64) -> bool {
    let fitness = if i.fitness == 0.0 {
        0.0
    } else {
        overage_fitness / i.fitness
    };
    fitness < 20.0
}

pub fn find_worst_fitness(individuals: &[Individual], overage_fitness: f64) -> f64 {
    individuals
        .iter()
        .fold(individuals.first().unwrap(), |acc, i| {
            if acc.fitness > i.fitness && filter_out_unviable_fetus(i, overage_fitness) {
                i
            } else {
                acc
            }
        })
        .fitness
}
