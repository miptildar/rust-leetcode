use std::collections::HashMap;

#[test]
fn runner() {
    println!("{:?}", Solution::build_array(vec![1,2], 4));
}

struct Solution;

impl Solution {
    pub fn build_array(target: Vec<i32>, n: i32) -> Vec<String> {
        let push_str: &'static str = "Push";
        let pop_str: &'static str = "Pop";

        let mut result: Vec<String> = Vec::new();

        let mut index: usize = 0;
        let mut head = 1;
        while index < target.len() {
            let to_be_achieved = target[index];

            if (head < to_be_achieved) {
                while head < to_be_achieved {
                    result.push(push_str.to_string());
                    result.push(pop_str.to_string());
                    head += 1;
                }
                continue;
            }


            result.push(push_str.to_string());
            head += 1;
            index += 1;
        }


        result
    }
}
