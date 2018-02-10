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


pub fn gray2bin(n: &u64) -> u64 {
    let mut r = (n >> 32) ^ n;
    r ^= r >> 16;
    r ^= r >> 8;
    r ^= r >> 4;
    r ^= r >> 2;
    r ^= r >> 1;
    r
}

pub fn decode_bits_to_u64s(bits: &[u64]) -> Vec<u64> {
    bits.iter().map(gray2bin).collect()
}

#[cfg(test)]
mod gray_to_bin {
    use super::*;

    #[test]
    fn from_1000_to_1111() {
        let res = gray2bin(&0b1000u64);
        assert_eq!(res, 0b1111u64);
    }

    #[test]
    fn from_1111_to_1010() {
        let res = gray2bin(&0b1111u64);
        assert_eq!(res, 0b1010u64);
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

#[cfg(test)]
mod decode_bits_to_u64s {
    use super::*;

    #[test]
    fn input_000000010_must_be_3() {
        assert_eq!(decode_bits_to_u64s(&vec![0b00000010u64]), [3])
    }

    #[test]
    fn input_10000000_must_be_255() {
        assert_eq!(decode_bits_to_u64s(&vec![0b10000000u64]), [255])
    }

    #[test]
fn input_00000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000011_must_be_32(){
        assert_eq!(
            decode_bits_to_u64s(&vec![
                    0b0000000000000000000000000000000000000000000000000000000000000010u64,
                    0b0000000000000000000000000000000000000000000000000000000000000011u64,
                ]),
            [3, 2]
        )
    }
}
