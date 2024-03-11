use std::rc::Rc;

use crate::{chromosome::Chromosome, random_utils::RandomUtils};

pub trait Breeding {
    fn generate_chromosome(&self) -> Chromosome;
    fn conception(&self, first_parent: &mut Chromosome, second_parent: &Chromosome);
    fn attempt_cross_zygotes(&self, chr: &mut Chromosome);
    fn attempt_mutate(&self, chr: &mut Chromosome);
}

pub struct BreedingStruct {
    random_utils: Rc<dyn RandomUtils>,
}

pub fn make_breeding(random_utils: Rc<dyn RandomUtils>) -> Rc<dyn Breeding> {
    Rc::new(BreedingStruct { random_utils })
}

impl Breeding for BreedingStruct {
    fn generate_chromosome(&self) -> Chromosome {
        Chromosome::new(
            self.random_utils.generate_zygote(),
            self.random_utils.generate_zygote(),
        )
    }

    fn conception(&self, first_parent: &mut Chromosome, second_parent: &Chromosome) {
        first_parent.cross_chromosomes(
            second_parent,
            self.random_utils.crossing_chromosome_pos(),
            self.random_utils.crossing_chromosome_pos(),
        );
        self.attempt_mutate(first_parent);
        self.attempt_cross_zygotes(first_parent);
    }

    fn attempt_cross_zygotes(&self, chr: &mut Chromosome) {
        if self.random_utils.should_cross_zygotes() {
            chr.cross_zygotes(
                self.random_utils.crossing_zygote_pos(),
                self.random_utils.crossing_zygote_pos() + 1,
            )
        }
    }

    fn attempt_mutate(&self, chr: &mut Chromosome) {
        if self.random_utils.should_mutate() {
            chr.mutate(
                self.random_utils.mutation_pos(),
                &self.random_utils.rand_gen(),
            );
        };
    }
}
