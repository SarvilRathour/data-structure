struct Solution;
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = 0;
        let mut max_profit = 0;
        while right < prices.len() {
            if prices[left]>prices[right]{
                left=right;
            }else{
                if max_profit<(prices[right]-prices[left]){
                    max_profit=prices[right]-prices[left];
                }
            }
            right+=1;
        }
        max_profit
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = Solution::max_profit(vec![7, 1, 5, 3, 6, 4]);
        assert_eq!(result, 5);
    }
}
