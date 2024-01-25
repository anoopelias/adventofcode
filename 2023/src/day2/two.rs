use std::cmp;

pub(crate) fn solve(lines: Vec<String>) -> String {
    let mut sum = 0;

    for line in lines {
        let splits: Vec<&str> = line.split(":").collect();
        let cube_sets = splits.get(1).unwrap().split(";");

        let mut min_red = 0;
        let mut min_green = 0;
        let mut min_blue = 0;

        for cube_set in cube_sets {
            let cubes = cube_set.split(", ");

            for cube in cubes {
                let splits: Vec<&str> = cube.trim().split(" ").collect();
                let count = splits.get(0).unwrap().parse::<i32>().unwrap();
                let color = splits.get(1).unwrap();
                match *color {
                    "red" => min_red = cmp::max(min_red, count),
                    "green" => min_green = cmp::max(min_green, count),
                    "blue" => min_blue = cmp::max(min_blue, count),
                    _ => {}
                }
            }
        }

        let power = min_red * min_green * min_blue;
        sum += power;
    }

    sum.to_string()
}

#[cfg(test)]
mod tests {
    use crate::day2::two::solve;
    use crate::utils::util;

    #[test]
    fn test_sample() {
        let lines = util::lines_in("./src/day2/input");
        assert_eq!("2286", solve(lines))
    }

    #[test]
    fn test_input() {
        let lines = util::lines_in("./src/day2/input1");
        assert_eq!("56322", solve(lines))
    }
}
