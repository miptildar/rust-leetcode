use std::collections::HashMap;

#[test]
fn runner() {
    let arr = vec![5,0,10,0,10,6];

    println!("{:?}", Solution::smaller_numbers_than_current(arr));
}

struct Solution;

impl Solution {
    pub fn smaller_numbers_than_current(nums: Vec<i32>) -> Vec<i32> {
        let mut popularity = [0; 101];

        for value in &nums {
            popularity[*value as usize] += 1;
        }

        let mut sums = [0;101];
        sums[0] = popularity[0];
        for i in 1..popularity.len() {
            sums[i] = sums[i - 1] + popularity[i];
        }

        let mut result: Vec<i32> = Vec::with_capacity(nums.len());
        for value in &nums {
            result.push(sums[*value as usize] - popularity[*value as usize]);
        }

        result
    }
}
