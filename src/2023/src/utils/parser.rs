pub trait I32Parser {
    fn parse_i32(self) -> Vec<i32>;
}

pub trait I64Parser {
    fn parse_i64(self) -> Vec<i64>;
}

pub trait SeparatorParser {
    fn parse_separator(self, sep: &str) -> Vec<String>;
}

impl I32Parser for &str {
    fn parse_i32(self) -> Vec<i32> {
        self.parse_separator(" ")
            .iter()
            .map(|s| s.parse::<i32>().unwrap())
            .collect()
    }
}

impl I64Parser for &str {
    fn parse_i64(self) -> Vec<i64> {
        self.parse_separator(" ")
            .iter()
            .map(|s| s.parse::<i64>().unwrap())
            .collect()
    }
}

impl SeparatorParser for &str {
    fn parse_separator(self, sep: &str) -> Vec<String> {
        self.trim()
            .split(sep)
            .collect::<Vec<_>>()
            .iter()
            .map(|s| s.trim())
            .filter(|s| s.len() > 0)
            .map(|s| s.to_string())
            .collect()
    }
}
