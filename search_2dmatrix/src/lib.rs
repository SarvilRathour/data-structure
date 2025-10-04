struct Solution;
impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
            let row=matrix.len();
            let column=matrix[0].len();
            let mut left=0;
            let mut right=(row*column)-1;
            while left<=right{
                let mid_val=left+(right-left)/2;
                let mid_val_row=mid_val/column;
                let mid_val_col=mid_val%column;
                let value=matrix[mid_val_row][mid_val_col];
                if value==target{
                    return true;
                }else if value<target{
                    left=mid_val+1;
                }else{
                    if mid_val==0{
                        break;
                    }
                    right=mid_val-1;
                }
            }
            false
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = Solution::search_matrix(vec![vec![1,3,5,7],vec![10,11,16,20],vec![23,30,34,60]], 20);
        assert!(result);
    }
}
