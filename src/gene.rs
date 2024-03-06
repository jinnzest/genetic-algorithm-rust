use std::fmt;
use std::str;

use rand::Rng;
use rand::prelude::IndexedRandom;
use rand_distr::Distribution;
use rand_distr::StandardUniform;

#[derive(PartialEq, Clone, Eq, Hash)]
pub enum Gene {
    D1,
    D0,
    R1,
    R0,
}

impl Distribution<Gene> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Gene {
        [Gene::D1, Gene::D0, Gene::R1, Gene::R0]
            .choose(rng)
            .unwrap()
            .clone()
    }
}

impl fmt::Display for Gene {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match *self {
            Gene::D1 => "D",
            Gene::D0 => "d",
            Gene::R1 => "R",
            Gene::R0 => "r",
        };
        write!(f, "{}", s)
    }
}

impl fmt::Debug for Gene {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl str::FromStr for Gene {
    type Err = String;

    fn from_str(s: &str) -> Result<Gene, String> {
        let chars: Vec<char> = s.chars().collect();
        match chars.len() {
            0 => Err("gene can't be extracted from an empty string".to_string()),
            1 => Gene::from_char(chars[0]),
            _ => Err("gene str must contain not more than one character".to_string()),
        }
    }
}

pub struct VecGen {
    genes: Vec<Gene>,
}

impl fmt::Display for VecGen {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = &self
            .genes
            .iter()
            .fold(String::new(), |acc, g| format!("{}{}", g, acc));
        write!(f, "{}", s)
    }
}

impl VecGen {
    pub fn new(genes: Vec<Gene>) -> Self {
        VecGen { genes }
    }
}

impl Gene {
    pub fn from_char(c: char) -> Result<Gene, String> {
        match c {
            'D' => Ok(Gene::D1),
            'd' => Ok(Gene::D0),
            'R' => Ok(Gene::R1),
            'r' => Ok(Gene::R0),
            _ => Err(format!("unexpected char to create gene: '{}'", c)),
        }
    }
    pub fn to_char(&self) -> char {
        match *self {
            Gene::D1 => 'D',
            Gene::D0 => 'd',
            Gene::R1 => 'R',
            Gene::R0 => 'r',
        }
    }
}

#[cfg(test)]
mod to_and_from_str {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn empty_str_to_gene_returns_err() {
        assert_eq!(
            Gene::from_str("").unwrap_err(),
            "gene can't be extracted from an empty string"
        );
    }

    #[test]
    fn gene_to_str() {
        assert_eq!(Gene::D1.to_string(), "D");
        assert_eq!(Gene::D0.to_string(), "d");
        assert_eq!(Gene::R1.to_string(), "R");
        assert_eq!(Gene::R0.to_string(), "r");
    }

    #[test]
    fn str_to_gene() {
        assert_eq!(Gene::from_str("D").unwrap(), Gene::D1);
        assert_eq!(Gene::from_str("d").unwrap(), Gene::D0);
        assert_eq!(Gene::from_str("R").unwrap(), Gene::R1);
        assert_eq!(Gene::from_str("r").unwrap(), Gene::R0);
    }
}
