
use chromosome::Chromosome;
use std::fmt;

#[derive(Clone)]
pub struct Individual {
    pub chromosome: Chromosome,
    pub fitness: f64,
}

impl fmt::Display for Individual {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\n{}\n{}\n", self.chromosome, self.fitness)
    }
}

impl fmt::Debug for Individual {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}
