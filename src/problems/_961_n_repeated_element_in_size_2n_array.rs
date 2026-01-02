use std::collections::{HashSet};
struct Solution;

impl Solution {
    pub fn repeated_n_times(nums: Vec<i32>) -> i32 {
        let mut seen = HashSet::new();

        // Return the first duplicated value.  The value to return is repeated N times.
        // All other values cannot be repeated and still meet the criteria of the problem.
        for x in nums {
            if seen.contains(&x) {
                return x;
            }
            seen.insert(x);
        }
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
        let nums = vec![1,2,3,3];
        let result = 3;
        assert_eq!(Solution::repeated_n_times(nums), result);
    }

    #[test]
    fn test_2() {
        let nums = vec![2,1,2,5,3,2];
        let result = 2;
        assert_eq!(Solution::repeated_n_times(nums), result);
    }

    #[test]
    fn test_3() {
        let nums = vec![5,1,5,2,5,3,5,4];
        let result = 5;
        assert_eq!(Solution::repeated_n_times(nums), result);
    }
}