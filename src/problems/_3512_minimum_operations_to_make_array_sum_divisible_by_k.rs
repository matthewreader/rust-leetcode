struct Solution;

impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        let sum = nums.iter().sum::<i32>();

        if k > sum {
            return sum;
        }

        if sum % k == 0 {
            return 0;
        }

        let mut total_ops: i32 = 0;
        for i in 1..=sum {
            total_ops += 1;
            if (sum - i) % k == 0 {
                return total_ops;
            }
        }
        total_ops
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
        let arr = [3, 9, 7];
        let k = 5;
        let result = 4;
        assert_eq!(Solution::min_operations(Vec::from(arr), k), result);
    }

    #[test]
    fn test_2() {
        let arr = [4, 1, 3];
        let k = 4;
        let result = 0;
        assert_eq!(Solution::min_operations(Vec::from(arr), k), result);
    }

    #[test]
    fn test_3() {
        let arr = [3, 2];
        let k = 6;
        let result = 5;
        assert_eq!(Solution::min_operations(Vec::from(arr), k), result);
    }

    #[test]
    fn test_852() {
        let arr = [9, 5];
        let k = 14;
        let result = 0;
        assert_eq!(Solution::min_operations(Vec::from(arr), k), result);
    }
}