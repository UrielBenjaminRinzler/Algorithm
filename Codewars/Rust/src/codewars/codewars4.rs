pub struct tests;

impl tests {
    pub fn find_average(slice : &[f64]) -> f64 {
        if slice.len() == 0 {
            0 as f64
        } else {
            slice.iter().sum::<f64>() / slice.len() as f64
        }
    }
}