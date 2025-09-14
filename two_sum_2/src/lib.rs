pub struct Solution;
impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut left=0;
        let mut right=numbers.len()-1;
        while right>left{
            let addition=numbers[left]+numbers[right];
            if addition>target{
                right-=1;
            }else if addition<target {
                left+=1;
            }else{
                return vec![(left + 1) as i32, (right + 1) as i32];
            }
        }
        vec![]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        let number = vec![2, 7, 11, 15];
        let target = 9;
        let result = Solution::two_sum(number, target);
        assert_eq!(result, [1, 2]);
    }
        #[test]
    fn test2() {
        let number = vec![-1,0];
        let target =  -1;
        let result = Solution::two_sum(number, target);
        assert_eq!(result,[1,2]);
    }
}
