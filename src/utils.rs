use global_constants::*;

pub fn normalize_fitness(fitness: f64, min_fitness: f64, max_fitness: f64) -> f64 {
    let based_fitness = fitness - min_fitness;
    let fitness_range = max_fitness - min_fitness;
    if fitness < min_fitness {
        0.0
    } else if fitness_range != 0.0 {
        based_fitness / fitness_range
    } else {
        1.0
    }
}

pub fn gray2bin(n: &[bool]) -> Vec<bool> {
    (0..n.len()).fold(Vec::new(), |acc, pos| {
        let mut new_acc = acc.clone();
        new_acc.push(xor_until_pos(n, pos));
        new_acc
    })
}

fn decode_bools_to_u64(a: &[bool]) -> u64 {
    a.iter().fold(
        0u64,
        |acc, &b| (acc << 1) + (if b { 1 } else { 0 }),
    )
}

pub fn decode_bools_to_u64s(bits: &[bool]) -> Vec<u64> {
    let mut bits_from = bits.to_vec();
    let u64ss_size = bits.len() / U64_BITS_AMOUNT +
        (if bits.len() % U64_BITS_AMOUNT > 0 {
             1
         } else {
             0
         });
    let mut u64s = Vec::with_capacity(u64ss_size);
    u64s.resize(u64ss_size, 0);
    for pos in (1..u64ss_size + 1).rev() {
        let len = bits_from.len();
        let num_bits = if len < U64_BITS_AMOUNT {
            bits_from.clone()
        } else {
            bits_from.split_off(len - U64_BITS_AMOUNT)
        };
        u64s[pos - 1] = decode_bools_to_u64(&gray2bin(&num_bits));
    }
    u64s
}

fn xor_until_pos(n: &[bool], pos_to: usize) -> bool {
    n.iter().take(pos_to + 1).fold(false, |acc, b| acc ^ b)
}

#[cfg(test)]
mod gray_to_bin {
    use super::*;

    #[test]
    fn from_1000_to_1111() {
        let res = gray2bin(&[true, false, false, false]);
        assert_eq!(res, &[true, true, true, true]);
    }

    #[test]
    fn from_1111_to_1010() {
        let res = gray2bin(&[true, true, true, true]);
        assert_eq!(res, &[true, false, true, false]);
    }
}


#[cfg(test)]
mod decode_booleans_to_u64s {
    use super::*;

    #[test]
    fn input_000000010_must_be_3() {
        assert_eq!(
            decode_bools_to_u64s(&[false, false, false, false, false, false, true, false]),
            [3]
        )
    }

    #[test]
    fn input_10000000_must_be_255() {
        assert_eq!(
            decode_bools_to_u64s(&[true, false, false, false, false, false, false, false]),
            [255]
        )
    }

    #[test]
fn input_00000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000011_must_be_3_2(){
        let list_of_52_false = (0..62).fold(Vec::new(), |mut acc, _| {
            acc.push(false);
            acc
        });
        let mut booleans = Vec::new();
        booleans.append(&mut (list_of_52_false.clone()));
        booleans.push(true);
        booleans.push(false);
        booleans.append(&mut (list_of_52_false.clone()));
        booleans.push(true);
        booleans.push(true);
        assert_eq!(decode_bools_to_u64s(&booleans), [3, 2])
    }
}

#[cfg(test)]
mod normalize_fitness {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(normalize_fitness(0f64, 0f64, 0f64), 1f64);
        assert_eq!(normalize_fitness(0f64, 0f64, 1f64), 0f64);
        assert_eq!(normalize_fitness(1f64, 1f64, 1f64), 1f64);
        assert_eq!(normalize_fitness(0.5f64, 0f64, 1f64), 0.5f64);
        assert_eq!(normalize_fitness(1f64, 1f64, 2f64), 0f64);
        assert_eq!(normalize_fitness(2f64, 1f64, 2f64), 1f64);
        assert_eq!(normalize_fitness(-15f64, -20f64, -10f64), 0.5f64);
        assert_eq!(normalize_fitness(0f64, -10f64, 10f64), 0.5f64);
        assert_eq!(normalize_fitness(-1_000_000f64, -10f64, 10f64), 0f64);
    }
}
