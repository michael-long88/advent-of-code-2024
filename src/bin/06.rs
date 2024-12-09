advent_of_code::solution!(6);

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

pub struct Grid {
    pub rows: Vec<GridRow>,
    pub guard: Guard,
}

impl Grid {
    pub fn new(rows: Vec<GridRow>, guard: Guard) -> Self {
        Grid { rows, guard }
    }

    pub fn update_grid(&mut self) {
        let new_direction = self.guard.get_new_direction(&self.rows);
        self.guard.direction = new_direction;
        self.rows[self.guard.row].cells[self.guard.col].cell_type = CellType::STEP;

        if !self.is_obstacle_in_direction(new_direction) {
            self.guard.move_in_direction();
        }
    }

    pub fn is_obstacle_in_direction(&self, direction: Direction) -> bool {
        let (new_row, new_col) = match direction {
            Direction::UP => (self.guard.row.saturating_sub(1), self.guard.col),
            Direction::RIGHT => (self.guard.row, self.guard.col + 1),
            Direction::DOWN => (self.guard.row + 1, self.guard.col),
            Direction::LEFT => (self.guard.row, self.guard.col.saturating_sub(1)),
        };

        if new_row == 0
            || new_row == self.rows.len() - 1
            || new_col == 0
            || new_col == self.rows[0].cells.len() - 1
        {
            return false;
        }

        if new_row < self.rows.len() && new_col < self.rows[0].cells.len() {
            self.rows[new_row].cells[new_col].cell_type == CellType::OBSTACLE
        } else {
            true
        }
    }
}

pub struct GridRow {
    pub cells: Vec<GridCell>,
}

impl GridRow {
    pub fn new(cells: Vec<GridCell>) -> Self {
        GridRow { cells }
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum CellType {
    EMPTY,
    OBSTACLE,
    STEP,
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct GridCell {
    cell_type: CellType,
    row: usize,
    col: usize,
}

pub struct Guard {
    pub direction: Direction,
    pub row: usize,
    pub col: usize,
}

impl Guard {
    pub fn new(row: usize, col: usize) -> Self {
        Guard {
            direction: Direction::UP,
            row,
            col,
        }
    }

    pub fn get_new_direction(&self, rows: &[GridRow]) -> Direction {
        match self.direction {
            Direction::UP => {
                if rows[self.row - 1].cells[self.col].cell_type == CellType::OBSTACLE {
                    Direction::RIGHT
                } else {
                    Direction::UP
                }
            }
            Direction::RIGHT => {
                if rows[self.row].cells[self.col + 1].cell_type == CellType::OBSTACLE {
                    Direction::DOWN
                } else {
                    Direction::RIGHT
                }
            }
            Direction::DOWN => {
                if rows[self.row + 1].cells[self.col].cell_type == CellType::OBSTACLE {
                    Direction::LEFT
                } else {
                    Direction::DOWN
                }
            }
            Direction::LEFT => {
                if rows[self.row].cells[self.col - 1].cell_type == CellType::OBSTACLE {
                    Direction::UP
                } else {
                    Direction::LEFT
                }
            }
        }
    }

    pub fn move_in_direction(&mut self) {
        match self.direction {
            Direction::UP => self.row -= 1,
            Direction::RIGHT => self.col += 1,
            Direction::DOWN => self.row += 1,
            Direction::LEFT => self.col -= 1,
        }
    }

    pub fn is_at_exit(&self, rows: &[GridRow]) -> bool {
        self.row == 0
            || self.row == rows.len() - 1
            || self.col == 0
            || self.col == rows[0].cells.len() - 1
    }
}

pub fn parse(input: &str) -> Grid {
    let mut guard = Guard::new(0, 0);
    let rows = input
        .split('\n')
        .filter(|row| !row.is_empty())
        .enumerate()
        .map(|(row_index, row)| {
            let grid_row: Vec<GridCell> = row
                .chars()
                .enumerate()
                .map(|(col_index, cell): (usize, char)| match cell {
                    '.' => GridCell {
                        cell_type: CellType::EMPTY,
                        row: row_index,
                        col: col_index,
                    },
                    '#' => GridCell {
                        cell_type: CellType::OBSTACLE,
                        row: row_index,
                        col: col_index,
                    },
                    '^' => {
                        guard =
                            Guard::new(row_index, row.chars().position(|ch| ch == '^').unwrap());
                        GridCell {
                            cell_type: CellType::STEP,
                            row: row_index,
                            col: col_index,
                        }
                    }
                    _ => panic!("Unknown character in grid"),
                })
                .collect();
            GridRow::new(grid_row)
        })
        .collect();

    Grid { rows, guard }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut grid = parse(input);

    while !grid.guard.is_at_exit(&grid.rows) {
        grid.update_grid();
    }

    let total_steps = grid.rows.iter().fold(0, |acc, row| {
        acc + row
            .cells
            .iter()
            .filter(|cell| cell.cell_type == CellType::STEP)
            .count()
    });

    Some(total_steps + 1)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut grid = parse(input);
    let starting_guard_position = (grid.guard.row, grid.guard.col);

    while !grid.guard.is_at_exit(&grid.rows) {
        grid.update_grid();
    }

    let mut step_cells: Vec<GridCell> = grid
        .rows
        .iter()
        .flat_map(|row| {
            row.cells
                .iter()
                .filter(|cell| {
                    cell.cell_type == CellType::STEP
                        && (cell.row != starting_guard_position.0
                            || cell.col != starting_guard_position.1)
                })
                .copied()
                .collect::<Vec<GridCell>>()
        })
        .collect();

    step_cells.push(GridCell {
        cell_type: CellType::STEP,
        row: grid.guard.row,
        col: grid.guard.col,
    });

    let mut valid_new_obstacles = 0;

    for step in step_cells {
        let mut grid = parse(input);

        grid.rows[step.row].cells[step.col] = GridCell {
            cell_type: CellType::OBSTACLE,
            row: step.row,
            col: step.col,
        };

        let mut total_steps = 0;
        let grid_length = grid.rows.len();
        let max_steps = grid_length * grid_length;

        'guard_loop: while !grid.guard.is_at_exit(&grid.rows) {
            grid.update_grid();
            total_steps += 1;
            if total_steps >= max_steps {
                valid_new_obstacles += 1;
                break 'guard_loop;
            }
        }
    }

    Some(valid_new_obstacles)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
