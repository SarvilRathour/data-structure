
struct Solution;
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack=Vec::new();
        for ch in s.chars(){
            if ch=='{' || ch=='[' || ch=='(' {
                stack.push(ch);
            }else{
              if let Some(pop)=stack.pop(){
                if pop=='(' && ch!=')'{return false};
                if pop=='{' && ch!='}'{return false};
                if pop=='[' && ch!=']'{return false};
              }else{
                return false;
              }
            }
        }
        stack.is_empty()
    }
}
#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test1(){
        let string="()[]{}".to_string();
        assert!(Solution::is_valid(string));
    }
        #[test]
    fn test2(){
        let string="()[]{}[[".to_string();
        assert!(!Solution::is_valid(string));
    }
    
}
