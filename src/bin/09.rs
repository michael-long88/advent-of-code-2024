advent_of_code::solution!(9);

pub struct File {
    pub id: u32,
    pub size: u32,
    pub free_blocks: u32,
}

pub fn parse(input: &str) -> Vec<File> {
    let files = input
        .chars()
        .collect::<Vec<char>>()
        .chunks(2)
        .enumerate()
        .map(|(index, file_chunk)| {
            let size = file_chunk[0].to_digit(10).unwrap();
            let mut free_blocks = 0;
            if file_chunk.len() == 2 {
                free_blocks = file_chunk[1].to_digit(10).unwrap();
            }
            File {
                id: index as u32,
                size,
                free_blocks,
            }
        })
        .collect();

    files
}

pub fn create_filesystem(files: &[File]) -> Vec<String> {
    let mut filesystem: Vec<String> = Vec::new();

    for file in files {
        let stringified_file = vec![file.id.to_string(); file.size as usize];
        let stringified_free_space = vec![".".to_string(); file.free_blocks as usize];
        filesystem.extend(stringified_file);
        filesystem.extend(stringified_free_space);
    }

    filesystem
}

pub fn part_one(input: &str) -> Option<u64> {
    let files = parse(input);
    let filesystem = create_filesystem(&files);

    let free_positions: Vec<usize> = filesystem
        .clone()
        .iter()
        .enumerate()
        .filter_map(|(index, block)| if block == "." { Some(index) } else { None })
        .collect();

    let mut file_blocks: Vec<String> = filesystem
        .iter()
        .filter(|block| *block != ".")
        .cloned()
        .collect();

    for free_position in free_positions {
        if free_position > file_blocks.len() {
            break;
        }
        if let Some(last_block) = file_blocks.pop() {
            file_blocks.insert(free_position, last_block);
        }
    }

    let checksum = file_blocks
        .iter()
        .enumerate()
        .filter_map(|(index, block)| {
            if block != "." {
                Some(block.parse::<u64>().unwrap() * index as u64)
            } else {
                None
            }
        })
        .sum();

    Some(checksum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let files = parse(input);
    let mut filesystem = create_filesystem(&files);

    for file in files.iter().rev().take(files.len() - 1) {
        let id_str = file.id.to_string();

        // Find first sequence of dots that's large enough
        let mut start_index = 0;
        'outer: while start_index < filesystem.len() {
            let mut space_size = 0;
            let mut space_start = None;

            // Count contiguous dots
            for (i, block) in filesystem[start_index..].iter().enumerate() {
                if block == "." {
                    if space_start.is_none() {
                        space_start = Some(start_index + i);
                    }
                    space_size += 1;
                } else {
                    if space_size >= file.size {
                        break;
                    }
                    space_size = 0;
                    space_start = None;
                }
            }
            // If we found a space large enough
            if space_size >= file.size {
                if let Some(space_start) = space_start {
                    let original_position = filesystem
                        .iter()
                        .position(|block| *block == id_str)
                        .unwrap();

                    if space_start > original_position {
                        break 'outer;
                    }

                    for i in 0..file.size {
                        filesystem[space_start + i as usize] = id_str.clone();
                        filesystem[original_position + i as usize] = ".".to_string();
                    }
                    break 'outer;
                }
            }

            start_index += 1;
        }
    }

    let checksum = filesystem
        .iter()
        .enumerate()
        .filter_map(|(index, block)| {
            if block != "." {
                Some(block.parse::<u64>().unwrap() * index as u64)
            } else {
                None
            }
        })
        .sum();

    Some(checksum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
