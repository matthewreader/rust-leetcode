struct Solution;

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut i = 0;
        while i < nums.len() {
            if nums[i] == val {
                nums.remove(i);
            } else {
                i += 1;
            }
        }
        nums.len() as i32
    }
}

#[test]
fn test() {
    let mut nums = vec![3, 2, 2, 3];
    assert_eq!(Solution::remove_element(&mut nums, 3), 2);
    assert_eq!(nums, vec![2, 2]);

    let mut nums = vec![0, 1, 2, 2, 3, 0, 4, 2];
    assert_eq!(Solution::remove_element(&mut nums, 2), 5);
    assert_eq!(nums, vec![0, 1, 3, 0, 4]);
}