use std::collections::HashSet;

#[test]
fn runner() {
    let arr = vec![4,3,2,7,8,2,3,1];

    println!("{:?}", Solution::find_disappeared_numbers(arr));
}

struct Solution;

impl Solution {
    pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
        let mut set:HashSet<i32> = HashSet::new();
        for value in &nums {
            set.insert(*value);
        }

        let mut result: Vec<i32> = Vec::new();
        for number in 1..=nums.len() {
            if !set.contains(&(number as i32)) {
                result.push(number as i32);
            }
        }

        result
    }
}
