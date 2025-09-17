struct Solution;
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut res = 0;
        let mut right = height.len() - 1;
        let mut left = 0;
        while right > left {
            let mut realh = if height[left] > height[right] {
                height[right]
            } else {
                height[left]
            };
            let mut internal = realh * (right - left) as i32;
            if internal > res {
                res = internal;
            }
            if height[left] < height[right] {
                left += 1;
            } else {
                right -= 1
            }
        }

        res
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test1() {
        let height = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
        let result = Solution::max_area(height);
        assert_eq!(result, 49);
    }
}
