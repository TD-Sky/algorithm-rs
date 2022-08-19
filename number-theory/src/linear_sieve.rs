use std::collections::BTreeSet;

// param: to >= 2
pub fn linear_sieve(to: u32) -> Vec<u32> {
    let mut composites = BTreeSet::new();
    let mut primes = Vec::new();

    for i in 2..=to {
        if !composites.contains(&i) {
            primes.push(i);
        }

        for &prime in primes.iter().take_while(|&p| i * p <= to) {
            composites.insert(i * prime);

            if i % prime == 0 {
                break;
            }
        }
    }

    primes
}

#[cfg(test)]
mod tests {
    use super::linear_sieve;

    #[test]
    fn front_100() {
        assert_eq!(
            linear_sieve(100),
            [
                2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79,
                83, 89, 97,
            ]
        );
    }

    #[test]
    fn illegal_usage() {
        assert_eq!(linear_sieve(0), []);
        assert_eq!(linear_sieve(1), []);
    }

    #[test]
    fn single_element() {
        assert_eq!(linear_sieve(2), [2]);
    }
}
