use std::hash::{Hash, Hasher, self};
use std::collections::hash_map::DefaultHasher;


pub fn generate_hash(){
    let mut s = DefaultHasher::new();
    let code = r#"use std::collections::HashMap;\n\nimpl Solution {\n    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {\n        let mut hash: HashMap<i32, usize> = HashMap::new();\n        let mut result: Vec<i32> = Vec::new();\n\n        for i in 0..nums.len() {\n            let ele = nums[i];\n            let required = (target - ele);\n            match hash.get(&required) {\n                Some(j) => {\n                    return vec![i as i32, j.clone() as i32];\n                }\n                None => {\n                    hash.insert(ele, i);\n                }\n            }\n        }\n        result\n    }\n}"#;
    code.hash(&mut s);
    let hashed_value = s.finish();
    println!("Hashed value {}", hashed_value);
}