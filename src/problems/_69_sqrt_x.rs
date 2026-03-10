struct Solution;

impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {

        if x < 2 {
            return x;
        }

        let mut low: i64 = 1;
        let mut high: i64 = x as i64 / 2;
        let mut answer: i64 = 1;

        while low <= high {
            let mid = low + (high - low) / 2;
            let square = mid * mid;

            if square == x as i64 {
                return mid as i32;
            }
            else if square < x as i64 {
                answer = mid;
                low = mid + 1;
            }
            else {
                high = mid - 1;
            }
        }

        answer as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
        let x = 4;
        let result = 2;
        assert_eq!(Solution::my_sqrt(x), result);
    }

    #[test]
    fn test_2() {
        let x = 8;
        let result = 2;
        assert_eq!(Solution::my_sqrt(x), result);
    }

    #[test]
    fn test_3() {
        let x = 0;
        let result = 0;
        assert_eq!(Solution::my_sqrt(x), result);
    }

    #[test]
    fn test_4() {
        let x = 1;
        let result = 1;
        assert_eq!(Solution::my_sqrt(x), result);
    }

    #[test]
    fn test_1017() {
        let x = 2;
        let result = 1;
        assert_eq!(Solution::my_sqrt(x), result);
    }

    #[test]
    fn test_1018() {
        let x = 2147395599;
        let result = 46339;
        assert_eq!(Solution::my_sqrt(x), result);
    }
}