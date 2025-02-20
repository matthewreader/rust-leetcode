/*
Given two arrays arr1 and arr2, the elements of arr2 are distinct, and all elements in arr2 are also
in arr1.

Sort the elements of arr1 such that the relative ordering of items in arr1 are the same as in arr2.
Elements that do not appear in arr2 should be placed at the end of arr1 in ascending order.
 */

struct Solution;

impl Solution {
    pub fn relative_sort_array(arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
        let mut arr1 = arr1;
        let mut result = Vec::new();
        for i in 0..arr2.len() {
            let mut j = 0;
            while j < arr1.len() {
                if arr1[j] == arr2[i] {
                    result.push(arr1.remove(j));
                } else {
                    j += 1;
                }
            }
        }
        arr1.sort();
        result.extend(arr1);
        result
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::relative_sort_array(
            vec![2, 3, 1, 3, 2, 4, 6, 7, 9, 2, 19],
            vec![2, 1, 4, 3, 9, 6]
        ),
        vec![2, 2, 2, 1, 4, 3, 3, 9, 6, 7, 19]
    );
    assert_eq!(
        Solution::relative_sort_array(
            vec![28, 6, 22, 8, 44, 17],
            vec![22, 28, 8, 6]
        ),
        vec![22, 28, 8, 6, 17, 44]
    );
}
