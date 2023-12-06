pub trait I32Parser {
    fn parse_i32(self) -> Vec<i32>;
}

pub trait I64Parser {
    fn parse_i64(self) -> Vec<i64>;
}

pub trait SpaceParser {
    fn parse_space(self) -> Vec<String>;
}

impl I32Parser for &str {
    fn parse_i32(self) -> Vec<i32> {
        self.parse_space()
            .iter()
            .map(|s| s.parse::<i32>().unwrap())
            .collect()
    }
}

impl I64Parser for &str {
    fn parse_i64(self) -> Vec<i64> {
        self.parse_space()
            .iter()
            .map(|s| s.parse::<i64>().unwrap())
            .collect()
    }
}

impl SpaceParser for &str {
    fn parse_space(self) -> Vec<String> {
        self.trim()
            .split(" ")
            .collect::<Vec<_>>()
            .iter()
            .map(|s| s.trim())
            .filter(|s| s.len() > 0)
            .map(|s| s.to_string())
            .collect()
    }
}
