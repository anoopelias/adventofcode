use crate::utils::parser::I64Parser;

struct Mapper {
    num_maps: Vec<(i64, i64, i64)>,
}

impl Mapper {
    fn new() -> Mapper {
        Mapper { num_maps: vec![] }
    }

    fn insert(&mut self, line: &str) {
        let mut num_maps = line.parse_i64();
        let num_map = (num_maps.remove(0), num_maps.remove(0), num_maps.remove(0));

        match self.num_maps.binary_search_by(|nm| nm.1.cmp(&num_map.1)) {
            Ok(_) => println!("These should not overlap"),
            Err(pos) => self.num_maps.insert(pos, num_map),
        }
    }

    fn map_range(&self, mut start: i64, len: i64) -> Vec<(i64, i64)> {
        let mut ranges = vec![];

        let to = start + len;
        for (dest, src, range) in self.num_maps.iter() {
            let tip = src + range;
            if to <= *src {
                break;
            }

            if start < *src {
                ranges.push((start, src - start));
                if to < tip {
                    ranges.push((*dest, to - src));
                    start = to;
                } else {
                    ranges.push((*dest, *range));
                    start = tip;
                }
            } else if start >= *src {
                let offset = start - src;
                if to < tip {
                    ranges.push((dest + offset, to - start));
                    start = to;
                } else if start < tip && to >= tip {
                    ranges.push((dest + offset, tip - start));
                    start = tip;
                }
            }
        }

        if start < to {
            ranges.push((start, to - start));
        }

        ranges
    }
}

pub(crate) fn solve(lines: Vec<String>) -> String {
    let splits = lines.get(0).unwrap().split(":").collect::<Vec<_>>();
    let seeds_numbers = splits.get(1).unwrap().parse_i64();
    let mut inputs: Vec<(i64, i64)> = seeds_numbers
        .chunks_exact(2)
        .map(|chunk| (chunk[0], chunk[1]))
        .collect();

    let mut mappers: Vec<Mapper> = (0..7).into_iter().map(|_| Mapper::new()).collect();

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
    use crate::utils::util;

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
