/*
You are given a string s. The score of a string is defined as the sum of the absolute difference
between the ASCII values of adjacent characters.

Return the score of s.
 */

struct Solution;

impl Solution {
    pub fn score_of_string(s: String) -> i32 {
        let mut score = 0;
        let s = s.as_bytes();
        for i in 1..s.len() {
            score += (s[i] as i32 - s[i - 1] as i32).abs();
        }
        score
    }
}

#[test]
fn test() {
    assert_eq!(Solution::score_of_string("hello".to_string()), 13);
    assert_eq!(Solution::score_of_string("aa".to_string()), 0);
    assert_eq!(Solution::score_of_string("aba".to_string()), 2);
    assert_eq!(Solution::score_of_string("a".to_string()), 0);
}