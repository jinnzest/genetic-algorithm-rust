use gen::Gen;
use zygote::Zygote;
use rand;
use conv::ValueFrom;
use std::marker::PhantomData;
use u64s::U64s;

pub trait RandomUtils {
    fn mutation_pos() -> usize;

    fn crossing_chromosome_pos() -> usize;

    fn crossing_zygote_pos() -> usize;

    fn should_cross_zygotes() -> bool;

    fn should_mutate() -> bool;

    fn rand_gen() -> Gen;

    fn generate_zygote() -> Zygote;
}

pub struct RandomUtilsStruct<R>
where
    R: RandomParams,
{
    _p: PhantomData<R>,
}

pub struct RandomParamsStruct;

pub trait RandomParams {
    fn chromosome_genes_amount() -> usize;
}

impl<R> RandomUtils for RandomUtilsStruct<R>
where
    R: RandomParams,
{
    fn mutation_pos() -> usize {
        rand::random::<usize>() % R::chromosome_genes_amount()
    }

    fn crossing_chromosome_pos() -> usize {
        rand::random::<usize>() % R::chromosome_genes_amount()
    }

    fn crossing_zygote_pos() -> usize {
        rand::random::<usize>() % R::chromosome_genes_amount()
    }

    fn should_cross_zygotes() -> bool {
        rand::random::<f64>() <
            f64::value_from(R::chromosome_genes_amount()).unwrap() * 2f64 / 1_000_000.0
    }

    fn should_mutate() -> bool {
        rand::random::<f64>() < f64::value_from(R::chromosome_genes_amount()).unwrap() / 10_000.0f64
    }

    fn rand_gen() -> Gen {
        rand::random::<Gen>()
    }

    fn generate_zygote() -> Zygote {
        let len = R::chromosome_genes_amount() / 64;
        let d = (0..len).map(|_| rand::random::<u64>()).collect();
        let v = (0..len).map(|_| rand::random::<u64>()).collect();
        let dominance = U64s::new(d);
        let values = U64s::new(v);
        Zygote::new(dominance, values)
    }
}

pub trait ChoosingProbability {
    fn select_individual_probability(fitness: f64) -> bool;
}

pub struct RandomChoosingProbability;

impl ChoosingProbability for RandomChoosingProbability {
    fn select_individual_probability(fitness: f64) -> bool {
        fitness > rand::random::<f64>()
    }
}

pub struct PerfChoosingProbability;

impl ChoosingProbability for PerfChoosingProbability {
    fn select_individual_probability(_fitness: f64) -> bool {
        true
    }
}
