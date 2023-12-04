use std::collections::HashSet;

pub(crate) fn solve(lines: Vec<String>) -> String {
    let mut sum = 0;
    for line in lines.iter() {
        let line: Vec<&str> = line.split(":").collect();
        let line = line.get(1).unwrap().trim();

        let line: Vec<&str> = line.split("|").collect();

        let winners = string_to_nums(line.get(0).unwrap().trim());
        let card = string_to_nums(line.get(1).unwrap().trim());

        let wins = card
            .iter()
            .filter(|num| winners.contains(num))
            .collect::<Vec<_>>()
            .len();

        let num: i32 = 2;
        if wins > 0 {
            sum += num.pow((wins - 1).try_into().unwrap());
        }
    }
    sum.to_string()
}

fn string_to_nums(str: &str) -> HashSet<i32> {
    let nums: Vec<&str> = str.split(" ").collect();
    nums.iter()
        .map(|s| s.trim())
        .filter(|s| s.len() > 0)
        .map(|s| s.parse::<i32>().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::day4::one::solve;
    use crate::util;

    #[test]
    fn test_sample() {
        let lines = util::lines_in("./src/day4/input");
        assert_eq!("13", solve(lines))
    }

    #[test]
    fn test_input() {
        let lines = util::lines_in("./src/day4/input1");
        assert_eq!("26426", solve(lines))
    }
}
