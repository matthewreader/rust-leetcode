struct Solution;

// Arithmetic solution
impl Solution {
    pub fn find_missing_and_repeated_values(grid: Vec<Vec<i32>>) -> Vec<i32> {
        // Input grid is assumed to be 2D square matrix
        // Need to use i64 to prevent overflow for some larger test cases.
        let n = (grid.len() * grid.len()) as i64;

        let sum_n = n * (n + 1) / 2;
        let ss_n = n * (n + 1) * (2 * n + 1) / 6;

        let mut arr_sum :i64 = 0;
        let mut arr_ss :i64 = 0;

        for row in &grid {
            for &num in row {
                arr_sum += num as i64;
                arr_ss += (num as i64) * (num as i64);
            }
        }

        let sum_difference = sum_n - arr_sum;
        let ss_difference = ss_n - arr_ss;

        let sum_xy = ss_difference / sum_difference;
        let missing = (sum_xy + sum_difference) / 2;
        let repeating = missing - sum_difference;

        vec![repeating as i32, missing as i32]
    }
}

#[test]
fn test() {
    assert_eq!(Solution::find_missing_and_repeated_values(vec![
        vec![1, 3],
        vec![2, 2]
    ]), vec![2, 4]);
    assert_eq!(Solution::find_missing_and_repeated_values(vec![
        vec![9, 1, 7],
        vec![8, 9, 2],
        vec![3, 4, 6],
    ]), vec![9, 5]);
}