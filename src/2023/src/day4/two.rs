use std::{collections::HashSet, vec};

pub(crate) fn solve(lines: Vec<String>) -> String {
    let mut sum = 0;
    let mut no_of_cards = vec![];
    for (i, line) in lines.iter().enumerate() {
        let line: Vec<&str> = line.split(":").collect();
        let line = line.get(1).unwrap().trim();

        let line: Vec<&str> = line.split("|").collect();

        let winner = string_to_nums(line.get(0).unwrap().trim());
        let card = string_to_nums(line.get(1).unwrap().trim());

        if no_of_cards.get(i) == None {
            no_of_cards.push(0);
        }

        let this_wins = no_of_cards.get(i).unwrap().clone() + 1;
        sum += this_wins;

        let mut wins: i32 = 0;
        for my_num in card {
            if winner.contains(&my_num) {
                wins += 1;
            }
        }

        for j in 0..(wins as usize) {
            let num = if let Some(val) = no_of_cards.get(i + j + 1) {
                Some(val + this_wins)
            } else {
                None
            };

            match num {
                Some(val) => {
                    let _ = std::mem::replace(&mut no_of_cards[i + j + 1], val);
                }
                None => no_of_cards.push(this_wins),
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
