use std::collections::HashMap;
pub struct Solution;
impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32)->Vec<i32>{
        let mut new:HashMap<i32,usize>=HashMap::new();
        for num in nums{
          *new.entry(num).or_insert(0)+=1;
        }
        let mut sort_vec:Vec<_>=new.iter().collect();
        sort_vec.sort_by(|a,b| b.1.cmp(a.1));
        let topkeys:Vec<i32>=sort_vec.iter().take(k as usize).map(|(k, _)| **k).collect();
        topkeys
    }
}
fn main(){
  let nums = vec![1,1,1,2,2,3];
  let k = 2;
  let ans=Solution::top_k_frequent(nums,k);
  println!("{:?}",ans);
}

