use pathfinding::prelude::{bfs_reach, count_paths};

advent_of_code::solution!(10);

pub fn count_reachable_nines(grid: &[Vec<i32>], part1: bool) -> usize {
    // Find all starting positions (zeros)
    let zeros: Vec<(usize, usize)> = grid
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, &val)| val == 0)
                .map(move |(j, _)| (i, j))
        })
        .collect();

    let grid_size = grid.len();

    // Function to get valid next positions
    let successors = |(x, y): &(usize, usize)| {
        let curr_val = grid[*x][*y];
        let target_val = curr_val + 1;

        // Define possible moves (up, down, left, right)
        [(0, 1), (1, 0), (0, -1), (-1, 0)]
            .iter()
            .filter_map(|&(dx, dy)| {
                let new_x = (*x as i32 + dx) as usize;
                let new_y = (*y as i32 + dy) as usize;

                if new_x < grid_size && new_y < grid_size && grid[new_x][new_y] == target_val {
                    Some((new_x, new_y))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
    };

    if part1 {
        // Use bfs_reach to find all reachable positions from all zeros
        let reachable_nines: usize = zeros
            .iter()
            .map(|&start| {
                let reachable_nodes: Vec<_> = bfs_reach(start, &successors).collect();

                reachable_nodes
                    .into_iter()
                    .filter(|&pos| grid[pos.0][pos.1] == 9)
                    .count()
            })
            .sum();

        reachable_nines
    } else {
        // Success function to identify when we've reached a 9
        let success = |&(x, y): &(usize, usize)| grid[x][y] == 9;

        // Count paths from each 0 to any 9
        zeros
            .iter()
            .map(|&start| count_paths(start, successors, success))
            .sum()
    }
}

pub fn parse(input: &str) -> Vec<Vec<i32>> {
    let grid_rows = input
        .split('\n')
        .filter(|row| !row.is_empty())
        .map(|row| {
            row.chars()
                .map(|elevation| elevation.to_digit(10).unwrap() as i32)
                .collect()
        })
        .collect();

    grid_rows
}

pub fn part_one(input: &str) -> Option<usize> {
    let grid_rows = parse(input);
    let reachable_nines = count_reachable_nines(&grid_rows, true);

    Some(reachable_nines)
}

pub fn part_two(input: &str) -> Option<usize> {
    let grid_rows = parse(input);
    let reachable_nines = count_reachable_nines(&grid_rows, false);

    Some(reachable_nines)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
