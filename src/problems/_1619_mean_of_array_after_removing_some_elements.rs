/*
Given an integer array arr, return the mean of the remaining integers after removing the smallest 5%
 and the largest 5% of the elements.

Answers within 10-5 of the actual answer will be considered accepted.
 */
struct Solution;

impl Solution {
    pub fn trim_mean(arr: Vec<i32>) -> f64 {
        let trim_length = arr.len() / 20;
        let mut arr = arr;

        arr.sort();
        arr.drain(0..trim_length);
        arr.drain(arr.len() - trim_length..);

        arr.iter().sum::<i32>() as f64 / arr.len() as f64
    }
}

#[test]
fn test() {
    assert_eq!(Solution::trim_mean(vec![1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 3]),
               2.0);
    assert_eq!(Solution::trim_mean(vec![6,2,7,5,1,2,0,3,10,2,5,0,5,5,0,8,7,6,8,0]),
               4.0);
    assert_eq!(Solution::trim_mean(vec![6,0,7,0,7,5,7,8,3,4,0,7,8,1,6,8,1,1,2,4,8,1,9,5,4,3,8,5,10,8,6,6,1,0,6,10,8,2,3,4]),
               4.777777777777778);
}