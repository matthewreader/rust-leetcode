struct Solution;

impl Solution {
    fn shortest_common_supersequence(str1: String, str2: String) -> String {
        let (m, n) = (str1.len(), str2.len());
        let s1 = str1.as_bytes();
        let s2 = str2.as_bytes();

        // Step 1: Compute LCS using DP
        let mut dp = vec![vec![0; n + 1]; m + 1];

        for i in 1..=m {
            for j in 1..=n {
                if s1[i - 1] == s2[j - 1] {
                    dp[i][j] = 1 + dp[i - 1][j - 1];
                } else {
                    dp[i][j] = dp[i - 1][j].max(dp[i][j - 1]);
                }
            }
        }

        // Step 2: Backtrack to construct the SCS
        let (mut i, mut j) = (m, n);
        let mut scs = Vec::new();

        while i > 0 && j > 0 {
            if s1[i - 1] == s2[j - 1] {
                scs.push(s1[i - 1]); // Common character
                i -= 1;
                j -= 1;
            } else if dp[i - 1][j] > dp[i][j - 1] {
                scs.push(s1[i - 1]);
                i -= 1;
            } else {
                scs.push(s2[j - 1]);
                j -= 1;
            }
        }

        while i > 0 {
            scs.push(s1[i - 1]);
            i -= 1;
        }
        while j > 0 {
            scs.push(s2[j - 1]);
            j -= 1;
        }

        scs.reverse();
        String::from_utf8(scs).unwrap()
    }
}

#[test]
fn test() {
    assert_eq!(Solution::shortest_common_supersequence("abac".to_string(), "cab".to_string()), "cabac");
    assert_eq!(Solution::shortest_common_supersequence("aaaaaa".to_string(), "aaaaaa".to_string()), "aaaaaa");
}
