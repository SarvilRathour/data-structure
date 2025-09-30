pub struct Solution;
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut low = 0;
        let mut high = nums.len() as i32 - 1;
        while high>=low{
            let mut middle=(low+high)/2;
            let mut middle_val=nums[middle as usize];
            if middle_val<target{
                low=middle+1;
            }else if middle_val>target{
                high=middle-1;
            }else{
                return middle;
            }
        }
       -1
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        let value = Solution::search(vec![-1, 0, 3, 5, 9, 12], 9);
        assert_eq!(value, 4);
    }
}
