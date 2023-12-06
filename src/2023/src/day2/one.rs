pub(crate) fn solve(lines: Vec<String>) -> String {
    let mut sum = 0;

    for line in lines {
        let splits: Vec<&str> = line.split(":").collect();
        let game = splits.get(0).unwrap();
        let gamesplit: Vec<&str> = game.split(" ").collect();
        let num = gamesplit.get(1).unwrap().parse::<i32>().unwrap();

        let cube_sets = splits.get(1).unwrap().split(";");
        let mut possible = true;

        for cube_set in cube_sets {
            let cubes = cube_set.split(", ");

            for cube in cubes {
                let splits: Vec<&str> = cube.trim().split(" ").collect();
                let count = splits.get(0).unwrap().parse::<i32>().unwrap();
                let color = splits.get(1).unwrap();

                if !is_possible(count, color) {
                    possible = false;
                    break;
                }
            }

            if !possible {
                break;
            }
        }

        if possible {
            sum += num;
        }
    }

    sum.to_string()
}

fn is_possible(count: i32, color: &str) -> bool {
    match color {
        "red" if count <= 12 => true,
        "green" if count <= 13 => true,
        "blue" if count <= 14 => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use crate::day2::one::solve;
    use crate::utils::util;

    #[test]
    fn test_sample() {
        let lines = util::lines_in("./src/day2/input");
        assert_eq!("8", solve(lines))
    }

    #[test]
    fn test_input() {
        let lines = util::lines_in("./src/day2/input1");
        assert_eq!("2447", solve(lines))
    }
}
