// Valid word has the following properties:
// 1. It contains a minimum of 3 characters.
// 2. It contains at least one vowel (a, e, i, o, u).
// 3. It contains only digits and English letters (both uppercase and lowercase).
// 4. It contains at least one consonant.
struct Solution;

impl Solution {
    pub fn is_valid(word: String) -> bool {
        let mut has_vowel = false;
        let mut has_consonant = false;
        let mut has_digit_or_letter = false;

        if word.len() < 3 {
            return false; // Rule 1
        }

        for c in word.chars() {
            if c.is_ascii_alphabetic() {
                has_digit_or_letter = true;
                if "aeiouAEIOU".contains(c) {
                    has_vowel = true; // Rule 2
                } else {
                    has_consonant = true; // Rule 4
                }
            } else if c.is_ascii_digit() {
                has_digit_or_letter = true; // Rule 3
            } else {
                return false; // Invalid character found
            }
        }

        has_vowel && has_consonant && has_digit_or_letter // All rules must be satisfied

    }
}