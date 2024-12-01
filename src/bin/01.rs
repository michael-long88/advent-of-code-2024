advent_of_code::solution!(1);

pub fn parse(input: &str) -> Vec<Vec<u32>> {
    let location_ids = input
        .split('\n')
        .filter(|location_list| !location_list.is_empty())
        .map(|location_list| {
            location_list
                .split_ascii_whitespace()
                .map(|location_str| location_str.parse::<u32>().unwrap())
                .collect()
        })
        .collect();

    location_ids
}

pub fn part_one(input: &str) -> Option<u32> {
    let location_ids = parse(input);
    let mut left_list: Vec<&u32> = location_ids
        .iter()
        .map(|location_list| location_list.first().unwrap())
        .collect();
    let mut right_list: Vec<&u32> = location_ids
        .iter()
        .map(|location_list| location_list.last().unwrap())
        .collect();
    left_list.sort_unstable();
    right_list.sort_unstable();

    let total_differences: u32 = left_list
        .iter()
        .zip(right_list.iter())
        .map(|(left, right)| (**left as i32 - **right as i32).unsigned_abs())
        .sum();

    Some(total_differences)
}

pub fn part_two(input: &str) -> Option<u32> {
    let location_ids = parse(input);
    let left_list: Vec<&u32> = location_ids
        .iter()
        .map(|location_list| location_list.first().unwrap())
        .collect();
    let right_list: Vec<&u32> = location_ids
        .iter()
        .map(|location_list| location_list.last().unwrap())
        .collect();

    let similarity_score = left_list
        .iter()
        .map(|left| right_list.iter().filter(|right| **right == *left).count() as u32 * **left)
        .sum();

    Some(similarity_score)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
