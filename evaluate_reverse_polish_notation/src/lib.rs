struct Solution;
impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
            let mut stack=Vec::new();
            for token in tokens{
                match token.as_str(){
                    "+"=>{
                        let a=stack.pop().unwrap();
                        let b=stack.pop().unwrap();
                        stack.push(b+a);
                    }
                    "-"=>{
                        let a=stack.pop().unwrap();
                        let b=stack.pop().unwrap();
                        stack.push(b-a);
                    }
                    "*"=>{
                         let a=stack.pop().unwrap();
                        let b=stack.pop().unwrap();
                        stack.push(b*a);
                    }
                    "/"=>{
                       let a=stack.pop().unwrap();
                        let b=stack.pop().unwrap();
                        stack.push(b/a);
                    }
                    _=>{
                        let val=token.parse::<i32>().unwrap();
                        stack.push(val);
                    }
                }
            }
            stack[0]
        }
}
#[cfg(test)]
mod test{
    use super::*;
    #[test]
    fn test1(){
        let pass=vec!["2".to_string(),"1".to_string(),"+".to_string(),"3".to_string(),"*".to_string()];
        let result=Solution::eval_rpn(pass);
        assert_eq!(result,9);
    }
}
