pub struct Solution;
impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut result:Vec<String>=Vec::new();
        let mut current_combination:String=String::new();
        backtrack(&mut result,current_combination,0,0,n);
        fn backtrack(r:&mut Vec<String>,current:String,cl:i32,co:i32,n:i32){
          if current.len()==(n*2) as usize{
            r.push(current.to_string());
            return;
          }
          if co<n{
           backtrack(r, format!("{}(",current), cl, co+1, n);
          }
          if cl<co{
            backtrack(r, format!("{})",current), cl+1, co, n)
          }
        }
        result
    }
}
fn main(){
  let result=Solution::generate_parenthesis(3);
  assert_eq!(result,vec!["((()))","(()())","(())()","()(())","()()()"]);
}
