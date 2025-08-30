use std::collections::HashSet;
pub struct Solution;
impl Solution{
   pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut contain=HashSet::new();
    for n in nums{
        if !contain.insert(n){
            return true;
        }
    }
    return false;
  }
}
#[cfg(test)]
mod test{
  use super::*;
  #[test]
  fn check(){
    assert!(Solution::contains_duplicate(vec![7,7,3]));
  }
}
