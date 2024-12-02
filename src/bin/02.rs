advent_of_code::solution!(2);

pub fn parse(input: &str) -> Vec<Vec<i32>> {
    let reports = input
        .split('\n')
        .filter(|report| !report.is_empty())
        .map(|report| {
            report
                .split_ascii_whitespace()
                .map(|level| level.parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    reports
}

pub fn is_safe(report: &[i32]) -> bool {
    let only_decreasing = report
        .windows(2)
        .all(|level_window| level_window[0] > level_window[1]);
    let only_increasing = report
        .windows(2)
        .all(|level_window| level_window[0] < level_window[1]);
    let distance_ok = report
        .windows(2)
        .all(|level_window| (1..=3).contains(&(level_window[0] - level_window[1]).abs()));

    (only_decreasing ^ only_increasing) && distance_ok
}

pub fn is_safe_with_removal(report: &[i32]) -> bool {
    if is_safe(report) {
        return true;
    }

    for level_index in 0..report.len() {
        let mut report_clone = report.to_vec();
        report_clone.remove(level_index);

        if is_safe(&report_clone) {
            return true;
        }
    }

    false
}

pub fn part_one(input: &str) -> Option<usize> {
    let reports = parse(input);

    let safe_report_count = reports.iter().filter(|report| is_safe(report)).count();

    Some(safe_report_count)
}

pub fn part_two(input: &str) -> Option<usize> {
    let reports = parse(input);

    let safe_report_count = reports
        .iter()
        .filter(|report| is_safe_with_removal(report))
        .count();

    Some(safe_report_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
