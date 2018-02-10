use zygote::Zygote;
use std::fmt;
use gen::Gen;

#[derive(Clone)]
pub struct Chromosome {
    dominant: Zygote,
    recessive: Zygote,
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
            dominant,
            recessive,
        }
    }

    pub fn decode_genotype(&self) -> Vec<u64> {
        let mut p = 0;
        let mut decoded = Vec::with_capacity(self.dominant.u64s_amount());
        while p < self.dominant.u64s_amount() {
            let dd = self.dominant.get_d_u64(p);
            let dv = self.dominant.get_v_u64(p);
            let rd = self.recessive.get_d_u64(p);
            let rv = self.recessive.get_v_u64(p);
            decoded.push(dv & !rd | rd & rv & !dd | dd & dv);
            p += 1
        }
        decoded
    }
    #[allow(dead_code)]
    fn from_strings(dominant: &str, recessive: &str) -> Chromosome {
        Chromosome {
            dominant: dominant.parse::<Zygote>().unwrap(),
            recessive: recessive.parse::<Zygote>().unwrap(),
        }
    }
    pub fn cross_zygotes(&mut self, begin: usize, amount: usize) -> Chromosome {
        self.dominant.cross(
            &mut self.recessive,
            begin,
            amount,
            true,
        );
        self.clone()
    }
    pub fn cross_chromosomes(&self, that: &Chromosome, begin: usize, amount: usize) -> Chromosome {
        let mut new_chr = Chromosome {
            dominant: self.dominant.clone(),
            recessive: self.recessive.clone(),
        };

        new_chr.dominant.cross(
            &mut that.dominant.clone(),
            begin,
            amount,
            false,
        );
        new_chr.recessive.cross(
            &mut that.recessive.clone(),
            begin,
            amount,
            false,
        );
        new_chr
    }
    pub fn mutate(&self, pos: usize, new_gen: &Gen) -> Chromosome {
        Chromosome {
            dominant: self.dominant.mutate(pos, new_gen),
            recessive: self.recessive.clone(),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_string_should_concat_zygotes() {
        assert_eq!(
            Chromosome::from_strings(
                "dDrR rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr ",
                "RrDd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd",
            ).to_string(),
            "dDrR rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr\
            \nRrDd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd"
        )
    }
}

#[cfg(test)]
mod decoding_first_zygote_with_dominant_genes {
    use super::*;

    #[test]
    fn must_override_recessive_genes_of_second_one() {
        let chr = Chromosome::from_strings("DDdd", "RrRr");
        assert_eq!(chr.decode_genotype(), vec![0b1100u64]);
    }

    #[test]
    fn must_override_dominant_genes_of_second_one() {
        let chr = Chromosome::from_strings("DDdd", "DdDd");
        assert_eq!(chr.decode_genotype(), vec![0b1100u64]);
    }
}

#[cfg(test)]
mod decoding_first_zygote_with_recessive_genes {
    use super::*;

    #[test]
    fn must_override_recessive_genes_of_second_one() {
        let chr = Chromosome::from_strings("RRrr", "RrRr");
        assert_eq!(chr.decode_genotype(), vec![0b1100u64]);
    }

    #[test]
    fn must_override_dominant_genes_of_second_one() {
        let chr = Chromosome::from_strings("RRrr", "DdDd");
        assert_eq!(chr.decode_genotype(), vec![0b1010u64]);
    }
}

#[cfg(test)]
mod crossing_zygote {
    use super::*;

    #[test]
    fn must_swap_3_genes_starting_from_pos_2() {
        let mut chr = Chromosome::from_strings(
            "dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd",
            "rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr",
        );
        chr.cross_zygotes(2, 3);
        assert_eq!(
            chr.to_string(),
            "dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddr rrdd\
            \nrrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrd ddrr"
        );
    }

    #[quickcheck]
    fn must_swap_whole_right_pos_if_amount_is_more_than_length(pos: usize) -> bool {
        let mut chr = Chromosome::from_strings(
            "dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd",
            "rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr",
        );
        chr.cross_zygotes(3, 61 + pos);
        chr.to_string() ==
            "rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rddd\
            \ndddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd drrr"
    }

    #[test]
    fn cross_chromosomes() {
        let first = Chromosome::from_strings(
            "dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd",
            "rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr",
        );
        let mut second = Chromosome::from_strings(
            "DDDD DDDD DDDD DDDD DDDD DDDD DDDD DDDD DDDD DDDD DDDD DDDD DDDD DDDD DDDD DDDD",
            "RRRR RRRR RRRR RRRR RRRR RRRR RRRR RRRR RRRR RRRR RRRR RRRR RRRR RRRR RRRR RRRR",
        );
        assert_eq!(
            first.cross_chromosomes(&mut second, 1, 2).to_string(),
            "dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dDDd\
            \nrrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rRRr"
        )
    }

    #[test]
    fn mutate_gen_in_dominant() {
        let chr = Chromosome::from_strings(
            "dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd",
            "rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr",
        );
        assert_eq!(
            chr.mutate(2, &Gen::R1).to_string(),
            "dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dRdd\
            \nrrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr"
        )
    }
}

fn _bools_to_str(bools: &[bool]) -> String {
    bools
        .iter()
        .map(|b| if *b { '1' } else { '0' })
        .rev()
        .collect()
}
