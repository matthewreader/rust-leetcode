struct Solution;

impl Solution {
    pub fn max_score_sightseeing_pair(values: Vec<i32>) -> i32 {
        let mut score = 0;
        let mut max_i = values[0];

        for j in 1..values.len() {
            score = score.max(max_i + values[j] - j as i32);
            max_i = max_i.max(values[j] + j as i32);
        }
        score
    }
}

#[test]
fn test() {
    assert_eq!(Solution::max_score_sightseeing_pair(vec![8, 1, 5, 2, 6]), 11);
    assert_eq!(Solution::max_score_sightseeing_pair(vec![1, 2]), 2);
}