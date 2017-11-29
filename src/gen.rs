use std::fmt;
use std::str;

#[derive(PartialEq, Clone, Rand, Eq, Hash)]
pub enum Gen {
    D1,
    D0,
    R1,
    R0,
}

impl fmt::Display for Gen {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match *self {
            Gen::D1 => "D",
            Gen::D0 => "d",
            Gen::R1 => "R",
            Gen::R0 => "r",
        };
        write!(f, "{}", s)
    }
}

impl fmt::Debug for Gen {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl str::FromStr for Gen {
    type Err = String;

    fn from_str(s: &str) -> Result<Gen, String> {
        let chars: Vec<char> = s.chars().collect();
        match chars.len() {
            0 => Err("gen can't be extracted from an empty string".to_string()),
            1 => Gen::from_char(chars[0]),
            _ => Err(
                "gen str must contain not more than one character".to_string(),
            ),
        }
    }
}

pub struct VecGen {
    genes: Vec<Gen>,
}

impl fmt::Display for VecGen {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = &self.genes.iter().fold(String::new(), |acc, g| {
            format!("{}{}", g, acc)
        });
        write!(f, "{}", s)
    }
}

impl VecGen {
    pub fn new(genes: Vec<Gen>) -> Self {
        VecGen { genes }
    }
}

impl Gen {
    pub fn from_char(c: char) -> Result<Gen, String> {
        match c {
            'D' => Ok(Gen::D1),
            'd' => Ok(Gen::D0),
            'R' => Ok(Gen::R1),
            'r' => Ok(Gen::R0),
            _ => Err(format!("unexpected char to create gen: '{}'", c)),
        }
    }
    pub fn to_char(&self) -> char {
        match *self {
            Gen::D1 => 'D',
            Gen::D0 => 'd',
            Gen::R1 => 'R',
            Gen::R0 => 'r',
        }
    }
}

#[cfg(test)]
mod to_and_from_str {
    use super::*;
    use std::str::FromStr;


    #[test]
    fn empty_str_to_gen_returns_err() {
        assert_eq!(
            Gen::from_str("").unwrap_err(),
            "gen can't be extracted from an empty string"
        );
    }

    #[test]
    fn gen_to_str() {
        assert_eq!(Gen::D1.to_string(), "D");
        assert_eq!(Gen::D0.to_string(), "d");
        assert_eq!(Gen::R1.to_string(), "R");
        assert_eq!(Gen::R0.to_string(), "r");
    }

    #[test]
    fn str_to_gen() {
        assert_eq!(Gen::from_str("D").unwrap(), Gen::D1);
        assert_eq!(Gen::from_str("d").unwrap(), Gen::D0);
        assert_eq!(Gen::from_str("R").unwrap(), Gen::R1);
        assert_eq!(Gen::from_str("r").unwrap(), Gen::R0);
    }
}
