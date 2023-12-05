use std::cmp;

use crate::util::string_to_nums;

struct Mapper {
    num_maps: Vec<(i64, i64, i64)>,
}

impl Mapper {
    fn new() -> Mapper {
        Mapper { num_maps: vec![] }
    }

    fn insert(&mut self, line: &str) {
        let num_maps = string_to_nums(&line);
        let num_map = (
            num_maps.get(0).unwrap().clone(),
            num_maps.get(1).unwrap().clone(),
            num_maps.get(2).unwrap().clone(),
        );

        match self.num_maps.binary_search_by(|nm| nm.1.cmp(&num_map.1)) {
            Ok(_) => println!("These should not overlap"),
            Err(pos) => self.num_maps.insert(pos, num_map),
        }
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
    let seeds_numbers = string_to_nums(splits.get(1).unwrap());
    let mut seed_pairs = vec![];

    let mut prev = None;
    for seed_number in seeds_numbers {
        if let Some(start) = prev {
            seed_pairs.push((start, seed_number));
            prev = None;
        } else {
            prev = Some(seed_number);
        }
    }

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
                mappers[mindex - 1].insert(&line);
            }
        } else {
            mindex += 1;
        }
    }

    let mut min = i64::MAX;

    for (start, range) in seed_pairs {
        for offset in 0..range {
            let mut from = start + offset;
            for mapper in mappers.iter() {
                from = mapper.map(from);
            }
            min = cmp::min(min, from);
        }
    }

    min.to_string()
}

#[cfg(test)]
mod tests {
    use crate::day5::two::solve;
    use crate::util;

    #[test]
    fn test_sample() {
        let lines = util::lines_in("./src/day5/input");
        assert_eq!("46", solve(lines))
    }

    #[test]
    fn test_input() {
        // let lines = util::lines_in("./src/day5/input1");
        // assert_eq!("51752125", solve(lines))
    }
}
