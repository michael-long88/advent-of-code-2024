use regex::Regex;

advent_of_code::solution!(3);

#[derive(Debug)]
pub struct Instruction {
    pub enabled: bool,
    pub first: u32,
    pub second: u32,
}

impl Instruction {
    pub fn new(enabled: bool, first: u32, second: u32) -> Self {
        Self {
            enabled,
            first,
            second,
        }
    }

    pub fn product(&self, part_one: bool) -> u32 {
        if self.enabled || part_one {
            self.first * self.second
        } else {
            0
        }
    }
}

pub fn parse(input: &str) -> Vec<Vec<Instruction>> {
    let re = Regex::new(r"(?:(?:(?:do\(\)|don't\(\)).*?)?mul\((\d{1,3}),(\d{1,3})\))").unwrap();
    let mut enabled = true;
    let memory_lines = input
        .split('\n')
        .filter(|memory| !memory.is_empty())
        .map(|memory| {
            re.captures_iter(memory)
                .map(|capture| {
                    // Need to update this so that `enabled` propogates until a new do() or don't() is found
                    let full_match = capture.get(0).unwrap().as_str();
                    enabled = if full_match.contains("do()") {
                        true
                    } else if full_match.contains("don't()") {
                        false
                    } else {
                        enabled
                    };
                    Instruction::new(
                        enabled,
                        capture.get(1).unwrap().as_str().parse::<u32>().unwrap(),
                        capture.get(2).unwrap().as_str().parse::<u32>().unwrap(),
                    )
                })
                .collect::<Vec<Instruction>>()
        })
        .collect();

    memory_lines
}

pub fn part_one(input: &str) -> Option<u32> {
    let memory_lines = parse(input);

    let total = memory_lines
        .iter()
        .map(|memory| {
            memory
                .iter()
                .map(|instruction| instruction.product(true))
                .sum::<u32>()
        })
        .sum::<u32>();

    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let memory_lines = parse(input);

    let total = memory_lines
        .iter()
        .map(|memory| {
            memory
                .iter()
                .map(|instruction| instruction.product(false))
                .sum::<u32>()
        })
        .sum::<u32>();

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
