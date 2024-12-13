use std::collections::HashSet;

advent_of_code::solution!(8);

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct Node {
    pub frequency: String,
    pub row: usize,
    pub col: usize,
}

impl Node {
    pub fn new(frequency: String, row: usize, col: usize) -> Self {
        Node {
            frequency,
            row,
            col,
        }
    }

    pub fn get_antinodes(&self, other: &Node, grid_size: i32, part1: bool) -> Vec<Node> {
        let mut antinodes = Vec::new();
        let dx = self.row as i32 - other.row as i32;
        let dy = self.col as i32 - other.col as i32;

        let add_antinodes = |start_row: i32, start_col: i32, dx: i32, dy: i32| {
            let mut antinode_row = start_row;
            let mut antinode_col = start_col;

            let mut local_antinodes = Vec::new();
            loop {
                antinode_row += dx;
                antinode_col += dy;

                if antinode_row < 0
                    || antinode_row >= grid_size
                    || antinode_col < 0
                    || antinode_col >= grid_size
                {
                    break;
                }

                local_antinodes.push(Node::new(
                    "#".to_string(),
                    antinode_row as usize,
                    antinode_col as usize,
                ));
            }

            local_antinodes
        };

        let mut first_half = add_antinodes(self.row as i32, self.col as i32, dx, dy);
        let mut second_half = add_antinodes(other.row as i32, other.col as i32, -dx, -dy);

        if part1 {
            if !first_half.is_empty() {
                antinodes.push(first_half[0].clone());
            }
            if !second_half.is_empty() {
                antinodes.push(second_half[0].clone());
            }
        } else {
            antinodes.append(&mut first_half);
            antinodes.append(&mut second_half);
        }
        antinodes
    }
}

pub fn parse(input: &str) -> (i32, Vec<Node>) {
    let grid_size = input.lines().count() as i32;
    let nodes = input
        .split('\n')
        .filter(|row| !row.is_empty())
        .enumerate()
        .filter_map(|(row_index, row)| {
            let nodes = row
                .chars()
                .enumerate()
                .filter_map(|(col_index, ch)| {
                    if ch == '.' {
                        None
                    } else {
                        let frequency = ch.to_string();
                        Some(Node::new(frequency, row_index, col_index))
                    }
                })
                .collect::<Vec<Node>>();
            if !nodes.is_empty() {
                Some(nodes)
            } else {
                None
            }
        })
        .flatten()
        .collect();

    (grid_size, nodes)
}

pub fn part_one(input: &str) -> Option<usize> {
    let (grid_size, nodes) = parse(input);
    let frequencies = nodes
        .iter()
        .map(|node| node.frequency.clone())
        .collect::<Vec<String>>();

    let mut antinodes = HashSet::new();

    frequencies.iter().for_each(|frequency| {
        let current_nodes = nodes
            .iter()
            .filter(|node| node.frequency == *frequency)
            .collect::<Vec<&Node>>();

        for (index, node) in current_nodes.iter().enumerate() {
            for other_node in &current_nodes[index + 1..] {
                let new_antinodes = node.get_antinodes(other_node, grid_size, true);
                for antinode in new_antinodes {
                    antinodes.insert(antinode.clone());
                }
            }
        }
    });

    antinodes.retain(|node| node.col != 99 && node.row != 99);

    Some(antinodes.len())
}

pub fn part_two(input: &str) -> Option<usize> {
    let (grid_size, nodes) = parse(input);
    let frequencies = nodes
        .iter()
        .map(|node| node.frequency.clone())
        .collect::<Vec<String>>();

    let mut antinodes = HashSet::new();

    frequencies.iter().for_each(|frequency| {
        let current_nodes = nodes
            .iter()
            .filter(|node| node.frequency == *frequency)
            .collect::<Vec<&Node>>();

        for (index, node) in current_nodes.iter().enumerate() {
            for other_node in &current_nodes[index + 1..] {
                let new_antinodes = node.get_antinodes(other_node, grid_size, false);
                for antinode in new_antinodes {
                    antinodes.insert(antinode.clone());
                }
            }
        }
    });

    for node in nodes {
        antinodes.insert(Node::new("#".to_owned(), node.row, node.col));
    }

    antinodes.retain(|node| node.col != 99 && node.row != 99);

    Some(antinodes.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
