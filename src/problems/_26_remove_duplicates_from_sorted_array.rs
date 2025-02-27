struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.len() == 0 {
            return 0
        }

        let mut j = 0;
        for i in 1..nums.len() {
            if nums[i] != nums[j] {
                j += 1;
                nums[j] = nums[i]
            }
        }
        (j + 1) as i32
    }
}

#[test]
fn test() {
    let mut nums = vec![1, 1, 2];
    assert_eq!(Solution::remove_duplicates(&mut nums), 2);

    let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
    assert_eq!(Solution::remove_duplicates(&mut nums), 5);
}
