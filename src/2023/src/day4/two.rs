use std::{collections::HashSet, vec};

pub(crate) fn solve(lines: Vec<String>) -> String {
    let mut sum = 0;
    let mut copy_counts = vec![];
    for (i, line) in lines.iter().enumerate() {
        let line: Vec<&str> = line.split(":").collect();
        let line = line.get(1).unwrap().trim();

        let line: Vec<&str> = line.split("|").collect();

        let winner = string_to_nums(line.get(0).unwrap().trim());
        let card = string_to_nums(line.get(1).unwrap().trim());

        if copy_counts.get(i) == None {
            copy_counts.push(0);
        }

        let this_wins = copy_counts.get(i).unwrap().clone() + 1;
        sum += this_wins;

        let wins = card
            .iter()
            .filter(|num| winner.contains(num))
            .collect::<Vec<_>>()
            .len();

        for j in 0..(wins as usize) {
            let copy_count = if let Some(val) = copy_counts.get(i + j + 1) {
                Some(val + this_wins)
            } else {
                None
            };

            match copy_count {
                Some(val) => {
                    let _ = std::mem::replace(&mut copy_counts[i + j + 1], val);
                }
                None => copy_counts.push(this_wins),
            }
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
    use crate::day4::two::solve;
    use crate::util;

    #[test]
    fn test_sample() {
        let lines = util::lines_in("./src/day4/input");
        assert_eq!("30", solve(lines))
    }

    #[test]
    fn test_input() {
        let lines = util::lines_in("./src/day4/input1");
        assert_eq!("6227972", solve(lines))
    }
}
