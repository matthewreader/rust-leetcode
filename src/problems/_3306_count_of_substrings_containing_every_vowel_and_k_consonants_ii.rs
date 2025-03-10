use std::collections::HashSet;

struct Solution;

impl Solution {
    fn at_least_k(word: &[char], k: usize) -> i64 {
        let vowels: HashSet<char> = ['a', 'e', 'i', 'o', 'u'].iter().cloned().collect();
        let mut vowel_count = [0; 26]; // Array for vowels instead of HashMap
        let mut vowel_types = 0; // Tracks how many unique vowels are present
        let mut consonant_count = 0;
        let mut num_valid_substrings = 0;
        let mut start = 0;

        for (end, &new_letter) in word.iter().enumerate() {
            // Add new letter to the window
            if vowels.contains(&new_letter) {
                let idx = (new_letter as u8 - b'a') as usize;
                if vowel_count[idx] == 0 {
                    vowel_types += 1; // We found a new vowel type
                }
                vowel_count[idx] += 1;
            } else {
                consonant_count += 1;
            }

            // Shrink window while maintaining conditions
            while vowel_types == 5 && consonant_count >= k {
                num_valid_substrings += word.len() - end;

                let start_letter = word[start];
                if vowels.contains(&start_letter) {
                    let idx = (start_letter as u8 - b'a') as usize;
                    vowel_count[idx] -= 1;
                    if vowel_count[idx] == 0 {
                        vowel_types -= 1;
                    }
                } else {
                    consonant_count -= 1;
                }
                start += 1;
            }
        }

        num_valid_substrings as i64
    }

    pub fn count_of_substrings(word: String, k: i32) -> i64 {
        let word_chars: Vec<char> = word.chars().collect(); // Convert to Vec<char> once
        Self::at_least_k(&word_chars, k as usize) - Self::at_least_k(&word_chars, (k + 1) as usize)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let word = "aeioqq".to_string();
        let k = 1;
        let result = 0;
        assert_eq!(Solution::count_of_substrings(word, k), result);
    }

    #[test]
    fn test_2() {
        let word = "aeiou".to_string();
        let k = 0;
        let result = 1;
        assert_eq!(Solution::count_of_substrings(word, k), result);
    }

    #[test]
    fn test_3() {
        let word = "ieaouqqieaouqq".to_string();
        let k = 1;
        let result = 3;
        assert_eq!(Solution::count_of_substrings(word, k), result);
    }

    #[test]
    fn large_test() {
        let word = "a".repeat(100000) + &"e".repeat(100000) + &"i".repeat(100000) + &"o".repeat(100000) + &"u".repeat(100000) + &"b".repeat(100000);
        let k = 1;
        assert!(Solution::count_of_substrings(word, k) > 0);
    }
}
