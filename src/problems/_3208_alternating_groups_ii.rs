struct Solution;

impl Solution {
    pub fn number_of_alternating_groups(colors: Vec<i32>, k: i32) -> i32 {
        let mut temp = colors.clone();
        temp.extend_from_slice(&colors[..(k as usize - 1)]);

        let mut count = 0;
        let mut left = 0;

        for right in 0..temp.len() {
            if right > 0 && temp[right] == temp[right - 1] {
                left = right;
            }

            if right - left + 1 >= k as usize {
                count += 1;
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn case_1() {
        let colors = vec![0,1,0,1,0];
        let k = 3;
        let result = 3;
        assert_eq!(Solution::number_of_alternating_groups(colors, k), result);
    }

    #[test]
    fn case_2() {
        let colors = vec![0,1,0,0,1,0,1];
        let k = 6;
        let result = 2;
        assert_eq!(Solution::number_of_alternating_groups(colors, k), result);
    }

    #[test]
    fn case_3() {
        let colors = vec![1,1,0,1];
        let k = 4;
        let result = 0;
        assert_eq!(Solution::number_of_alternating_groups(colors, k), result);
    }

    #[test]
    fn case_489() {
        let colors = vec![0,1,0,1];
        let k = 3;
        let result = 4;
        assert_eq!(Solution::number_of_alternating_groups(colors, k), result);
    }
}