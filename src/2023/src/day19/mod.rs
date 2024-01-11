const DAY: &str = "day19";

use std::{collections::HashMap, time::Instant};

use crate::utils::{
    iter::GroupLines,
    string::WrapperRemover,
    util::{self},
};

#[allow(unused)]
pub(crate) fn solve() -> String {
    let lines = util::lines_in(&format!("./src/{}/input1", DAY));
    let time = Instant::now();
    let part1 = part1(&lines);
    let elapsed1 = time.elapsed();

    let time = Instant::now();
    let part2 = part2(&lines);
    let elapsed2 = time.elapsed();
    return format!(
        "result1: {} in {:?} \nresult2: {} in {:?}",
        part1, elapsed1, part2, elapsed2,
    );
}

fn part1(lines: &Vec<String>) -> String {
    let mut groups = lines.group_lines();
    let part_strings = groups.pop().unwrap();
    let workflows = groups
        .pop()
        .unwrap()
        .iter()
        .map(|workflow_string| Workflow::from(workflow_string))
        .collect::<Vec<_>>();

    let mut workflow_map = HashMap::new();

    workflows.iter().for_each(|workflow| {
        workflow_map.insert(workflow.name.clone(), workflow);
    });

    let parts = part_strings
        .iter()
        .map(|part_string| Part::from(part_string));

    let mut sum = 0;
    for part in parts {
        let workflow = workflow_map.get("in");
        let mut response = workflow.unwrap().eval(&part);

        while response.final_result.is_none() {
            let next_workflow = workflow_map.get(response.next.as_ref().unwrap()).unwrap();
            response = next_workflow.eval(&part);
        }

        let result = response.final_result.as_ref().unwrap();

        if *result == FinalResult::A {
            sum += part.sum();
        }
    }

    sum.to_string()
}

fn part2(lines: &Vec<String>) -> String {
    "".to_string()
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum Category {
    x,
    m,
    a,
    s,
}

impl Category {
    fn from(c: char) -> Category {
        match c {
            'x' => Category::x,
            'm' => Category::m,
            'a' => Category::a,
            's' => Category::s,
            _ => panic!("Invalid category"),
        }
    }
}

#[derive(Debug, PartialEq)]
enum Op {
    lt,
    gt,
}

impl Op {
    fn from(c: char) -> Op {
        match c {
            '<' => Op::lt,
            '>' => Op::gt,
            _ => panic!("Invalid op"),
        }
    }
}

#[derive(Debug, PartialEq)]
enum FinalResult {
    A,
    R,
}

impl FinalResult {
    fn from(c: char) -> Option<FinalResult> {
        match c {
            'A' => Some(FinalResult::A),
            'R' => Some(FinalResult::R),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq)]
struct Response {
    final_result: Option<FinalResult>,
    next: Option<String>,
}

impl Response {
    fn from(str: &str) -> Response {
        match FinalResult::from(str.chars().next().unwrap()) {
            Some(final_result) => Response {
                final_result: Some(final_result),
                next: None,
            },
            None => Response {
                final_result: None,
                next: Some(str.to_string()),
            },
        }
    }
}

#[derive(Debug, PartialEq)]
struct Rule {
    category: Category,
    op: Op,
    num: i32,
    response: Response,
}

impl Rule {
    fn from(str: &str) -> Rule {
        let mut chars = str.chars();
        let category = Category::from(chars.next().unwrap());
        let op = Op::from(chars.next().unwrap());
        let (num_str, resp_str) = chars.as_str().split_once(":").unwrap();
        let response = Response::from(resp_str);
        let num = num_str.parse::<i32>().unwrap();
        Rule {
            category,
            op,
            num,
            response,
        }
    }

    fn eval(&self, part: &Part) -> Option<&Response> {
        let val = part.cartegory_values.get(&self.category).unwrap();
        match self.op {
            Op::lt => {
                if val < &self.num {
                    Some(&self.response)
                } else {
                    None
                }
            }
            Op::gt => {
                if val > &self.num {
                    Some(&self.response)
                } else {
                    None
                }
            }
        }
    }
}

#[derive(Debug, PartialEq)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
    otherwise: Response,
}

impl Workflow {
    fn from(line: &str) -> Workflow {
        let (name_str, rules_str) = line.split_once("{").unwrap();
        let name = name_str.to_string();
        let mut rule_strs = rules_str[0..rules_str.len() - 1]
            .split(",")
            .collect::<Vec<_>>();

        let otherwise = Response::from(rule_strs.pop().unwrap());
        let rules = rule_strs
            .iter()
            .map(|rule_str| Rule::from(rule_str))
            .collect();

        Workflow {
            name,
            rules,
            otherwise,
        }
    }

    fn eval(&self, part: &Part) -> &Response {
        for rule in &self.rules {
            if let Some(response) = rule.eval(part) {
                return response;
            }
        }
        &self.otherwise
    }
}

#[derive(Debug, PartialEq)]
struct Part {
    cartegory_values: HashMap<Category, i32>,
}

impl Part {
    fn from(str: &str) -> Part {
        let mut cartegory_values = HashMap::new();
        for pair in str.remove_wrapping().split(",") {
            let (cat_str, val_str) = pair.split_once("=").unwrap();
            let cat = Category::from(cat_str.chars().next().unwrap());
            let val = val_str.parse::<i32>().unwrap();
            cartegory_values.insert(cat, val);
        }
        Part { cartegory_values }
    }

    fn sum(&self) -> i32 {
        self.cartegory_values
            .iter()
            .map(|(_, val)| val)
            .sum::<i32>()
    }
}

#[cfg(test)]
mod tests {
    use super::{part1, part2, Category, FinalResult, Op, Response, Rule, Workflow, DAY};
    use crate::{day19::Part, utils::util};

    #[test]
    fn test_parser_workflow() {
        let expected = Workflow {
            name: "px".to_string(),
            rules: vec![
                Rule {
                    category: Category::a,
                    op: Op::lt,
                    num: 2006,
                    response: Response {
                        final_result: None,
                        next: Some("qkq".to_string()),
                    },
                },
                Rule {
                    category: Category::m,
                    op: Op::gt,
                    num: 2090,
                    response: Response {
                        final_result: Some(FinalResult::A),
                        next: None,
                    },
                },
            ],
            otherwise: Response {
                final_result: None,
                next: Some("rfg".to_string()),
            },
        };

        assert_eq!(expected, Workflow::from("px{a<2006:qkq,m>2090:A,rfg}"));
    }

    #[test]
    fn test_parse_part() {
        let expected = Part {
            cartegory_values: vec![
                (Category::x, 787),
                (Category::m, 2655),
                (Category::a, 1222),
                (Category::s, 2876),
            ]
            .into_iter()
            .collect(),
        };

        assert_eq!(expected, Part::from("{x=787,m=2655,a=1222,s=2876}"));
    }

    #[test]
    fn test_part1_sample() {
        let lines = util::lines_in(&format!("./src/{}/input", DAY));
        assert_eq!("19114", part1(&lines))
    }

    #[test]
    fn test_part1_input() {
        let lines = util::lines_in(&format!("./src/{}/input1", DAY));
        assert_eq!("386787", part1(&lines))
    }

    #[test]
    fn test_part2_sample() {
        // let lines = util::lines_in(&format!("./src/{}/input", DAY));
        // assert_eq!("952408144115", part2(&lines));
    }

    #[test]
    fn test_part2_input() {
        // let lines = util::lines_in(&format!("./src/{}/input1", DAY));
        // assert_eq!("66296566363189", part2(&lines));
    }
}
