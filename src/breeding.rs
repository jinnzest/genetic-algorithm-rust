use std::marker::PhantomData;
use random_utils::RandomUtils;
use chromosome::Chromosome;

pub trait Breeding<RU: RandomUtils> {
    fn generate_chromosome() -> Chromosome;
    fn conception(first_parent: &Chromosome, second_parent: &Chromosome) -> Chromosome;
    fn attempt_cross_zygotes(chr: Chromosome) -> Chromosome;
    fn attempt_mutate(chr: Chromosome) -> Chromosome;
}

pub struct BreedingStruct<RU: RandomUtils> {
    phantom: PhantomData<RU>,
}

impl<RU: RandomUtils> Breeding<RU> for BreedingStruct<RU> {
    fn generate_chromosome() -> Chromosome {
        Chromosome::new(RU::generate_zygote(), RU::generate_zygote())
    }

    fn conception(first_parent: &Chromosome, second_parent: &Chromosome) -> Chromosome {
        Self::attempt_cross_zygotes(Self::attempt_mutate(first_parent.cross_chromosomes(
            second_parent,
            RU::crossing_chromosome_pos(),
            RU::crossing_chromosome_pos(),
        )))
    }

    fn attempt_cross_zygotes(chr: Chromosome) -> Chromosome {
        if RU::should_cross_zygotes() {
            chr.cross_zygotes(RU::crossing_zygote_pos(), RU::crossing_zygote_pos() + 1)
        } else {
            chr
        }
    }

    fn attempt_mutate(chr: Chromosome) -> Chromosome {
        if RU::should_mutate() {
            chr.mutate(RU::mutation_pos(), &RU::rand_gen())
        } else {
            chr
        }
    }
}
