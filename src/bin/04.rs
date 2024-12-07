advent_of_code::solution!(4);

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Letter {
    X,
    M,
    A,
    S,
}

impl From<char> for Letter {
    fn from(c: char) -> Self {
        match c {
            'X' => Letter::X,
            'M' => Letter::M,
            'A' => Letter::A,
            'S' => Letter::S,
            _ => panic!("Invalid letter"),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct ElfLetter {
    pub row: usize,
    pub column: usize,
    pub letter: Letter
}

impl ElfLetter {
    pub fn new(row: usize, column: usize, letter: Letter) -> Self {
        Self {
            row,
            column,
            letter,
        }
    }

    pub fn is_part_one_valid_distance(&self, next: &ElfLetter, x_letter: &ElfLetter) -> bool {
        if next.letter == Letter::A || next.letter == Letter::S {
            let (first_direction, second_direction) = self.get_directions(x_letter, next);
    
            if first_direction != second_direction {
                return false;
            }
        }

        let (row_distance, column_distance) = self.get_distances(next);

        row_distance <= 1 && column_distance <= 1
    }

    pub fn is_part_two_valid_distance(&self, next: &ElfLetter, m_letter: &ElfLetter) -> bool {
        if next.letter == Letter::S {
            let (first_direction, second_direction) = self.get_directions(m_letter, next);
    
            if first_direction != second_direction {
                return false;
            }
        }

        let (row_distance, column_distance) = self.get_distances(next);

        row_distance == 1 && column_distance == 1
    }

    fn get_directions(&self, first: &ElfLetter, next: &ElfLetter) -> (isize, isize) {
        let first_direction = (first.row as isize - self.row as isize, first.column as isize - self.column as isize);
        let next_direction = (next.row as isize - self.row as isize, next.column as isize - self.column as isize);

        (first_direction.0 * next_direction.1, first_direction.1 * next_direction.0)
    }

    fn get_distances(&self, next: &ElfLetter) -> (usize, usize) {
        let row_distance = if self.row > next.row {
            self.row - next.row
        } else {
            next.row - self.row
        };
        let column_distance = if self.column > next.column {
            self.column - next.column
        } else {
            next.column - self.column
        };
            
        (row_distance, column_distance)
    }
}

pub fn parse(input: &str) -> Vec<ElfLetter> {
    let elf_letters = input
        .split('\n')
        .filter(|row| !row.is_empty())
        .enumerate()
        .flat_map(|(row_index, row)| {
            row
                .split("")
                .skip(1)
                .take(row.len())
                .enumerate()
                .map(|(col_index, elf_letter)| {
                    let letter = Letter::from(elf_letter.chars().next().unwrap());
                    ElfLetter::new(row_index, col_index, letter)
                })
                .collect::<Vec<ElfLetter>>()
        })
        .collect();

        elf_letters
}

pub fn part_one(input: &str) -> Option<u32> {
    let elf_letters = parse(input);

    let x_letters: Vec<&ElfLetter> = elf_letters.iter().filter(|elf_letter| elf_letter.letter == Letter::X).collect();
    let m_letters: Vec<&ElfLetter> = elf_letters.iter().filter(|elf_letter| elf_letter.letter == Letter::M).collect();
    let a_letters: Vec<&ElfLetter> = elf_letters.iter().filter(|elf_letter| elf_letter.letter == Letter::A).collect();
    let s_letters: Vec<&ElfLetter> = elf_letters.iter().filter(|elf_letter| elf_letter.letter == Letter::S).collect();

    let mut valid_word_counts = 0;

    x_letters
        .iter()
        .for_each(|elf_letter| {
            for m_letter in m_letters.iter() {
                if elf_letter.is_part_one_valid_distance(m_letter, elf_letter) {
                    for a_letter in a_letters.iter() {
                        if m_letter.is_part_one_valid_distance(a_letter, elf_letter) {
                            for s_letter in s_letters.iter() {
                                if a_letter.is_part_one_valid_distance(s_letter, elf_letter) {
                                    valid_word_counts += 1
                                }
                            }
                        }
                    }
                }
            }
        });

    Some(valid_word_counts as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let elf_letters = parse(input);

    let m_letters: Vec<&ElfLetter> = elf_letters.iter().filter(|elf_letter| elf_letter.letter == Letter::M).collect();
    let a_letters: Vec<&ElfLetter> = elf_letters.iter().filter(|elf_letter| elf_letter.letter == Letter::A).collect();
    let s_letters: Vec<&ElfLetter> = elf_letters.iter().filter(|elf_letter| elf_letter.letter == Letter::S).collect();

    let mut diagonal_words = vec![];

    m_letters
        .iter()
        .for_each(|elf_letter| {
            for a_letter in a_letters.iter() {
                if elf_letter.is_part_two_valid_distance(a_letter, elf_letter) {
                    for s_letter in s_letters.iter() {
                        if a_letter.is_part_two_valid_distance(s_letter, elf_letter) {
                            diagonal_words.push([elf_letter, a_letter, s_letter]);
                        }
                    }
                }
            }
        });

    let mut total_num_xmas = 0;

    for (index, diagonal_word) in diagonal_words.iter().enumerate() {
        for word in diagonal_words[(index + 1)..].iter() {
            if diagonal_word[1].row == word[1].row && diagonal_word[1].column == word[1].column {
                total_num_xmas += 1;
            }
        }
    }

    Some(total_num_xmas as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
