pub trait I32Parser {
    fn parse_i32(self) -> Vec<i32>;
}

impl I32Parser for &str {
    fn parse_i32(self) -> Vec<i32> {
        self.trim()
            .split(" ")
            .collect::<Vec<_>>()
            .iter()
            .map(|s| s.trim())
            .filter(|s| s.len() > 0)
            .map(|s| s.parse::<i32>().unwrap())
            .collect()
    }
}
