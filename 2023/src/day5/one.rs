use std::cmp;

use crate::utils::parser::I64Parser;

struct Mapper {
    num_maps: Vec<(i64, i64, i64)>,
}

impl Mapper {
    fn new() -> Mapper {
        Mapper { num_maps: vec![] }
    }

    fn map(&self, from: i64) -> i64 {
        for (dst, src, range) in self.num_maps.iter() {
            if from >= *src && from < (src + range) {
                return dst + from - src;
            }
        }
        from
    }
}

pub(crate) fn solve(lines: Vec<String>) -> String {
    let splits = lines.get(0).unwrap().split(":").collect::<Vec<_>>();
    let seeds = splits.get(1).unwrap().parse_i64();

    let mut mappers = vec![
        Mapper::new(),
        Mapper::new(),
        Mapper::new(),
        Mapper::new(),
        Mapper::new(),
        Mapper::new(),
        Mapper::new(),
    ];

    let mut mindex = 0;
    for line in lines {
        if let Some(ch) = line.chars().nth(0) {
            if ch.is_numeric() {
                let num_maps = &line.parse_i64();
                let num_map = (
                    num_maps.get(0).unwrap().clone(),
                    num_maps.get(1).unwrap().clone(),
                    num_maps.get(2).unwrap().clone(),
                );
                mappers[mindex - 1].num_maps.push(num_map);
            }
        } else {
            mindex += 1;
        }
    }

    let mut min = i64::MAX;
    for seed in seeds {
        let mut from = seed;
        for mapper in mappers.iter() {
            from = mapper.map(from);
        }
        min = cmp::min(min, from);
    }

    min.to_string()
}

#[cfg(test)]
mod tests {
    use crate::day5::one::solve;
    use crate::utils::util;

    #[test]
    fn test_sample() {
        let lines = util::lines_in("../../aoc-files/2023/day5/input");
        assert_eq!("35", solve(lines))
    }

    #[test]
    fn test_input() {
        let lines = util::lines_in("../../aoc-files/2023/day5/input1");
        assert_eq!("51752125", solve(lines))
    }
}
