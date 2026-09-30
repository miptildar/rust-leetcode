use std::collections::HashMap;

#[test]
fn runner() {
    let arr = vec![
        1,2,1
    ];

    println!("{:?}", Solution::get_concatenation(arr));
}

struct Solution;

impl Solution {
    pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut ans = vec![0i32; 2*n];
        for i in 0..nums.len() {
            ans[i] = nums[i];
            ans[i + n] = nums[i];
        }

        ans
    }
}
