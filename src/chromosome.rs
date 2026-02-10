use std::fmt;

use crate::{gene::Gene, zygote::Zygote};

#[derive(Clone)]
pub struct Chromosome {
    dominant: Zygote,
    recessive: Zygote,
    decoded_genotype: Vec<u64>,
}

impl fmt::Display for Chromosome {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}\n{}", self.dominant, self.recessive)
    }
}

impl fmt::Debug for Chromosome {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self, f)
    }
}

impl Chromosome {
    pub fn new(dominant: Zygote, recessive: Zygote) -> Self {
        Self {
            decoded_genotype: vec![0; dominant.u64s_amount()],
            dominant,
            recessive,
        }
    }

    pub fn overwrite(&mut self, source: &Chromosome) {
        self.dominant.overwrite(&source.dominant);
        self.recessive.overwrite(&source.recessive);
    }

    pub fn decode_genotype(&mut self) {
        let mut p = 0;
        while p < self.dominant.u64s_amount() {
            let dd = self.dominant.get_d_u64(p);
            let dv = self.dominant.get_v_u64(p);
            let rd = self.recessive.get_d_u64(p);
            let rv = self.recessive.get_v_u64(p);
            self.decoded_genotype[p] = dv & !rd | rd & rv & !dd | dd & dv;
            p += 1
        }
    }
    pub fn decoded_genotype(&self) -> &[u64] {
        &self.decoded_genotype
    }

    pub fn cross_zygotes(&mut self, begin: usize, amount: usize) {
        self.dominant
            .cross_bidirectional(&mut self.recessive, begin, amount);
    }
    pub fn cross_chromosomes(&mut self, that: &Chromosome, begin: usize, amount: usize) {
        self.dominant.cross(&that.dominant, begin, amount);
        self.recessive.cross(&that.recessive, begin, amount);
    }
    pub fn mutate(&mut self, pos: usize, new_generation: &Gene) {
        self.dominant.mutate(pos, new_generation);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn from_strings(dominant: &str, recessive: &str) -> Chromosome {
        Chromosome {
            decoded_genotype: vec![0; dominant.len()],
            dominant: dominant.parse::<Zygote>().unwrap(),
            recessive: recessive.parse::<Zygote>().unwrap(),
        }
    }

    #[test]
    fn to_string_should_concat_zygotes() {
        assert_eq!(
            from_strings(
                "dDrR rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr ",
                "RrDd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd",
            )
            .to_string(),
            "dDrR rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr\
            \nRrDd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd"
        )
    }

    #[cfg(test)]
    mod decoding_first_zygote_with_dominant_genes {
        use super::*;

        #[test]
        fn must_override_recessive_genes_of_second_one() {
            let mut chr = from_strings("DDdd", "RrRr");
            chr.decode_genotype();
            assert_eq!(chr.decoded_genotype(), vec![0b1100u64, 0, 0, 0]);
        }

        #[test]
        fn must_override_dominant_genes_of_second_one() {
            let mut chr = from_strings("DDdd", "DdDd");
            chr.decode_genotype();
            assert_eq!(chr.decoded_genotype(), vec![0b1100u64, 0, 0, 0]);
        }
    }

    #[cfg(test)]
    mod decoding_first_zygote_with_recessive_genes {
        use super::*;

        #[test]
        fn must_override_recessive_genes_of_second_one() {
            let mut chr = from_strings("RRrr", "RrRr");
            chr.decode_genotype();
            assert_eq!(chr.decoded_genotype(), vec![0b1100u64, 0, 0, 0]);
        }

        #[test]
        fn must_override_dominant_genes_of_second_one() {
            let mut chr = from_strings("RRrr", "DdDd");
            chr.decode_genotype();
            assert_eq!(chr.decoded_genotype(), vec![0b1010u64, 0, 0, 0]);
        }
    }

    #[cfg(test)]
    mod crossing_zygote {
        use super::*;
        use quickcheck::quickcheck;

        #[test]
        fn must_swap_3_genes_starting_from_pos_2() {
            let mut chr = from_strings(
                "dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd",
                "rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr",
            );
            chr.cross_zygotes(2, 3);
            assert_eq!(
            chr.to_string(),
            from_strings(
                "dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddr rrdd",
                "rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrd ddrr",
            )
            .to_string()
        );
        }

        quickcheck! {
        fn must_swap_whole_right_pos_if_amount_is_more_than_length(pos: usize) -> bool {
            let mut chr = from_strings(
                "dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd",
                "rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr",
            );
            chr.cross_zygotes(3, pos.saturating_add(61));
            chr.to_string() ==
                "rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rddd\
                \ndddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd drrr"
        }}

        #[test]
        fn cross_chromosomes() {
            let mut first = from_strings(
                "dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd",
                "rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr",
            );
            let second = from_strings(
                "DDDD DDDD DDDD DDDD DDDD DDDD DDDD DDDD DDDD DDDD DDDD DDDD DDDD DDDD DDDD DDDD",
                "RRRR RRRR RRRR RRRR RRRR RRRR RRRR RRRR RRRR RRRR RRRR RRRR RRRR RRRR RRRR RRRR",
            );
            first.cross_chromosomes(&second, 1, 2);
            assert_eq!(
                first.to_string(),
                "dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dDDd\
            \nrrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rRRr"
            )
        }

        #[test]
        fn mutate_gene_in_dominant() {
            let mut chr = from_strings(
                "dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd",
                "rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr",
            );
            chr.mutate(2, &Gene::R1);
            assert_eq!(
                chr.to_string(),
                "dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dRdd\
            \nrrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr"
            )
        }
    }
}
