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
use std::rc::Rc;
use std::cell::RefCell;
pub struct Solution;
impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node_rc)=&root{  //borrowing the option just for duration of the block,let safely access the innervalue of root
          let mut node=node_rc.borrow_mut();
              let left=node.left.take();
              let right=node.right.take();
              node.left=Solution::invert_tree(right);
              node.right=Solution::invert_tree(left);
        }
        root
    }
}
fn main(){
  
}
