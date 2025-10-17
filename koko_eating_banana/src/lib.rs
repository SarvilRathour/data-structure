struct Solution;
impl Solution {
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
            let mut left=1;
            let max=*piles.iter().max().unwrap();
            let mut right=max;
            let mut result=max;
            while left<=right{
              let middle=left+(right-left)/2;
              let hours:i64=piles.iter().map(|&pile| ((pile as i64)+(middle as i64)-1)/(middle as i64)).sum();
              if hours<=h as i64{
              result=middle;
              right=middle-1;
              }else{
              left=middle+1;
              }
            }
            result as i32
    }
}
#[cfg(test)]
mod tests{
  use super::*;
  #[test]
  fn test1(){
    let piles=vec![3,6,7,11];
    let h=8;
    let result=Solution::min_eating_speed(piles, h);
    assert_eq!(result,4);
  }
}


