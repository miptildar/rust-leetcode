use std::collections::HashMap;

#[test]
fn runner() {
    println!("{:?}", Solution::eval_rpn(vec!["4","13","5","/","+"].into_iter().map(String::from).collect()));
}

struct Solution;

impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack: Vec<i32> = Vec::new();
        for token in &tokens {
            match token.as_str() {
                op @ ("+" | "-" | "*" | "/") => {
                    let right = stack.pop().unwrap();
                    let left = stack.pop().unwrap();
                    stack.push(match op {
                        "+" => left + right,
                        "-" => left - right,
                        "*" => left * right,
                        _ => left / right,
                    });
                }
                num => stack.push(num.parse().unwrap()),
            }
        }

        stack.pop().unwrap()
    }
}
