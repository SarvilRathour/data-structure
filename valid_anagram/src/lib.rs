pub struct Solution;
impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut c1:Vec<char>=s.chars().collect();
        let mut c2:Vec<char>=t.chars().collect();
        c1.sort();
        c2.sort();
        c1==c2
    }
}
#[cfg(test)]
mod test{
    use super::*;
    #[test]
    fn test1(){
        assert!(Solution::is_anagram("anagram".to_string(),"nagaram".to_string()));
    }
}
