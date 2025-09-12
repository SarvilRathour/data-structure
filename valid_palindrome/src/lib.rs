struct Solution;
impl Solution {
    pub fn is_palindrome(s: String) -> bool {
            let filtered_string:Vec<char>=s.to_lowercase().chars().filter(|c| c.is_alphanumeric()).collect();
            if filtered_string.is_empty(){
                return true;
            }
            let mut left=0;
            let mut right=filtered_string.len()-1;
            while right>left{
                if filtered_string[right]!=filtered_string[left]{
                    return false;
                }
                left+=1;
                right-=1;
            }
            true
      }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = Solution::is_palindrome("OP".to_string());
        assert_eq!(result, false);
    }
    #[test]
    fn for_null(){
        let result1=Solution::is_palindrome("".to_string());
        assert_eq!(result1,true);
    }
}
