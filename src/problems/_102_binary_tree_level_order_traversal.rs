use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
   pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
   #[inline]
   pub fn new(val: i32) -> Self {
     TreeNode {
       val,
       left: None,
       right: None
     }
   }
}

struct Solution;

impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut ans: Vec<Vec<i32>> = Vec::new();
        let mut q: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();

        if let Some(r) = root {
            q.push_back(r);
        } else {
            return ans;
        }

        while !q.is_empty() {
            let level_size = q.len();
            let mut level: Vec<i32> = Vec::with_capacity(level_size);

            for _ in 0..level_size {
                let node_rc = q.pop_front().unwrap();
                let node = node_rc.borrow();

                level.push(node.val);

                if let Some(left) = node.left.clone() {
                    q.push_back(left);
                }
                if let Some(right) = node.right.clone() {
                    q.push_back(right);
                }
            }

            ans.push(level);
        }
        ans
    }
}

fn build_tree(data: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
    if data.is_empty() || data[0].is_none() {
        return None;
    }

    let root = Rc::new(RefCell::new(TreeNode::new(data[0].unwrap())));
    let mut queue: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();
    queue.push_back(root.clone());

    let mut i = 1;
    while i < data.len() {
        let current = queue.pop_front().unwrap();

        // left child
        if i < data.len() {
            if let Some(v) = data[i] {
                let left = Rc::new(RefCell::new(TreeNode::new(v)));
                current.borrow_mut().left = Some(left.clone());
                queue.push_back(left);
            }
            i += 1;
        }

        // right child
        if i < data.len() {
            if let Some(v) = data[i] {
                let right = Rc::new(RefCell::new(TreeNode::new(v)));
                current.borrow_mut().right = Some(right.clone());
                queue.push_back(right);
            }
            i += 1;
        }
    }

    Some(root)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        // root = [3,9,20,null,null,15,7]
        let root = build_tree(vec![
            Some(3),
            Some(9),
            Some(20),
            None,
            None,
            Some(15),
            Some(7),
        ]);

        let expected = vec![vec![3], vec![9, 20], vec![15, 7]];
        assert_eq!(Solution::level_order(root), expected);
    }

    #[test]
    fn example_2() {
        // root = [1]
        let root = build_tree(vec![Some(1)]);
        let expected = vec![vec![1]];
        assert_eq!(Solution::level_order(root), expected);
    }

    #[test]
    fn example_3() {
        // root = []
        let root = build_tree(vec![]);
        let expected: Vec<Vec<i32>> = vec![];
        assert_eq!(Solution::level_order(root), expected);
    }
}