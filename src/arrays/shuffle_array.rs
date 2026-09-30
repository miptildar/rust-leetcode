use std::collections::HashMap;

#[test]
fn runner() {
    let arr = vec![
        2,5,1,3,4,7
    ];

    println!("{:?}", Solution::shuffle(arr, 3));
}

struct Solution;

impl Solution {
    pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
        let mut ans = Vec::with_capacity((n * 2) as usize);

        let half = n as usize;
        for i in 0..(n as usize) {
            ans.push(nums[i]);
            ans.push(nums[i + half]);
        }

        ans
    }
}
