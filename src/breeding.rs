use random_utils::RandomUtils;
use chromosome::Chromosome;
use std::rc::Rc;

pub trait Breeding {
    fn generate_chromosome(&self) -> Chromosome;
    fn conception(&self, first_parent: &Chromosome, second_parent: &Chromosome) -> Chromosome;
    fn attempt_cross_zygotes(&self, chr: Chromosome) -> Chromosome;
    fn attempt_mutate(&self, chr: Chromosome) -> Chromosome;
}

pub struct BreedingStruct {
    random_utils: Rc<RandomUtils>,
}

pub fn make_breeding(random_utils: Rc<RandomUtils>) -> Rc<Breeding> {
    Rc::new(BreedingStruct { random_utils })
}

impl Breeding for BreedingStruct {
    fn generate_chromosome(&self) -> Chromosome {
        Chromosome::new(
            self.random_utils.generate_zygote(),
            self.random_utils.generate_zygote(),
        )
    }

    fn conception(&self, first_parent: &Chromosome, second_parent: &Chromosome) -> Chromosome {
        Self::attempt_cross_zygotes(
            self,
            Self::attempt_mutate(
                self,
                first_parent.cross_chromosomes(
                    second_parent,
                    self.random_utils.crossing_chromosome_pos(),
                    self.random_utils.crossing_chromosome_pos(),
                ),
            ),
        )
    }

    fn attempt_cross_zygotes(&self, chr: Chromosome) -> Chromosome {
        if self.random_utils.should_cross_zygotes() {
            chr.cross_zygotes(
                self.random_utils.crossing_zygote_pos(),
                self.random_utils.crossing_zygote_pos() + 1,
            )
        } else {
            chr
        }
    }

    fn attempt_mutate(&self, chr: Chromosome) -> Chromosome {
        if self.random_utils.should_mutate() {
            chr.mutate(
                self.random_utils.mutation_pos(),
                &self.random_utils.rand_gen(),
            )
        } else {
            chr
        }
    }
}
