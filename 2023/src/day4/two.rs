use crate::day4::wins_for;
use std::vec;

pub(crate) fn solve(lines: Vec<String>) -> String {
    let mut sum = 0;
    let mut copy_counts = vec![];
    for (i, line) in lines.iter().enumerate() {
        let wins = wins_for(line);

        fill(&mut copy_counts, i);

        let copy_count = copy_counts.get(i).unwrap().clone();
        sum += copy_count;

        for j in 0..(wins as usize) {
            let k = i + j + 1;
            fill(&mut copy_counts, k);
            let copy_count = copy_counts.get(k).unwrap() + copy_count;
            let _ = std::mem::replace(&mut copy_counts[k], copy_count);
        }
    }

    sum.to_string()
}

fn fill(v: &mut Vec<i32>, index: usize) {
    if v.get(index) == None {
        v.push(1);
    }
}

#[cfg(test)]
mod tests {
    use crate::day4::two::solve;
    use crate::utils::util;

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
