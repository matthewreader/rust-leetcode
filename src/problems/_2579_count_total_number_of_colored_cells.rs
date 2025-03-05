struct Solution;

impl Solution {
    pub fn colored_cells(n: i32) -> i64 {
        let n64 :i64 = n as i64;
        1 + 2 * (n64 - 1) * n64
    }
}

#[test]
fn test() {
    let n = 1;
    assert_eq!(Solution::colored_cells(n), 1);

    let n = 2;
    assert_eq!(Solution::colored_cells(n), 5);

    let n = 3;
    assert_eq!(Solution::colored_cells(n), 13);

    // Check for overflow issues
    let n = 69675;
    assert_eq!(Solution::colored_cells(n), 9709071901);
}
