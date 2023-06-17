struct Solution;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let merged:&mut Vec<i32> = &mut [nums1, nums2].concat();
        merged.sort();
        if merged.len() % 2 == 0 {
            let midFr = (merged.len() / 2 - 1) as usize;
            (merged[midFr] + merged[midFr + 1]) as f64 / 2 as f64
        } else {
            merged[merged.len() / 2  as usize] as f64
        }
    }
}

fn main() {
    let result:f64 = Solution::find_median_sorted_arrays(vec![1, 3], vec![5]);
    println!("answer is : {}", result);
}
