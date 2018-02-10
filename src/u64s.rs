use std::fmt;
use std::str;
use std;

pub trait U64s {
    fn cross_bits(&mut self, that: &Self, from: usize, amount: usize);

    fn cross_bits_bidirectional(&mut self, that: &mut Self, from: usize, amount: usize);

    fn overwrite(&mut self, source: &U64sStruct);

    fn get(&self, pos: usize) -> bool;

    fn set(&mut self, pos: usize, b: bool);

    fn len(&self) -> usize;

    fn is_empty(&self) -> bool;

    fn u64s_amount(&self) -> usize;

    fn get_u64(&self, p: usize) -> u64;

    fn new(u64s: Vec<u64>) -> Self;
}

impl fmt::Display for U64sStruct {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut string = String::new();
        for l in &self.u64s {
            let binary = format!("{:b}", l);
            let aligned = format!("{:0>1$}", binary, 64);
            string = format!("{}{}", aligned, string);
        }
        write!(f, "{}", string)
    }
}

impl fmt::Debug for U64sStruct {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl str::FromStr for U64sStruct {
    type Err = String;

    fn from_str(s: &str) -> Result<U64sStruct, String> {
        let mut filtered: String = s.chars().filter(|c| !c.is_whitespace()).rev().collect();

        let mut u64s = Vec::with_capacity(filtered.len() / 64);

        while !filtered.is_empty() {
            let filtered_tmp = filtered.clone();
            let len = filtered_tmp.len();
            let splitted = if len < 64 {
                (filtered_tmp.as_str(), "")
            } else {
                filtered_tmp.split_at(64)
            };
            let x = splitted.0;
            let x2: String = x.chars().rev().collect();
            let formatted = format!("{:0>1$}", x2, 64);
            u64s.push(bits_to_u(formatted.as_str()));
            filtered = splitted.1.to_string();
        }
        Ok(U64sStruct { u64s: u64s.to_vec() })
    }
}

fn bits_to_u(s: &str) -> u64 {
    let mut num = 0u64;
    let mut one = 1u64;
    for ch in s.chars().rev() {
        match ch {
            '1' => num |= one,
            '0' => (),
            _ => panic!(format!("{}", ch)),
        };
        one <<= 1;
    }
    num
}

#[derive(Clone)]
pub struct U64sStruct {
    u64s: Vec<u64>,
}

impl U64s for U64sStruct {
    fn new(u64s: Vec<u64>) -> Self {
        Self { u64s }
    }

    fn overwrite(&mut self, source: &U64sStruct) {
        self.u64s.clone_from_slice(&source.u64s);
    }

    fn get_u64(&self, p: usize) -> u64 {
        self.u64s[p]
    }

    fn u64s_amount(&self) -> usize {
        self.u64s.len()
    }

    fn cross_bits(&mut self, that: &U64sStruct, from: usize, amount: usize) {
        if amount > 0 {
            let CopyParams {
                first_bits_amount,
                p_byte_from,
                shift_from,
                shift_to,
                full_bytes_from,
                p_byte_to,
                full_bytes_to,
                full_bytes_present,
                from_present,
                to_present,
            } = self.calc_copy_params(from, amount);

            if from_present {
                cross_first_u64(
                    &mut self.u64s,
                    &that.u64s,
                    shift_from,
                    p_byte_from,
                    first_bits_amount,
                );
            };

            if full_bytes_present {
                cross_middle_u64s(&mut self.u64s, &that.u64s, full_bytes_from, full_bytes_to);
            };

            if to_present {
                cross_last_u64(&mut self.u64s, &that.u64s, p_byte_to, shift_to);
            };
        }
    }

    fn cross_bits_bidirectional(&mut self, that: &mut U64sStruct, from: usize, amount: usize) {
        if amount > 0 {
            let CopyParams {
                first_bits_amount,
                p_byte_from,
                shift_from,
                shift_to,
                full_bytes_from,
                p_byte_to,
                full_bytes_to,
                full_bytes_present,
                from_present,
                to_present,
            } = self.calc_copy_params(from, amount);

            if from_present {
                cross_first_u64_bidirectional(
                    &mut self.u64s,
                    &mut that.u64s,
                    shift_from,
                    p_byte_from,
                    first_bits_amount,
                );
            };

            if full_bytes_present {
                cross_middle_u64s_bidirectional(
                    &mut self.u64s,
                    &mut that.u64s,
                    full_bytes_from,
                    full_bytes_to,
                );
            };

            if to_present {
                cross_last_u64_bidirectional(&mut self.u64s, &mut that.u64s, p_byte_to, shift_to);
            };
        }
    }
    fn get(&self, pos: usize) -> bool {
        (self.u64s[pos / 64] & u64_mask(pos)) != 0
    }
    fn set(&mut self, pos: usize, b: bool) {
        let mask = u64_mask(pos);
        if b {
            self.u64s[pos / 64] |= mask
        } else {
            self.u64s[pos / 64] &= !mask
        }
    }
    fn len(&self) -> usize {
        self.u64s.len()
    }
    fn is_empty(&self) -> bool {
        self.u64s.is_empty()
    }
}

fn u64_mask(p: usize) -> u64 {
    1u64 << (p % 64)
}

fn cross_middle_u64s(array_to: &mut Vec<u64>, array_from: &[u64], from: usize, to: usize) {
    let mut pos = from;
    while pos < to {
        let tmp = array_from[pos];
        array_to[pos] = tmp;
        pos += 1;
    }
}

fn cross_middle_u64s_bidirectional(
    array_to: &mut Vec<u64>,
    array_from: &mut Vec<u64>,
    from: usize,
    to: usize,
) {
    let mut pos = from;
    while pos < to {
        std::mem::swap(&mut array_from[pos], &mut array_to[pos]);
        pos += 1;
    }
}

fn cross_first_u64(
    array_to: &mut Vec<u64>,
    array_from: &[u64],
    from_bit: usize,
    pos: usize,
    bits_amount: usize,
) -> CrossParams {
    let mask = (!0u64 >> (64 - bits_amount)) << from_bit;
    let int_from = array_from[pos];
    let int_to = array_to[pos];
    let masked_from = int_from & mask;
    array_to[pos] = int_to & !mask | masked_from;
    CrossParams {
        mask,
        int_from,
        int_to,
    }
}

fn cross_first_u64_bidirectional(
    array_to: &mut Vec<u64>,
    array_from: &mut Vec<u64>,
    from_bit: usize,
    pos: usize,
    bits_amount: usize,
) {
    let CrossParams {
        mask,
        int_from,
        int_to,
    } = cross_first_u64(array_to, array_from, from_bit, pos, bits_amount);
    let masked_to = int_to & mask;
    array_from[pos] = int_from & !mask | masked_to;
}

fn cross_last_u64(
    array_to: &mut Vec<u64>,
    array_from: &[u64],
    pos: usize,
    bits_amount: usize,
) -> CrossParams {
    let mask = !0u64 >> (64 - bits_amount);
    let int_from = array_from[pos];
    let int_to = array_to[pos];
    let last_masked_from = int_from & mask;
    array_to[pos] = int_to & !mask | last_masked_from;
    CrossParams {
        mask,
        int_from,
        int_to,
    }
}

fn cross_last_u64_bidirectional(
    array_to: &mut Vec<u64>,
    array_from: &mut Vec<u64>,
    pos: usize,
    bits_amount: usize,
) {
    let CrossParams {
        mask,
        int_from,
        int_to,
    } = cross_last_u64(array_to, array_from, pos, bits_amount);
    let last_masked_to = int_to & mask;
    array_from[pos] = int_from & !mask | last_masked_to;
}

trait Grouped
where
    Self: std::marker::Sized,
{
    fn grouped(&self, p: usize) -> Vec<Self>;
}

impl Grouped for Vec<char> {
    fn grouped(&self, p: usize) -> Vec<Vec<char>> {
        let mut vec = Vec::new();
        let mut pos = 0;
        let mut gathered = Vec::new();
        for &ch in self {
            if pos == p {
                vec.push(gathered.clone());
                gathered = Vec::new();
                gathered.push(ch);
                pos = 1;
            } else {
                gathered.push(ch);
                pos += 1;
            }
        }
        vec.push(gathered);

        vec
    }
}


pub fn group_by_u64_and_byte_pos(s: &[char]) -> Vec<char> {
    let new_str = s.to_vec().grouped(64).iter().fold(
        Vec::new(),
        |mut acc, v| {
            let b = v.grouped(4).into_iter().fold(Vec::new(), |mut acc2, v2| {
                acc2.push(v2);
                acc2.push(vec![' ']);
                acc2
            });
            acc.append(&mut b.clone());
            acc.append(&mut vec![vec![' ']]);
            acc
        },
    );
    let mut new_vec = Vec::new();
    for v in new_str {
        let mut cp = v.clone();
        new_vec.append(&mut cp);
    }
    new_vec
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;
    use std::iter;
    use gen::Gen;
    use quickcheck;

    fn cross(to: &Vec<char>, from: &Vec<char>, begin: usize, amount: usize) -> Vec<char> {
        let (h1, _, t1) = split_genes(&to, begin, amount);
        let (_, m2, _) = split_genes(&from, begin, amount);
        let mut vec: Vec<char> = Vec::with_capacity(to.len());
        vec.extend_from_slice(h1);
        vec.extend_from_slice(m2);
        vec.extend_from_slice(t1);
        vec
    }

    fn split_genes<'a>(
        genes: &'a Vec<char>,
        begin: usize,
        amount: usize,
    ) -> (&'a [char], &'a [char], &'a [char]) {
        fn normalize_pos(p: usize, len: usize) -> usize {
            if p > len - 1 { len - 1 } else { p }
        }
        let n_begin = normalize_pos(begin, genes.len());
        let (head, tail) = genes.split_at(n_begin);
        let n_end = normalize_pos(amount, tail.len());
        let (head_of_tail, tail_of_tail) = tail.split_at(n_end);
        (head, head_of_tail, tail_of_tail)
    }

    impl quickcheck::Arbitrary for Gen {
        fn arbitrary<G: quickcheck::Gen>(g: &mut G) -> Self {
            g.gen()
        }

        fn shrink(&self) -> Box<Iterator<Item = Self>> {
            Box::new(iter::empty())
        }
    }

    fn to_str(genes: Vec<u64>) -> String {
        genes
            .iter()
            .map(|u| {
                let b = format!("{:b}", u);
                format!("{:0>1$}", b, 64)
            })
            .collect()
    }

    #[test]
    fn two_u64s() {
        let from = "10000000000000000000000000000000000000000000000000000000000000110";
        let ls = U64sStruct::from_str(&from).unwrap();
        let result = format!("{}", ls);
        let fmt_source = format!("{:0>1$}", from, result.len());
        assert_eq!(fmt_source, result)
    }

    #[test]
    fn two_u64s_set() {
        let from = "01000000000000000000000000000000000000000000000000000000000000110";
        let mut ls = U64sStruct::from_str(&from).unwrap();
        ls.set(0, true);
        let result = format!("{}", ls);
        let expected_result = "00000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000111";
        assert_eq!(expected_result, result)
    }

    #[quickcheck]
    fn from_str_to_str_should_return_original_str(gens: Vec<Gen>) -> bool {
        let mut source: String = gens.iter()
            .map(|g| match g {
                &Gen::D1 | &Gen::D0 => '1',
                _ => '0',
            })
            .collect();
        source = if source.len() == 0 {
            "0".to_string()
        } else {
            source
        };
        let ls = U64sStruct::from_str(&source).unwrap();
        let result = format!("{}", ls);
        let fmt_source = format!("{:0>1$}", source, result.len());
        fmt_source == result
    }

    //    #[test]
    fn test2() {
        let start_bit = 2;
        let bits_amount = 1;

        let from: Vec<char> = to_str(vec![0, 0, 0, 0]).chars().collect();
        let to: Vec<char> = to_str(vec![!0u64, !0u64, !0u64, !0u64]).chars().collect();

        let from_rev = from.iter().rev().map(|c| c.clone()).collect();
        let to_rev = to.iter().rev().map(|c| c.clone()).collect();

        let crossed_str_to: Vec<char> = group_by_u64_and_byte_pos(
            &cross(&to_rev, &from_rev, start_bit, bits_amount),
        ).into_iter()
            .filter(|c| !c.is_whitespace())
            .rev()
            .map(|c| c.clone())
            .collect();
        let crossed_str_from: Vec<char> = group_by_u64_and_byte_pos(
            &cross(&from_rev, &to_rev, start_bit, bits_amount),
        ).into_iter()
            .filter(|c| !c.is_whitespace())
            .rev()
            .map(|c| c.clone())
            .collect();
        let from_str: String = from.into_iter().collect();
        let mut line_from = from_str.parse::<U64sStruct>().unwrap();
        let to_str: String = to.into_iter().collect();
        let mut line_to = to_str.parse::<U64sStruct>().unwrap();
        line_to.cross_bits_bidirectional(&mut line_from, start_bit, bits_amount);
        let result_str_to = format!("{}", line_to);
        let result_str_from = format!("{}", line_from);
        let from_str2: String = crossed_str_from.into_iter().collect();
        let to_str2: String = crossed_str_to.into_iter().collect();
        assert!(result_str_to == to_str2 && result_str_from == from_str2)
    }

    #[quickcheck]
    fn test(
        genes_from: Vec<u64>,
        genes_to: Vec<u64>,
        start_bit: usize,
        bits_amount: usize,
    ) -> bool {
        //        env::set_var("QUICKCHECK_MAX_TESTS", "1000000");
        //        env::set_var("QUICKCHECK_TESTS", "1000000");
        //        env::set_var("QUICKCHECK_GENERATOR_SIZE", "10000");

        if genes_from.len() > 0 && bits_amount > 0 && genes_from.len() == genes_to.len() {
            let from: Vec<char> = to_str(genes_from).chars().collect();
            let to: Vec<char> = to_str(genes_to).chars().collect();

            let from_rev = from.iter().rev().map(|c| c.clone()).collect();
            let to_rev = to.iter().rev().map(|c| c.clone()).collect();

            let crossed_str_to: Vec<char> = group_by_u64_and_byte_pos(
                &cross(&to_rev, &from_rev, start_bit, bits_amount),
            ).into_iter()
                .filter(|c| !c.is_whitespace())
                .rev()
                .map(|c| c.clone())
                .collect();
            let crossed_str_from: Vec<char> = group_by_u64_and_byte_pos(
                &cross(&from_rev, &to_rev, start_bit, bits_amount),
            ).into_iter()
                .filter(|c| !c.is_whitespace())
                .rev()
                .map(|c| c.clone())
                .collect();
            let from_str: String = from.into_iter().collect();
            let mut line_from = from_str.parse::<U64sStruct>().unwrap();
            let to_str: String = to.into_iter().collect();
            let mut line_to = to_str.parse::<U64sStruct>().unwrap();
            line_to.cross_bits_bidirectional(&mut line_from, start_bit, bits_amount);
            let result_str_to = format!("{}", line_to);
            let result_str_from = format!("{}", line_from);
            let from_str2: String = crossed_str_from.into_iter().collect();
            let to_str2: String = crossed_str_to.into_iter().collect();
            result_str_to == to_str2 && result_str_from == from_str2
        } else {
            true
        }
    }
}

struct CopyParams {
    first_bits_amount: usize,
    p_byte_from: usize,
    shift_from: usize,
    shift_to: usize,
    full_bytes_from: usize,
    p_byte_to: usize,
    full_bytes_to: usize,
    full_bytes_present: bool,
    from_present: bool,
    to_present: bool,
}

struct CrossParams {
    mask: u64,
    int_from: u64,
    int_to: u64,
}

impl U64sStruct {
    fn calc_copy_params(&self, from: usize, amount: usize) -> CopyParams {
        let size = self.u64s.len() * 64;
        let n_from = if from > size { size } else { from };
        let n_bits_amount = if size - n_from < amount {
            size - n_from
        } else {
            amount
        };
        let first_bits_amount = if (64 - n_from % 64) >= n_bits_amount {
            n_bits_amount
        } else {
            64 - n_from % 64
        };
        let after_last_bit = n_from + n_bits_amount;
        let p_byte_from = n_from / 64;
        let shift_from = n_from % 64;
        let shift_to = after_last_bit % 64;
        let full_bytes_from = if shift_from > 0 {
            p_byte_from + 1
        } else {
            p_byte_from
        };
        let p_byte_to = (after_last_bit - 1) / 64;
        let full_bytes_to = if shift_to != 0 {
            p_byte_to
        } else {
            p_byte_to + 1
        };
        let full_bytes_present = (n_bits_amount + shift_from) / 64 > 0 ||
            shift_from == 0 && n_bits_amount / 64 > 0;
        let from_present = if full_bytes_present && n_from % 64 == 0 {
            false
        } else {
            shift_to > 0 || n_from % 64 > 0
        };
        let to_present = after_last_bit / 64 > 0 && shift_to > 0 && p_byte_from != p_byte_to;
        CopyParams {
            first_bits_amount,
            p_byte_from,
            shift_from,
            shift_to,
            full_bytes_from,
            p_byte_to,
            full_bytes_to,
            full_bytes_present,
            from_present,
            to_present,
        }
    }
}
