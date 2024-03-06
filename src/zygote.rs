use std::fmt;
use std::marker;
use std::str;

use crate::gene::Gene;
use crate::global_constants::U64_BITS_AMOUNT;

#[derive(Clone)]
pub struct Zygote {
    genes: Vec<Gene>,
}

impl fmt::Display for Zygote {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let chars = self
            .genes
            .iter()
            .map(|g| match *g {
                Gene::D1 => 'D',
                Gene::D0 => 'd',
                Gene::R1 => 'R',
                Gene::R0 => 'r',
            })
            .rev()
            .collect::<Vec<char>>();

        let formatted = group_by_u64_and_byte_pos(&chars);
        write!(f, "{}", formatted.trim())
    }
}

impl fmt::Debug for Zygote {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self, f)
    }
}

impl str::FromStr for Zygote {
    type Err = String;

    fn from_str(s: &str) -> Result<Zygote, String> {
        let filtered = drop_spaces(s);
        let result_gens = filtered
            .chars()
            .map(|c| match c {
                'D' => Ok(Gene::D1),
                'd' => Ok(Gene::D0),
                'R' => Ok(Gene::R1),
                'r' => Ok(Gene::R0),
                _ => Err(c),
            })
            .rev()
            .collect::<Vec<Result<Gene, char>>>();
        let unexpected_char = result_gens.iter().find(|c| c.is_err());
        match unexpected_char {
            Some(c) => Err(format!(
                "unexpected character in genes string: {}",
                c.clone().unwrap_err()
            )),
            None => {
                let genes = result_gens
                    .clone()
                    .into_iter()
                    .map(|c| c.unwrap())
                    .collect::<Vec<Gene>>();
                Ok(Zygote::new(genes))
            }
        }
    }
}

impl Zygote {
    pub fn new(genes: Vec<Gene>) -> Self {
        Self { genes }
    }

    pub fn get_genes(&self) -> Vec<Gene> {
        self.genes.clone()
    }

    pub fn mutate(&self, pos: usize, new_gen: &Gene) -> Zygote {
        let mut genes = self.genes.clone();
        genes[pos] = new_gen.clone();
        Zygote { genes }
    }

    pub fn cross(&self, that: &Zygote, begin: usize, amount: usize) -> Zygote {
        let (h1, _, t1) = self.split_genes(begin, amount);
        let (_, m2, _) = that.split_genes(begin, amount);
        let mut vec = Vec::new();
        vec.extend_from_slice(h1);
        vec.extend_from_slice(m2);
        vec.extend_from_slice(t1);
        Zygote { genes: vec }
    }

    fn split_genes(&self, begin: usize, amount: usize) -> (&[Gene], &[Gene], &[Gene]) {
        fn normalize_pos(p: usize, len: usize) -> usize {
            if p > len - 1 { len } else { p }
        }

        let n_begin = normalize_pos(begin, self.genes.len());
        let (head, tail) = self.genes.split_at(n_begin);
        let n_end = normalize_pos(amount, tail.len());
        let (head_of_tail, tail_of_tail) = tail.split_at(n_end);
        (head, head_of_tail, tail_of_tail)
    }
}

#[cfg(test)]
mod to_and_from_str {
    use crate::gene;

    use super::*;
    use gene::VecGen;
    use quickcheck::{Arbitrary, Gen, quickcheck};
    use std::iter;
    use std::str::FromStr;

    impl Arbitrary for Gene {
        fn arbitrary(g: &mut Gen) -> Self {
            g.choose(&[Gene::D0, Gene::D1, Gene::R0, Gene::R1])
                .unwrap()
                .clone()
        }

        fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
            Box::new(iter::empty())
        }
    }

    #[test]
    fn zygote_to_str() {
        assert_eq!(
            Zygote {
                genes: vec![Gene::D1, Gene::D0, Gene::R1, Gene::R0]
            }
            .to_string(),
            "rRdD"
        );
    }

    #[test]
    fn str_to_zygote() {
        assert_eq!(
            Zygote::from_str("rRdD").unwrap().genes,
            vec![Gene::D1, Gene::D0, Gene::R1, Gene::R0]
        );
    }

    #[test]
    fn str_to_zygote_drops_spaces() {
        assert_eq!(
            Zygote::from_str(" r  R d      D  ").unwrap().genes,
            vec![Gene::D1, Gene::D0, Gene::R1, Gene::R0]
        );
    }

    quickcheck! {
        fn to_and_from_str_genes(genes: Vec<Gene>) -> bool {
            if genes.is_empty() {
                true
            } else {
                let genes_str = format!("{}", VecGen::new(genes));
                let zgt = Zygote::from_str(&genes_str).unwrap();
                let back_str = format!("{}", zgt);
                let filtered_back: String = drop_spaces(&back_str);
                let aligned_genes = format!("{:r>1$}", genes_str, filtered_back.len());
                println!("from: {}\nto  : {}", aligned_genes, filtered_back);
                aligned_genes == filtered_back
            }
        }
    }
}

#[cfg(test)]
mod mutating {

    use crate::gene;

    use super::*;
    use gene::*;
    use quickcheck::quickcheck;
    use std::str::FromStr;

    quickcheck! {
    fn not_modify_genes_outside_defined_pos(genes: Vec<Gene>, pos: usize, new_gen: Gene) -> bool {
        if genes.is_empty() {
            true
        } else {
            let n_pos = pos % genes.len();
            let genes_str = &format!("{}", VecGen::new(genes));
            let zgt = Zygote::from_str(genes_str).unwrap();
            let mutated_zygote = zgt.mutate(n_pos, &new_gen);
            let mutated_zgt_str: String = format!("{}", mutated_zygote)
                .chars()
                .filter(|c| !c.is_whitespace())
                .collect();
            let mut mutated_zgt_chars = mutated_zgt_str.chars().rev();
            let normilazed = format!("{:r>1$}", genes_str, mutated_zgt_str.len());
            let vec: Vec<char> = normilazed.chars().collect();
            println!("genes_str  ={}", group_by_u64_and_byte_pos(&vec));
            println!("mutated_zgt={}", zgt);
            let genes_iter = genes_str.chars().rev();
            let (res, _) = genes_iter.fold((true, 0), |(acc, p), g| {
                let tst_gen = mutated_zgt_chars.next().unwrap();
                (
                    acc && if p == n_pos {
                        true
                    } else {
                        if g == tst_gen {
                            true
                        } else {
                            println!("{}!={}", g, tst_gen);
                            false
                        }
                    },
                    p + 1,
                )
            });
            res
        }
    }}

    quickcheck! {
        fn modify_gene_defined_by_pos(genes: Vec<Gene>, pos: usize, new_gen: Gene) -> bool {
            if genes.is_empty() {
                true
            } else {
                let n_pos = pos % genes.len();
                let zgt = Zygote { genes: genes };
                let mutated_gen = zgt.mutate(n_pos, &new_gen);
                let (res, _) = mutated_gen.genes.iter().fold((true, 0), |(acc, p), g| {
                    (acc && if p == pos { *g == new_gen } else { true }, p + 1)
                });
                res
            }
        }
    }
}
#[cfg(test)]
mod crossing {

    use super::*;
    use std::str::FromStr;

    #[test]
    fn cross_parts() {
        let zgt1 = Zygote::from_str(
            "dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd",
        )
        .unwrap();
        let zgt2 = Zygote::from_str(
            "rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr",
        )
        .unwrap();
        let crossed = zgt1.cross(&zgt2, 3, 4);
        assert_eq!(
            crossed.to_string(),
            "dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd drrr rddd"
        )
    }

    #[test]
    fn cross_parts_when_end_pos_bigger_than_size() {
        let zgt1 = Zygote::from_str(
            "dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd",
        )
        .unwrap();
        let zgt2 = Zygote::from_str(
            "rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr",
        )
        .unwrap();
        let crossed = zgt1.cross(&zgt2, 3, 100);
        assert_eq!(
            crossed.to_string(),
            "rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rddd"
        )
    }
}

pub fn drop_spaces(s: &str) -> String {
    s.chars().filter(|c| !c.is_whitespace()).collect::<String>()
}

trait Grouped
where
    Self: marker::Sized,
{
    fn grouped(&self, p: usize) -> Vec<Vec<char>>;
}

impl Grouped for &'_ [char] {
    fn grouped(&self, p: usize) -> Vec<Vec<char>> {
        let mut vec = Vec::new();
        let mut pos = 0;
        let mut gathered = Vec::new();
        for ch in self.iter() {
            if pos == p {
                vec.push(gathered.clone());
                gathered = Vec::new();
                gathered.push(*ch);
                pos = 1;
            } else {
                gathered.push(*ch);
                pos += 1;
            }
        }
        vec.push(gathered);

        vec
    }
}

pub fn group_by_u64_and_byte_pos(s: &[char]) -> String {
    let new_str = s
        .grouped(U64_BITS_AMOUNT)
        .iter()
        .fold(Vec::new(), |mut acc, v| {
            let b = v
                .as_slice()
                .grouped(4)
                .into_iter()
                .fold(Vec::new(), |mut acc2, v2| {
                    acc2.push(v2);
                    acc2.push(vec![' ']);
                    acc2
                });
            acc.append(&mut b.clone());
            acc.append(&mut vec![vec![' ']]);
            acc
        });
    let mut new_vec = Vec::new();
    for v in new_str {
        let mut cp = v.clone();
        new_vec.append(&mut cp);
    }
    new_vec.iter().collect()
}
