const DAY: &str = "day20";

use std::{
    collections::{HashMap, VecDeque},
    time::Instant,
};

use crate::utils::util::{self};

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
    let mut modules = parse_lines(lines);

    let mut count_low = 0;
    let mut count_hi = 0;
    for _ in 0..1000 {
        let mut signals = VecDeque::new();
        signals.push_front(Signal::new(Pulse::Low, "button", "broadcaster"));
        count_low += 1;
        while signals.len() != 0 {
            let signal = signals.pop_front().unwrap();
            match modules.get_mut(signal.target) {
                Some(module) => {
                    let new_signals = module.process(signal);
                    let count_new_low = new_signals
                        .iter()
                        .filter(|signal| signal.pulse == Pulse::Low)
                        .collect::<Vec<_>>()
                        .len();
                    let count_new_hi = new_signals.len() - count_new_low;

                    count_low += count_new_low;
                    count_hi += count_new_hi;
                    signals.extend(new_signals);
                }
                None => {}
            }
        }
    }

    (count_low * count_hi).to_string()
}

fn parse_lines<'a>(lines: &'a Vec<String>) -> HashMap<&'a str, Module<'a>> {
    let mut modules: HashMap<&'a str, Module<'a>> = lines
        .iter()
        .map(|line| {
            let mut chars = line.chars();
            match chars.next().unwrap() {
                '%' => {
                    let (name, outputs) = parse_module_str(chars.as_str());
                    (name, Module::Ff(FfMod::new(name, outputs)))
                }
                '&' => {
                    let (name, outputs) = parse_module_str(chars.as_str());
                    (name, Module::Cj(CjMod::new(name, outputs)))
                }
                _ => {
                    let (name, outputs) = parse_module_str(line);
                    (name, Module::Bc(BcMod::new(name, outputs)))
                }
            }
        })
        .collect();

    modules
        .iter()
        .flat_map(|(source, module)| {
            #[allow(suspicious_double_ref_op)]
            module
                .targets()
                .into_iter()
                .map(|target| (source.clone(), target.clone()))
                .collect::<Vec<(&str, &str)>>()
        })
        .collect::<Vec<(&str, &str)>>()
        .into_iter()
        .for_each(|(source, target)| match modules.get_mut(target) {
            Some(module) => module.source(source),
            None => {}
        });
    modules
}

fn parse_module_str(s: &str) -> (&str, Vec<&str>) {
    let (name, output_str) = s.split_once(" -> ").unwrap();
    (name, output_str.split(", ").collect())
}

enum Module<'a> {
    Bc(BcMod<'a>),
    Ff(FfMod<'a>),
    Cj(CjMod<'a>),
}

impl<'a> Module<'a> {
    fn targets(&self) -> &Vec<&'a str> {
        match self {
            Module::Bc(bc) => &bc.targets,
            Module::Ff(ff) => &ff.targets,
            Module::Cj(cj) => &cj.targets,
        }
    }

    fn source(&mut self, source: &'a str) {
        match self {
            Module::Bc(_) => {}
            Module::Ff(_) => {}
            Module::Cj(cj) => cj.source(source),
        }
    }

    fn process(&mut self, signal: Signal<'a>) -> Vec<Signal<'a>> {
        match self {
            Module::Bc(bc_mod) => bc_mod.process(signal),
            Module::Ff(ff_mod) => ff_mod.process(signal),
            Module::Cj(cj_mod) => cj_mod.process(signal),
        }
    }
}

struct BcMod<'a> {
    name: &'a str,
    targets: Vec<&'a str>,
}

impl<'a> BcMod<'a> {
    fn new(name: &'a str, outputs: Vec<&'a str>) -> BcMod<'a> {
        BcMod {
            name,
            targets: outputs,
        }
    }
    fn process(&mut self, input: Signal) -> Vec<Signal<'a>> {
        self.targets
            .iter()
            .map(|output| Signal {
                pulse: input.pulse.clone(),
                source: self.name,
                target: &output,
            })
            .collect()
    }
}

struct FfMod<'a> {
    name: &'a str,
    targets: Vec<&'a str>,
    high: bool,
}

impl<'a> FfMod<'a> {
    fn new(name: &'a str, outputs: Vec<&'a str>) -> FfMod<'a> {
        FfMod {
            name,
            targets: outputs,
            high: false,
        }
    }
    fn process(&mut self, signal: Signal) -> Vec<Signal<'a>> {
        match signal.pulse {
            Pulse::High => vec![],
            Pulse::Low => {
                self.high = !self.high;
                self.targets
                    .iter()
                    .map(|target| Signal::new(Pulse::from(self.high), self.name, &target))
                    .collect()
            }
        }
    }
}

struct CjMod<'a> {
    name: &'a str,
    targets: Vec<&'a str>,
    last_signal: HashMap<&'a str, Pulse>,
}

impl<'a> CjMod<'a> {
    fn new(name: &'a str, outputs: Vec<&'a str>) -> CjMod<'a> {
        CjMod {
            name,
            targets: outputs,
            last_signal: HashMap::new(),
        }
    }
}

impl<'a> CjMod<'a> {
    fn all_high(&self) -> bool {
        self.last_signal
            .iter()
            .all(|(_, pulse)| pulse == &Pulse::High)
    }

    fn source(&mut self, source_name: &'a str) {
        self.last_signal.insert(source_name, Pulse::Low);
    }

    fn process(&mut self, signal: Signal<'a>) -> Vec<Signal<'a>> {
        self.last_signal.insert(signal.source, signal.pulse);
        let pulse = match self.all_high() {
            true => Pulse::Low,
            false => Pulse::High,
        };
        self.targets
            .iter()
            .map(|target| Signal::new(pulse.clone(), self.name, target))
            .collect()
    }
}

#[derive(PartialEq, Clone)]
enum Pulse {
    High,
    Low,
}

impl Pulse {
    fn from(high: bool) -> Pulse {
        match high {
            true => Pulse::High,
            false => Pulse::Low,
        }
    }
}

struct Signal<'a> {
    pulse: Pulse,
    source: &'a str,
    target: &'a str,
}

impl<'a> Signal<'a> {
    fn new(pulse: Pulse, source: &'a str, target: &'a str) -> Signal<'a> {
        Signal {
            pulse,
            source,
            target,
        }
    }
}

fn buttons(module_name: &str, mut modules: HashMap<&str, Module<'_>>) -> i64 {
    let mut signals = VecDeque::new();
    let mut count = 0;
    loop {
        if signals.len() == 0 {
            signals.push_front(Signal::new(Pulse::Low, "button", "broadcaster"));
            count += 1;
        }

        let signal = signals.pop_front().unwrap();
        if signal.target == module_name && signal.pulse == Pulse::Low {
            break;
        }

        match modules.get_mut(signal.target) {
            Some(module) => {
                let new_signals = module.process(signal);
                signals.extend(new_signals);
            }
            None => {}
        }
    }
    count
}

fn part2(lines: &Vec<String>) -> String {
    let modules = parse_lines(lines);
    let (rx_source, _) = modules
        .iter()
        .filter(|(_, module)| module.targets().contains(&"rx"))
        .next()
        .unwrap();

    modules
        .iter()
        .filter(|(_, module)| module.targets().contains(rx_source))
        .map(|(source, _)| buttons(*source, parse_lines(lines)))
        .fold(1, |acc, num| acc * num)
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2, DAY};
    use crate::utils::util;

    #[test]
    fn test_part1_sample() {
        let lines = util::lines_in(&format!("./src/{}/input", DAY));
        assert_eq!("32000000", part1(&lines))
    }

    #[test]
    fn test_part1_sample2() {
        let lines = util::lines_in(&format!("./src/{}/input2", DAY));
        assert_eq!("11687500", part1(&lines))
    }

    #[test]
    fn test_part1_input() {
        let lines = util::lines_in(&format!("./src/{}/input1", DAY));
        assert_eq!("684125385", part1(&lines))
    }

    #[test]
    fn test_part2_input() {
        let lines = util::lines_in(&format!("./src/{}/input1", DAY));
        assert_eq!("225872806380073", part2(&lines));
    }
}
