use std::collections::HashMap;
pub struct Solution;

impl Solution{
     pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
       let mut map:HashMap<[u8;26],Vec<String>>=HashMap::new();
       for values in strs{
       let mut chars_key=[0;26];
            for value in values.chars(){
                chars_key[value as usize - 'a' as usize]+=1;
            }
            map.entry(chars_key).or_insert(vec![]).push(values);
       }
     map.into_values().collect()
   }
}
#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test(){
    let strs = vec![
            "eat".to_string(),
            "tea".to_string(),
            "tan".to_string(),
            "ate".to_string(),
            "nat".to_string(),
            "bat".to_string(),
        ];
           let mut expected = vec![
            vec!["bat".to_string()],
            vec!["nat".to_string(), "tan".to_string()],
            vec!["ate".to_string(), "eat".to_string(), "tea".to_string()],
        ];
    let mut output = Solution::group_anagrams(strs);
    assert_eq!(output,expected);
    
    }
}
