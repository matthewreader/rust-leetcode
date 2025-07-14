#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

struct Solution;

impl Solution {
    pub fn get_decimal_value(head: Option<Box<ListNode>>) -> i32 {
        let mut result = 0;
        let mut current = head;

        while let Some(node) = current {
            result = (result << 1) | node.val;
            current = node.next;
        }
         result
    }
}

fn create_linked_list(nums: Vec<i32>) -> Option<Box<ListNode>> {
    if nums.is_empty() {
        return None;
    }

    let mut head = Box::new(ListNode::new(nums[0]));
    let mut current = &mut head;

    for &val in &nums[1..] {
        current.next = Some(Box::new(ListNode::new(val)));
        current = current.next.as_mut().unwrap();
    }

    Some(head)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        // Test case: [1,0,1] -> 5
        let head = create_linked_list(vec![1, 0, 1]);
        assert_eq!(Solution::get_decimal_value(head), 5);
    }

    #[test]
    fn test_example_2() {
        // Test case: [0] -> 0
        let head = create_linked_list(vec![0]);
        assert_eq!(Solution::get_decimal_value(head), 0);
    }
}