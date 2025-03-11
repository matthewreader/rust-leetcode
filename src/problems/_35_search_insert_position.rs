struct Solution;

impl Solution {
    // Simple binary search
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut low :usize = 0;
        let mut high: usize = nums.len() as usize - 1;

        while low <= high {
            let mid = (low + high) / 2;
            let guess = nums[mid];

            if guess == target {
                return mid as i32
            } else if guess > target {
                if mid == 0 {
                    break;
                }
                high = mid - 1;
            } else {
                low = mid + 1;
            }
        }
        low as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
        let nums = vec![1,3,5,6];
        let target = 5;
        let result = 2;
        assert_eq!(Solution::search_insert(nums, target), result);
    }

    #[test]
    fn test_2() {
        let nums = vec![1,3,5,6];
        let target = 2;
        let result = 1;
        assert_eq!(Solution::search_insert(nums, target), result);
    }

    #[test]
    fn test_3() {
        let nums = vec![1,3,5,6];
        let target = 7;
        let result = 4;
        assert_eq!(Solution::search_insert(nums, target), result);
    }

    #[test]
    fn test_4() {
        let nums = vec![1,3,5,6];
        let target = 0;
        let result = 0;
        assert_eq!(Solution::search_insert(nums, target), result);
    }
}