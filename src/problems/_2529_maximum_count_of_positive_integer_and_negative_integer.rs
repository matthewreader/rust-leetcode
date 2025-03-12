struct Solution;

impl Solution {
    pub fn maximum_count(nums: Vec<i32>) -> i32 {
        let count_neg = nums.partition_point(|&x| x < 0);
        let count_pos = nums.len() - nums.partition_point(|&x| x <= 0);
        count_neg.max(count_pos) as i32
    }
}