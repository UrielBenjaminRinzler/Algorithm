use std::collections::{HashSet};
pub struct tests;

impl tests {
    pub fn count_duplicates(text: &str) -> u32 {
        let result = &mut 0u32;
        let temp = text.to_string().clone();
        let v1:&String = &temp.chars().filter(|chr|chr.is_alphanumeric()).map(|chr|chr.to_lowercase().next().unwrap()).collect();
        let v2:String = v1.clone();
        let hs:HashSet<char> = v2.chars().collect();
        for ch in hs {
            let tmp = v1.chars().filter(|chr|*chr == ch).count();
            if tmp > 1 {
                *result += 1;
            }
        }
        *result
    }
}