struct Solution;
impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
       use std::collections::HashMap;
       use std::cmp::max;
       let k_usize=k as usize;
       let mut freq:HashMap<char,usize>=HashMap::new();
       let (mut left,mut max_freq,mut res)=(0,0,0);
       let s_chars:Vec<char>=s.chars().collect();
       for right in 0..s_chars.len(){
        let count=freq.entry(s_chars[right]).and_modify(|c| *c+=1).or_insert(1);
        max_freq = max(max_freq, *count);
            while (right-left +1)-max_freq>k_usize{
                    if let Some(count)=freq.get_mut(&s_chars[left]){
                        *count-=1;
                    }
                    left+=1;
            }
        
        res=max(res,right-left+1);
       }
       
       res as i32   
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = Solution::character_replacement("ABAB".to_string(), 2);
        assert_eq!(result, 4);
    }
}
