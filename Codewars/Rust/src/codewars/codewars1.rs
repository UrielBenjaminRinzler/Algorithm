use std::collections::{HashSet};
pub struct tests;

impl tests {
    pub fn solution(num:i32) -> i32 {
        let cp = &num.clone();
        let tgt = &vec![3, 5, 15];
        let temp = &mut HashSet::<i32>::new();
        let i = &mut 1;
        while ((tgt[0] * *i) < *cp) || (tgt[1] * *i < *cp) || (tgt[2] * *i < *cp) {
            if (tgt[0] * *i) < *cp {
                temp.insert(tgt[0] * *i);
            }
            if (tgt[1] * *i) < *cp {
                temp.insert(tgt[1] * *i);
            }
            if (tgt[2] * *i) < *cp {
                temp.insert(tgt[2] * *i);
            }
            *i += 1;
        }
        temp.iter().sum::<i32>()
    }
}