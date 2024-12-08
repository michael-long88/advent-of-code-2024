use std::collections::HashMap;

advent_of_code::solution!(5);

pub fn parse(input: &str) -> (HashMap<u32, Vec<u32>>, Vec<Vec<u32>>) {
    let sections: Vec<&str> = input
        .split("\n\n")
        .filter(|section| !section.is_empty())
        .collect();

    let mut rules: HashMap<u32, Vec<u32>> = HashMap::new();

    sections[0]
        .split('\n')
        .filter(|rule| !rule.is_empty())
        .for_each(|rule| {
            let line_rules = rule
                .split("|")
                .map(|rule| rule.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();
            rules.entry(line_rules[0]).or_default().push(line_rules[1]);
        });

    let pages = sections[1]
        .split('\n')
        .filter(|page| !page.is_empty())
        .map(|page| {
            page.split(",")
                .map(|page| page.parse::<u32>().unwrap())
                .collect()
        })
        .collect();

    (rules, pages)
}

pub fn get_page_set_checks(rules: &HashMap<u32, Vec<u32>>, pages: &[Vec<u32>]) -> Vec<bool> {
    pages
        .iter()
        .map(|page_set| {
            let mut valid = true;
            let reversed_set: Vec<&u32> = page_set.iter().rev().collect();
            'outer_loop: for (index, page) in reversed_set.iter().enumerate() {
                for check_page in &reversed_set[(index + 1)..] {
                    if rules.get(page).unwrap_or(&vec![]).contains(check_page) {
                        valid = false;
                        break 'outer_loop;
                    }
                }
            }
            valid
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let (rules, pages) = parse(input);

    let page_set_checks = get_page_set_checks(&rules, &pages);
    let page_number_sum: u32 = pages
        .iter()
        .enumerate()
        .filter(|(index, _)| page_set_checks[*index])
        .map(|(_, page_set)| page_set[(page_set.len() - 1) / 2])
        .sum();

    Some(page_number_sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (rules, pages) = parse(input);

    let page_set_checks = get_page_set_checks(&rules, &pages);
    let page_number_sum: u32 = pages
        .iter()
        .enumerate()
        .filter(|(index, _)| !page_set_checks[*index])
        .map(|(_, page_set)| {
            let mut sorted_pages = page_set.clone();
            sorted_pages.sort_by(|a, b| {
                let default_value = Vec::new();
                let a_rules = rules.get(a).unwrap_or(&default_value);
                if a_rules.contains(b) {
                    return std::cmp::Ordering::Less;
                }
                std::cmp::Ordering::Equal
            });
            sorted_pages[(page_set.len() - 1) / 2]
        })
        .sum();

    Some(page_number_sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
