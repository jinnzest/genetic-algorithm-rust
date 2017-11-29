use zygote::Zygote;
use std::collections::HashMap;
use std::fmt;
use gen::Gen;

#[allow(dead_code)]
lazy_static! {
#[allow(dead_code)]
static ref DECODING_RULES: HashMap<(Gen, Gen), bool> = {
    let mut hm = HashMap::new();
    hm.insert((Gen::D1, Gen::D0), true);
    hm.insert((Gen::D1, Gen::D1), true);
    hm.insert((Gen::D1, Gen::R1), true);
    hm.insert((Gen::D1, Gen::R0), true);

    hm.insert((Gen::D0, Gen::D0), false);
    hm.insert((Gen::D0, Gen::D1), false);
    hm.insert((Gen::D0, Gen::R1), false);
    hm.insert((Gen::D0, Gen::R0), false);

    hm.insert((Gen::R1, Gen::D0), false);
    hm.insert((Gen::R1, Gen::D1), true);
    hm.insert((Gen::R1, Gen::R1), true);
    hm.insert((Gen::R1, Gen::R0), true);

    hm.insert((Gen::R0, Gen::D0), false);
    hm.insert((Gen::R0, Gen::D1), true);
    hm.insert((Gen::R0, Gen::R1), false);
    hm.insert((Gen::R0, Gen::R0), false);
    hm
};
}

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
    pub fn decode_genotype(&self) -> Vec<bool> {
        let pairs = self.dominant.get_genes().into_iter().zip(
            self.recessive
                .get_genes(),
        );
        pairs
            .map(|(d, r)| DECODING_RULES.get(&(d, r)))
            .map(|o| o.unwrap())
            .cloned()
            .collect()
    }
    fn from_strings(dominant: &str, recessive: &str) -> Chromosome {
        Chromosome {
            dominant: dominant.parse::<Zygote>().unwrap(),
            recessive: recessive.parse::<Zygote>().unwrap(),
        }
    }
    pub fn cross_zygotes(&self, begin: usize, amount: usize) -> Chromosome {
        Chromosome {
            dominant: self.dominant.cross(&self.recessive, begin, amount),
            recessive: self.recessive.cross(&self.dominant, begin, amount),
        }
    }
    pub fn cross_chromosomes(&self, that: &Chromosome, begin: usize, amount: usize) -> Chromosome {
        Chromosome {
            dominant: self.dominant.cross(&that.dominant, begin, amount),
            recessive: self.recessive.cross(&that.recessive, begin, amount),
        }
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
        assert_eq!(bools_to_str(&chr.decode_genotype()), "1100");
    }

    #[test]
    fn must_override_dominant_genes_of_second_one() {
        let chr = Chromosome::from_strings("DDdd", "DdDd");
        assert_eq!(bools_to_str(&chr.decode_genotype()), "1100");
    }
}

#[cfg(test)]
mod decoding_first_zygote_with_recessive_genes {
    use super::*;

    #[test]
    fn must_override_recessive_genes_of_second_one() {
        let chr = Chromosome::from_strings("RRrr", "RrRr");
        assert_eq!(bools_to_str(&chr.decode_genotype()), "1100");
    }

    #[test]
    fn must_override_dominant_genes_of_second_one() {
        let chr = Chromosome::from_strings("RRrr", "DdDd");
        assert_eq!(bools_to_str(&chr.decode_genotype()), "1010");
    }
}

#[cfg(test)]
mod crossing_zygote {
    use super::*;

    #[test]
    fn must_swap_3_genes_starting_from_pos_2() {
        let chr = Chromosome::from_strings(
            "dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd",
            "rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr",
        );
        assert_eq!(
            chr.cross_zygotes(2, 3).to_string(),
            "dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddr rrdd\
            \nrrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrd ddrr"
        );
    }

    #[quickcheck]
    fn must_swap_whole_right_pos_if_amount_is_more_than_length(pos: usize) -> bool {
        let chr = Chromosome::from_strings(
            "dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd",
            "rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr",
        );
        chr.cross_zygotes(3, 61 + pos).to_string() ==
            "rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rddd\
            \ndddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd drrr"
    }

    #[test]
    fn cross_chromosomes() {
        let first = Chromosome::from_strings(
            "dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd",
            "rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr",
        );
        let second = Chromosome::from_strings(
            "DDDD DDDD DDDD DDDD DDDD DDDD DDDD DDDD DDDD DDDD DDDD DDDD DDDD DDDD DDDD DDDD",
            "RRRR RRRR RRRR RRRR RRRR RRRR RRRR RRRR RRRR RRRR RRRR RRRR RRRR RRRR RRRR RRRR",
        );
        assert_eq!(
            first.cross_chromosomes(&second, 1, 2).to_string(),
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

fn bools_to_str(bools: &[bool]) -> String {
    bools
        .iter()
        .map(|b| if *b { '1' } else { '0' })
        .rev()
        .collect()
}
