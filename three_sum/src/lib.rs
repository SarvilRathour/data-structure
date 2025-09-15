struct Solution;
impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans: Vec<Vec<i32>> = Vec::new();
        let mut num = nums;
        num.sort_by(|a, b| a.cmp(b));
            for i in 0..num.len(){
            if i > 0 && num[i] == num[i - 1] { continue; }
                let mut left=i+1;
                let mut right=num.len()-1;
                while left<right{
                    let sum=num[i]+num[left]+num[right];
                    if sum==0{
                        ans.push(vec![num[i],num[left],num[right]]);
                        right-=1;
                        left+=1;
                        while left<right && num[left]==num[left-1]{
                            left+=1;
                        }
                        while left<right && num[right]==num[right+1]{
                            right-=1;
                        }
                    }else if sum>0{
                        right-=1;
                    }else{
                        left+=1;
                    }
                }
            }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        let nums = vec![-1, 0, 1, 2, -1, -4];
        let result1 = Solution::three_sum(nums);
        assert_eq!(result1, vec![[-1, -1, 2], [-1, 0, 1]]);
    }
}
