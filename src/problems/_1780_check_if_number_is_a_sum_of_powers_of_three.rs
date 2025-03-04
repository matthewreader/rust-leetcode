struct Solution;

// Check if number can be re-written as base 3.  If any digit is greater than 1, return false.
impl Solution {
    pub fn check_powers_of_three(n: i32) -> bool {
        let mut n = n;
        while n > 0 {
            if n % 3 > 1 {
                return false;
            }
            n /= 3;
        }
        true
    }
}

#[test]
fn test() {
    assert_eq!(Solution::check_powers_of_three(12), true);
    assert_eq!(Solution::check_powers_of_three(91), true);
    assert_eq!(Solution::check_powers_of_three(21), false);
}