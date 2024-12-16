use std::collections::HashMap;

advent_of_code::solution!(11);

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
pub struct Stone {
    pub value: u64,
    pub n_digits: u32,
}

impl Stone {
    pub fn new(value: u64) -> Self {
        Self {
            value,
            n_digits: value.checked_ilog10().unwrap_or(0) + 1,
        }
    }

    #[inline]
    pub fn blink(&self) -> Vec<Stone> {
        if self.value == 0 {
            vec![Stone::new(1)]
        } else if self.n_digits % 2 == 0 {
            // Avoid string conversion for better performance
            let mut left_value = self.value;
            let div = 10_u64.pow(self.n_digits / 2);
            let right_value = left_value % div;
            left_value /= div;
            vec![Stone::new(left_value), Stone::new(right_value)]
        } else {
            vec![Stone::new(self.value.checked_mul(2024).unwrap_or(0))]
        }
    }
}

fn blink_counter(counts: HashMap<Stone, usize>, times: usize) -> HashMap<Stone, usize> {
    let mut counts = counts;

    for _ in 1..=times {
        let mut new_counts = HashMap::new();
        for (stone, &occurrences) in counts.iter() {
            for new_stone in stone.blink() {
                *new_counts.entry(new_stone).or_insert(0) += occurrences;
            }
        }
        counts = new_counts;
    }

    counts
}

pub fn parse(input: &str) -> Vec<Stone> {
    let stones = input
        .split_ascii_whitespace()
        .filter(|location_list| !location_list.is_empty())
        .map(|stone| {
            let value = stone.parse::<u64>().unwrap();
            Stone::new(value)
        })
        .collect();

    stones
}

pub fn part_one(input: &str) -> Option<usize> {
    let stones = parse(input);

    let mut initial_counts = HashMap::new();
    for stone in stones {
        *initial_counts.entry(stone).or_insert(0) += 1;
    }

    let final_counts = blink_counter(initial_counts, 25);

    Some(final_counts.values().sum())
}

pub fn part_two(input: &str) -> Option<usize> {
    let stones = parse(input);

    let mut initial_counts = HashMap::new();
    for stone in stones {
        *initial_counts.entry(stone).or_insert(0) += 1;
    }

    let final_counts = blink_counter(initial_counts, 75);

    Some(final_counts.values().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(65601038650482));
    }
}
