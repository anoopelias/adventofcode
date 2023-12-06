use super::util::string_to_nums;

pub trait I32Parser {
    fn parse_i32(&self) -> Vec<i32>;
}

impl I32Parser for &str {
    fn parse_i32(&self) -> Vec<i32> {
        string_to_nums(self)
    }
}
