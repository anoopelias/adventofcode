pub trait I32Parser {
    fn parse_i32(self) -> Vec<i32>;
}

pub trait I64Parser {
    fn parse_i64(self) -> Vec<i64>;
}

pub trait UsizeParser {
    fn parse_usize(self, separator: &str) -> Vec<usize>;
}

pub trait SeparatorParser {
    fn parse_separator(self, sep: Self) -> Vec<Self>
    where
        Self: Sized;
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

impl UsizeParser for &str {
    fn parse_usize(self, separator: &str) -> Vec<usize> {
        self.parse_separator(separator)
            .iter()
            .map(|s| s.parse::<usize>().unwrap())
            .collect()
    }
}

impl SeparatorParser for &str {
    fn parse_separator(self, sep: &str) -> Vec<Self>
    where
        Self: Sized,
    {
        self.trim()
            .split(sep)
            .collect::<Vec<_>>()
            .iter()
            .map(|s| s.trim())
            .filter(|s| s.len() > 0)
            .collect()
    }
}

pub trait TwoSplitter {
    fn split_in_two(self, sep: &str) -> (Self, Self)
    where
        Self: Sized;
}

impl TwoSplitter for &str {
    fn split_in_two(self, sep: &str) -> (Self, Self)
    where
        Self: Sized,
    {
        let mut splits = self.split(sep);
        (splits.next().unwrap().trim(), splits.next().unwrap().trim())
    }
}
