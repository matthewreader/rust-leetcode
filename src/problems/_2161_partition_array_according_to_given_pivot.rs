struct Solution;

// Iterate over the array and partition it into three parts: less than pivot, equal to pivot,
// and greater than pivot.  Then concatenate these three parts to get the final result.
impl Solution {
    pub fn pivot_array(nums: Vec<i32>, pivot: i32) -> Vec<i32> {
        let mut left = vec![];
        let mut right = vec![];
        let mut equal = vec![];

        for num in nums {
            if num < pivot {
                left.push(num);
            } else if num > pivot {
                right.push(num);
            } else {
                equal.push(num);
            }
        }

        left.append(&mut equal);
        left.append(&mut right);

        left

    }
}

#[test]
fn test() {
    assert_eq!(Solution::pivot_array(vec![9,12,5,10,14,3,10], 10), vec![9,5,3,10,10,12,14]);
    assert_eq!(Solution::pivot_array(vec![-3,4,3,2], 2), vec![-3,2,4,3]);
}