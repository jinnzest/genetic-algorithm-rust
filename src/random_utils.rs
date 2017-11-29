use gen::Gen;
use zygote::Zygote;
use rand;
use conv::ValueFrom;
use std::rc::Rc;

pub trait RandomUtils {
    fn mutation_pos(&self) -> usize;

    fn crossing_chromosome_pos(&self) -> usize;

    fn crossing_zygote_pos(&self) -> usize;

    fn should_cross_zygotes(&self) -> bool;

    fn should_mutate(&self) -> bool;

    fn rand_gen(&self) -> Gen;

    fn generate_zygote(&self) -> Zygote;
}

pub struct RandomUtilsStruct {
    random_params: Rc<RandomParams>,
}

pub struct RandomParamsStruct;

pub trait RandomParams {
    fn chromosome_genes_amount(&self) -> usize;
}

pub fn make_random_utils(random_params: Rc<RandomParams>) -> Rc<RandomUtils> {
    Rc::new(RandomUtilsStruct { random_params })
}

impl RandomUtils for RandomUtilsStruct {
    fn mutation_pos(&self) -> usize {
        rand::random::<usize>() % self.random_params.chromosome_genes_amount()
    }

    fn crossing_chromosome_pos(&self) -> usize {
        rand::random::<usize>() % self.random_params.chromosome_genes_amount()
    }

    fn crossing_zygote_pos(&self) -> usize {
        rand::random::<usize>() % self.random_params.chromosome_genes_amount()
    }

    fn should_cross_zygotes(&self) -> bool {
        rand::random::<f64>() <
            f64::value_from(self.random_params.chromosome_genes_amount()).unwrap() * 2f64 /
                1_000_000.0
    }

    fn should_mutate(&self) -> bool {
        rand::random::<f64>() <
            f64::value_from(self.random_params.chromosome_genes_amount()).unwrap() / 10_000.0f64
    }

    fn rand_gen(&self) -> Gen {
        rand::random::<Gen>()
    }

    fn generate_zygote(&self) -> Zygote {
        let genes: Vec<Gen> = (0..self.random_params.chromosome_genes_amount())
            .map(|_| rand::random::<Gen>())
            .collect();
        Zygote::new(genes)
    }
}

pub trait ChoosingProbability {
    fn select_individual_probability(&self, fitness: f64) -> bool;
}

pub struct RandomChoosingProbability;

impl ChoosingProbability for RandomChoosingProbability {
    fn select_individual_probability(&self, fitness: f64) -> bool {
        fitness > rand::random::<f64>()
    }
}

pub struct PerfChoosingProbability;

impl ChoosingProbability for PerfChoosingProbability {
    fn select_individual_probability(&self, _fitness: f64) -> bool {
        true
    }
}
