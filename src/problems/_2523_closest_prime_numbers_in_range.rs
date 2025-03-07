struct Solution;

impl Solution {
    pub fn closest_primes(left: i32, right: i32) -> Vec<i32> {
        let max_n = right as usize;

        let mut is_prime = vec![true; max_n + 1];
        is_prime[0] = false;
        is_prime[1] = false;

        for i in 2..=((max_n as f64).sqrt() as usize) {
            if is_prime[i] {
                for j in (i * i..=max_n).step_by(i) {
                    is_prime[j] = false;
                }
            }
        }

        let mut primes_in_range = Vec::new();

        for i in left as usize..=right as usize {
            if is_prime[i] {
                primes_in_range.push(i as i32);
            }
        }

        if primes_in_range.len() < 2 {
            return vec![-1, -1];
        }

        let mut min_diff = i32::MAX;
        let mut closest_pair = vec![-1, -1];

        for i in 0..primes_in_range.len() - 1 {
            let diff = primes_in_range[i + 1] - primes_in_range[i];
            if diff < min_diff {
                min_diff = diff;
                closest_pair = vec![primes_in_range[i], primes_in_range[i + 1]];
            }
        }
        closest_pair
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let left = 10;
        let right = 20;
        let result = vec![11, 13];
        assert_eq!(Solution::closest_primes(left, right), result);
    }

    #[test]
    fn test_2() {
        let left = 4;
        let right = 10;
        let result = vec![5, 7];
        assert_eq!(Solution::closest_primes(left, right), result);
    }

    #[test]
    fn test_3() {
        let left = 11;
        let right = 13;
        let result = vec![11, 13];
        assert_eq!(Solution::closest_primes(left, right), result);
    }

    #[test]
    fn test_4() {
        let left = 3;
        let right = 3;
        let result = vec![-1, -1];
        assert_eq!(Solution::closest_primes(left, right), result);
    }
}


