use std::collections::HashMap;

#[test]
fn runner() {
    let arr = vec![
        "eat".to_string(),
        "tea".to_string(),
        "tan".to_string(),
        "ate".to_string(),
        "nat".to_string(),
        "bat".to_string(),
    ];
    Solution::group_anagrams(arr);
    println!("{:?}", 1);
}

struct Solution;

impl Solution {
    const A_CODE: u8 = 'a' as u8;

    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        if strs.len() == 0 {
            return vec![vec![]];
        }

        let mut map: HashMap<String, Vec<String>> = HashMap::new();
        for s in strs.iter() {
            let key = Self::calculate(&s);
            map.entry(key)
                .and_modify(|v| v.push(s.clone()))
                .or_insert(vec![s.clone()]);
        }

        map.values().cloned().collect()
    }

    pub fn calculate(s: &str) -> String {
        let mut arr: Vec<u8> = vec![0; 26];
        for c in s.chars() {
            let code = c as u8;
            arr[code as usize - Self::A_CODE as usize] += 1;
        }

        let mut result = String::new();
        for (i, v) in arr.iter().enumerate() {
            if arr[i] == 0 {
                continue;
            }

            let code: char = (i as u8 + Self::A_CODE) as char;
            result.push(code);
            result.push_str(&v.to_string());
        }

        result
    }
}
