use std::usize;

struct Number {
    p: usize,
    q: usize,
    done: bool,
    value: u32,
}

impl Number {
    fn new(p: usize, q: usize, value: u32) -> Number {
        return Number {
            p,
            q,
            done: false,
            value,
        };
    }
}

pub(crate) fn solve(lines: Vec<String>) -> String {
    let (m, n) = (lines.len() as i32, lines.get(0).unwrap().len() as i32);

    let mut sum = 0;

    for (p, line) in lines.iter().enumerate() {
        let chars = line.chars();
        let mut num_part = 0;
        let mut should_add = false;
        for (q, ch) in chars.enumerate() {
            if ch.is_numeric() {
                num_part = (num_part * 10) + ch.to_digit(10).unwrap();
                if has_symbol(&lines, p as i32, q as i32, m, n) {
                    should_add = true;
                }
            } else {
                if should_add {
                    println!("Adding {}", num_part);
                    sum += num_part;
                }
                num_part = 0;
                should_add = false;
            }
        }
        if should_add {
            sum += num_part;
        }
    }

    sum.to_string()
}

fn char_to_num(st: &str) -> u32 {
    st.chars().nth(0).unwrap().to_digit(10).unwrap()
}

fn has_symbol(lines: &Vec<String>, p: i32, q: i32, m: i32, n: i32) -> bool {
    let neighbors = vec![
        (p - 1, q - 1),
        (p - 1, q),
        (p - 1, q + 1),
        (p, q - 1),
        (p, q + 1),
        (p + 1, q - 1),
        (p + 1, q),
        (p + 1, q + 1),
    ];
    let neighbors: Vec<&(i32, i32)> = neighbors
        .iter()
        .filter(|(p, q)| p >= &0 && q >= &0 && p < &m && q < &n)
        .collect();

    for (p, q) in neighbors {
        let ch = lines
            .get(*p as usize)
            .unwrap()
            .chars()
            .nth(*q as usize)
            .unwrap();
        if ch != '.' && !ch.is_numeric() {
            return true;
        }
    }

    return false;
}

#[cfg(test)]
mod tests {
    use crate::day3::one::solve;
    use crate::util;

    #[test]
    fn test_sample() {
        let lines = util::lines_in("./src/day3/input");
        assert_eq!("4361", solve(lines))
    }

    #[test]
    fn test_input() {
        let lines = util::lines_in("./src/day3/input1");
        assert_eq!("538046", solve(lines))
    }
}
