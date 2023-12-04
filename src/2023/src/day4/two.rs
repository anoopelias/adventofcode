use crate::day4::wins_for;
use std::vec;

pub(crate) fn solve(lines: Vec<String>) -> String {
    let mut sum = 0;
    let mut copy_counts = vec![];
    for (i, line) in lines.iter().enumerate() {
        let wins = wins_for(line);

        if copy_counts.get(i) == None {
            copy_counts.push(1);
        }

        let copy_count = copy_counts.get(i).unwrap().clone();
        sum += copy_count;

        for j in 0..(wins as usize) {
            let index = i + j + 1;
            if copy_counts.get(index) == None {
                copy_counts.push(1);
            }
            let copy_count = copy_counts.get(index).unwrap() + copy_count;
            let _ = std::mem::replace(&mut copy_counts[index], copy_count);
        }
    }

    sum.to_string()
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
