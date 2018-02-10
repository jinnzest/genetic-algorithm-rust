use std::str;
use std::fmt;
use gen::Gen;
use u64s::{U64s, U64sStruct};
use u64s;

#[derive(Clone)]
pub struct Zygote {
    dominance: U64sStruct,
    values: U64sStruct,
}

impl fmt::Display for Zygote {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let d_str = format!("{}", self.dominance);
        let v_str = format!("{}", self.values);
        let res = d_str
            .chars()
            .zip(v_str.chars())
            .map(|(d, v)| match (d, v) {
                ('1', '1') => 'D',
                ('1', '0') => 'd',
                ('0', '1') => 'R',
                ('0', '0') => 'r',
                (_, _) => panic!(),
            })
            .collect::<String>();
        let chars: Vec<char> = res.chars().collect();
        let formatted: String = u64s::group_by_u64_and_byte_pos(&chars).iter().collect();
        write!(f, "{}", formatted.trim())
    }
}

impl fmt::Debug for Zygote {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl str::FromStr for Zygote {
    type Err = String;

    fn from_str(s: &str) -> Result<Zygote, String> {
        let filtered: String = s.chars().filter(|c| !c.is_whitespace()).collect();
        let dominance_bit_chars: String = filtered
            .chars()
            .map(|c| match c {
                'D' | 'd' => '1',
                'R' | 'r' => '0',
                _ => panic!(format!("unexpected char = {}", c)),
            })
            .collect();
        let values_bit_chars: String = filtered
            .chars()
            .map(|c| match c {
                'R' | 'D' => '1',
                'd' | 'r' => '0',
                _ => panic!(format!("unexpected char = {}", c)),
            })
            .collect();
        result! {
                let dominance <- dominance_bit_chars.parse::<U64sStruct>();
                let values <- values_bit_chars.parse::<U64sStruct>();
                Zygote::new ( dominance, values )
        }
    }
}

impl Zygote {
    pub fn new(dominance: U64sStruct, values: U64sStruct) -> Self {
        Self { dominance, values }
    }

    pub fn get_d_u64(&self, p: usize) -> u64 {
        self.dominance.get_u64(p)
    }

    pub fn get_v_u64(&self, p: usize) -> u64 {
        self.values.get_u64(p)
    }

    pub fn u64s_amount(&self) -> usize {
        self.dominance.u64s_amount()
    }

    pub fn mutate(&self, pos: usize, new_gen: &Gen) -> Zygote {
        let dominance = self.dominance.clone();
        let values = self.values.clone();
        let mut zgt = Zygote { dominance, values };
        zgt.set(pos, new_gen);
        zgt
    }

    fn set(&mut self, pos: usize, gen: &Gen) {
        match *gen {
            Gen::D1 => {
                self.dominance.set(pos, true);
                self.values.set(pos, true);
            }
            Gen::D0 => {
                self.dominance.set(pos, true);
                self.values.set(pos, false);
            }
            Gen::R1 => {
                self.dominance.set(pos, false);
                self.values.set(pos, true);
            }
            Gen::R0 => {
                self.dominance.set(pos, false);
                self.values.set(pos, false);
            }
        }
    }

    fn get(&self, pos: usize) -> Gen {
        match (self.dominance.get(pos), self.values.get(pos)) {
            (true, true) => Gen::D1,
            (true, false) => Gen::D0,
            (false, true) => Gen::R1,
            (false, false) => Gen::R0,
        }
    }

    pub fn cross(&mut self, that: &mut Zygote, begin: usize, amount: usize, bidirectional: bool) {
        self.dominance.cross_bits(
            &mut that.dominance,
            begin,
            amount,
            bidirectional,
        );
        self.values.cross_bits(
            &mut that.values,
            begin,
            amount,
            bidirectional,
        );
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;
    use gen::VecGen;

    #[test]
    fn zygote_to_str() {
        assert_eq!(
            Zygote {
                dominance: U64s::new(vec![0b0011u64]),
                values: U64s::new(vec![0b0101u64]),
            }.to_string(),
            "rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rRdD"
        );
    }

    #[test]
    fn str_to_zygote() {
        let zgt = Zygote::from_str("rRdD").unwrap();
        assert_eq!(zgt.dominance.get_u64(0), 0b0011u64);
        assert_eq!(zgt.values.get_u64(0), 0b0101u64);
    }

    #[test]
    fn str_to_zygote_drops_spaces() {
        assert_eq!(
            format!("{}", Zygote::from_str(" r  R d      D  ").unwrap()),
            "rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rRdD"
        );
    }

    //    #[test]
    fn not_mod() {
        let n_pos = 0;
        let genes_str = &format!("{}", VecGen::new(vec![Gen::D0, Gen::D1]));
        let zgt = Zygote::from_str(genes_str).unwrap();
        println!("genes_str={}", genes_str);
        println!("before mut={}", zgt);
        let mutated_zgt = zgt.mutate(n_pos, &Gen::D1);
        let mutated_zgt_str: String = format!("{}", mutated_zgt)
            .chars()
            .filter(|c| !c.is_whitespace())
            .collect();
        let mut mutated_zgt_chars = mutated_zgt_str.chars().rev();
        println!("{}", mutated_zgt);
        let genes_iter = genes_str.chars().rev();
        let (res, _) = genes_iter.fold((true, 0), |(acc, p), g| {
            let tst_gen = mutated_zgt_chars.next().unwrap();
            println!("g={}, tst_gen={}", g, tst_gen);
            (acc && if p == n_pos { true } else { g == tst_gen }, p + 1)
        });
        assert!(res)
    }

    #[quickcheck]
    fn to_and_from_str_genes(genes: Vec<Gen>) -> bool {
        if genes.is_empty() {
            true
        } else {
            let genes_str = format!("{}", VecGen::new(genes));
            let zgt = Zygote::from_str(&genes_str).unwrap();
            let back_str = format!("{}", zgt);
            let filtered_back: String = back_str.chars().filter(|c| !c.is_whitespace()).collect();
            let aligned_genes = format!("{:r>1$}", genes_str, filtered_back.len());
            println!("from: {}\nto  : {}", aligned_genes, filtered_back);
            aligned_genes == filtered_back
        }
    }

    #[quickcheck]
    fn not_modify_genes_outside_defined_pos(genes: Vec<Gen>, pos: usize, new_gen: Gen) -> bool {
        if genes.is_empty() {
            true
        } else {
            let n_pos = pos % genes.len();
            let genes_str = &format!("{}", VecGen::new(genes));
            let zgt = Zygote::from_str(genes_str).unwrap();
            let mutated_zgt = zgt.mutate(n_pos, &new_gen);
            let mutated_zgt_str: String = format!("{}", mutated_zgt)
                .chars()
                .filter(|c| !c.is_whitespace())
                .collect();
            let mut mutated_zgt_chars = mutated_zgt_str.chars().rev();
            let normilazed = format!("{:r>1$}", genes_str, mutated_zgt_str.len());
            let chars: Vec<char> = normilazed.chars().collect();
            println!(
                "genes_str  ={}",
                u64s::group_by_u64_and_byte_pos(&chars)
                    .iter()
                    .collect::<String>()
            );
            println!("mutated_zgt={}", mutated_zgt);
            let genes_iter = genes_str.chars().rev();
            let (res, _) = genes_iter.fold((true, 0), |(acc, p), g| {
                let tst_gen = mutated_zgt_chars.next().unwrap();
                //                println!("g={}, tst_gen={}",g, tst_gen);
                (
                    acc &&
                        if p == n_pos {
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
    }

    #[quickcheck]
    fn modify_gen_defined_by_pos(genes: Vec<Gen>, pos: usize, new_gen: Gen) -> bool {
        if genes.is_empty() {
            true
        } else {
            let n_pos = pos % genes.len();
            let genes_str = &format!("{}", VecGen::new(genes));
            let zgt = Zygote::from_str(genes_str).unwrap();
            let mutated_zgt = zgt.mutate(n_pos, &new_gen);
            let mutated_zgt_str: String = format!("{}", mutated_zgt)
                .chars()
                .filter(|c| !c.is_whitespace())
                .collect();
            let mut mutated_zgt_chars = mutated_zgt_str.chars().rev();
            let normilazed = format!("{:r>1$}", genes_str, mutated_zgt_str.len());
            let chars: Vec<char> = normilazed.chars().collect();
            println!(
                "genes_str  ={}",
                u64s::group_by_u64_and_byte_pos(&chars)
                    .iter()
                    .collect::<String>()
            );
            println!("mutated_zgt={}", mutated_zgt);
            let genes_iter = genes_str.chars().rev();
            let (res, _) = genes_iter.fold((true, 0), |(acc, p), g| {
                let tst_gen = mutated_zgt_chars.next().unwrap();
                (
                    acc &&
                        if p == n_pos {
                            if tst_gen == Gen::to_char(&new_gen) {
                                true
                            } else {
                                println!("{}!={}", g, tst_gen);
                                false
                            }
                        } else {
                            true
                        },
                    p + 1,
                )
            });
            res
        }
    }

    #[test]
    fn cross_parts() {
        let crossed: Result<Zygote, String> =
            result! {
                let mut zgt1 <- Zygote::from_str("dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd");
                let mut zgt2 <- Zygote::from_str("rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr");
                zgt1.cross(&mut zgt2, 3, 4, true);
                zgt1
        };
        assert_eq!(
            crossed.unwrap().to_string(),
            "dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd drrr rddd"
        )
    }

    #[test]
    fn cross_parts_when_end_pos_bigger_than_size() {
        let crossed: Result<Zygote, String> =
            result! {
                    let mut zgt1 <- Zygote::from_str("dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd dddd");
                    let mut zgt2 <- Zygote::from_str("rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr");
                    zgt1.cross(&mut zgt2, 3, 100, true);
                    zgt1
            };
        assert_eq!(
            crossed.unwrap().to_string(),
            "rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rrrr rddd"
        )
    }
}
