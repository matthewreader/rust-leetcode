use std::collections::HashMap;

struct Solution;

impl Solution {
    fn number_of_substrings(s: String) -> i32 {
        let mut char_count: HashMap<char, usize> = HashMap::new();
        let mut left = 0;
        let mut result = 0;
        let chars: Vec<char> = s.chars().collect();

        for right in 0..s.len() {
            // Expand the window by adding the right character
            *char_count.entry(chars[right]).or_insert(0) += 1;

            // Shrink the window from the left while it's valid (contains 'a', 'b', and 'c')
            while char_count.len() == 3 {
                result += s.len() - right; // All substrings from `left` to `right` are valid

                // Remove the left character from the count and shrink window
                let count = char_count.get_mut(&chars[left]).unwrap();
                *count -= 1;
                if *count == 0 {
                    char_count.remove(&chars[left]); // Remove the key if count reaches 0
                }
                left += 1;
            }
        }
        result as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
        let s = "abcabc".to_string();
        let result = 10;
        assert_eq!(Solution::number_of_substrings(s), result);
    }

    #[test]
    fn test_2() {
        let s = "aaacb".to_string();
        let result = 3;
        assert_eq!(Solution::number_of_substrings(s), result);
    }

    #[test]
    fn test_3() {
        let s = "abc".to_string();
        let result = 1;
        assert_eq!(Solution::number_of_substrings(s), result);
    }
}
