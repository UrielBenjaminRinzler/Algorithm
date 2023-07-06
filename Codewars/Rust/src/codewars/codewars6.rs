pub struct tests;

impl tests {
    pub fn get_count(string: &str) -> usize {
        let sample = vec!['a', 'e', 'i', 'o', 'u'];
        string.clone().chars().filter(|chr|sample.contains(chr)).count()
    }
}