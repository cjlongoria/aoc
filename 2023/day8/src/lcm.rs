pub fn lcm(numbers: &[u64]) -> u64 {
    numbers
        .iter()
        .fold(1, |acc, &num| (acc * num) / gcd(acc, num))
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    let mut tmp;
    while b != 0 {
        tmp = a;
        a = b;
        b = tmp % b;
    }
    a
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gcd_8_12() {
        let res = gcd(8, 12);
        assert_eq!(res, 4);
    }

    #[test]
    fn gcd_12_8() {
        let res = gcd(12, 8);
        assert_eq!(res, 4);
    }

    #[test]
    fn lcm_12_8() {
        let res = lcm(&[12, 8]);
        assert_eq!(res, 24);
    }
}

