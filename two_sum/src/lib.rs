use std::collections::HashMap;
struct Solution;
impl Solution{
    fn two_sum(nums:Vec<i32>,target:i32)->Vec<i32>{
        let mut indexing=HashMap::new();
        for(i,num) in nums.iter().enumerate(){
            let mut res=target-num;
                if let Some(check)=indexing.get(&res){
                    return vec![*check as i32, i as i32];
                }else{
                    indexing.insert(num, i);
                }
        }
        vec![]
    }
}
#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test(){
        let nums=vec![2,7,11,15];
        let target=9;
        let result=Solution::two_sum(nums,target);
        assert_eq!(result,vec![0,1]);
    }
}
