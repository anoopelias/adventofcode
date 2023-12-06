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

    fn map_range(&self, from: i64, len: i64) -> Vec<(i64, i64)> {
        let mut ranges = vec![];

        let mut start = from;
        let mut remain = len;
        for (dest, src, range) in self.num_maps.iter() {
            if start + remain < *src {
                break;
            } else if start < *src && (start + remain) > *src {
                let offset = src - start;
                ranges.push((start, offset));
                if (start + remain) < (src + range) {
                    ranges.push((*dest, range - offset));
                    start = start + remain;
                    remain = 0;
                } else {
                    ranges.push((*dest, *range));
                    start = start + offset + range;
                    remain = remain - offset - range;
                }
            } else if start >= *src && (start + remain) < (src + range) {
                let offset = start - src;
                ranges.push((dest + offset, remain));
                start = start + remain;
                remain = 0;
            } else if start >= *src && start < (src + range) && (start + remain) >= (src + range) {
                let offset = start - src;
                ranges.push((dest + offset, (src + range) - start));
                start = start + (range - offset);
                remain = remain - range + offset;
            }
        }

        if remain != 0 {
            ranges.push((start, remain));
        }

        ranges
    }
}

pub(crate) fn solve(lines: Vec<String>) -> String {
    let splits = lines.get(0).unwrap().split(":").collect::<Vec<_>>();
    let seeds_numbers = string_to_nums(splits.get(1).unwrap());
    let mut inputs = vec![];

    let mut prev = None;
    for seed_number in seeds_numbers {
        if let Some(start) = prev {
            inputs.push((start, seed_number));
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

    for mapper in mappers {
        let mut outputs = vec![];
        for (from, len) in inputs {
            outputs.extend(mapper.map_range(from, len));
        }
        inputs = outputs;
    }

    inputs
        .iter()
        .min_by(|x, y| x.0.cmp(&y.0))
        .unwrap()
        .0
        .to_string()
}

#[cfg(test)]
mod tests {
    use crate::day5::two::solve;
    use crate::util;

    use super::Mapper;

    #[test]
    fn test_sample() {
        let lines = util::lines_in("./src/day5/input");
        assert_eq!("46", solve(lines))
    }

    #[test]
    fn test_input() {
        let lines = util::lines_in("./src/day5/input1");
        assert_eq!("12634632", solve(lines))
    }

    #[test]
    fn test_range_below() {
        let mut mapper = Mapper::new();
        mapper.insert("10 20 10");
        let maps = mapper.map_range(15, 5);

        assert_eq!(1, maps.len());
        assert_eq!((15, 5), *maps.get(0).unwrap());
    }

    #[test]
    fn test_range_lower_half() {
        let mut mapper = Mapper::new();
        mapper.insert("50 20 10");
        let maps = mapper.map_range(15, 10);

        assert_eq!(2, maps.len());
        assert_eq!((15, 5), *maps.get(0).unwrap());
        assert_eq!((50, 5), *maps.get(1).unwrap());
    }

    #[test]
    fn test_range_going_over() {
        let mut mapper = Mapper::new();
        mapper.insert("50 20 10");
        let maps = mapper.map_range(15, 22);

        assert_eq!(3, maps.len());
        assert_eq!((15, 5), *maps.get(0).unwrap());
        assert_eq!((50, 10), *maps.get(1).unwrap());
        assert_eq!((30, 7), *maps.get(2).unwrap());
    }

    #[test]
    fn test_range_all_inside() {
        let mut mapper = Mapper::new();
        mapper.insert("50 20 10");
        let maps = mapper.map_range(23, 3);

        assert_eq!(1, maps.len());
        assert_eq!((53, 3), *maps.get(0).unwrap());
    }

    #[test]
    fn test_range_upper_half() {
        let mut mapper = Mapper::new();
        mapper.insert("50 20 10");
        let maps = mapper.map_range(23, 13);

        assert_eq!(2, maps.len());
        assert_eq!((53, 7), *maps.get(0).unwrap());
        assert_eq!((30, 6), *maps.get(1).unwrap());
    }

    #[test]
    fn test_range_two_joint_sections() {
        let mut mapper = Mapper::new();
        mapper.insert("50 20 10");
        mapper.insert("70 30 9");
        let maps = mapper.map_range(23, 24);

        assert_eq!(3, maps.len());
        assert_eq!((53, 7), *maps.get(0).unwrap());
        assert_eq!((70, 9), *maps.get(1).unwrap());
        assert_eq!((39, 8), *maps.get(2).unwrap());
    }

    #[test]
    fn test_range_two_disjoint_sections() {
        let mut mapper = Mapper::new();
        mapper.insert("50 20 10");
        mapper.insert("70 38 9");
        let maps = mapper.map_range(13, 50);

        assert_eq!(5, maps.len());
        assert_eq!((13, 7), *maps.get(0).unwrap());
        assert_eq!((50, 10), *maps.get(1).unwrap());
        assert_eq!((30, 8), *maps.get(2).unwrap());
        assert_eq!((70, 9), *maps.get(3).unwrap());
        assert_eq!((47, 16), *maps.get(4).unwrap());
    }
}
