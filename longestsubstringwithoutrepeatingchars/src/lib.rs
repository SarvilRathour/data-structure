struct Solution;
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut left=0;
        let mut right=0;
        let mut max_window=0;
        let chars:Vec<char>=s.chars().collect();
        let mut window_size:Vec<char>=Vec::new();
        while right<chars.len(){
            let c=chars[right];
            if window_size.contains(&c){
                    while window_size.contains(&c){
                        window_size.remove(0);
                        left+=1;
                }
            }
            window_size.push(c);
            right+=1;
            if max_window<window_size.len(){
                max_window=window_size.len();
            }
        }
        max_window as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = Solution::length_of_longest_substring(String::from("abcabcbb"));
        assert_eq!(result, 3);
    }
}
