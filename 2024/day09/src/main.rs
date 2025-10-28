use std::collections::HashSet;

use utils::parse_input;

#[derive(Clone, Copy)]
struct FileBlock {
    file_id: i64,
    file_size: usize,
    is_free: bool,
}

struct DiskMap {
    blocks: Vec<FileBlock>,
}

fn parse_input_entries(input: String) -> DiskMap {
    let mut disk_map_data = Vec::<FileBlock>::new();

    let mut is_free_space = false;
    let mut cur_file_id = 0;

    for c in input.chars() {
        if !c.is_digit(10) && c != '\n' {
            panic!("Invalid character in input: {}", c);
        }

        let n = c.to_digit(10).unwrap();
        for _ in 0..n {
            disk_map_data.push(FileBlock {
                file_id: if is_free_space { -1 } else { cur_file_id },
                file_size: n as usize,
                is_free: is_free_space,
            });
        }

        cur_file_id += if is_free_space { 0 } else { 1 };
        is_free_space = !is_free_space;
    }

    DiskMap { blocks: disk_map_data }
}

fn compact_disk_map_with_fragmentation(disk_map: &DiskMap) -> DiskMap {
    let mut new_blocks = Vec::<FileBlock>::new();

    let mut current_blocks_rev_iter = disk_map.blocks.iter().enumerate().rev();
    let mut touched_used_block_indexes = HashSet::<usize>::new();

    for (idx, entry) in disk_map.blocks.iter().enumerate() {
        if !entry.is_free && !touched_used_block_indexes.contains(&idx) {
            new_blocks.push(FileBlock {
                file_id: entry.file_id,
                file_size: entry.file_size,
                is_free: false,
            });

            continue;
        } else if touched_used_block_indexes.contains(&idx) {
            new_blocks.push(FileBlock { file_id: -1, file_size: 1, is_free: true });
            continue;
        }

        let (used_block_idx, next_used_block) = current_blocks_rev_iter
            .find(|(_, e)| !e.is_free)
            .unwrap_or((usize::MAX, &FileBlock { file_id: -1, file_size: 1, is_free: true }));

        if used_block_idx <= idx {
            new_blocks.push(FileBlock { file_id: entry.file_id, file_size: entry.file_size, is_free: entry.is_free });
            continue;
        }

        new_blocks.push(FileBlock {
            file_id: next_used_block.file_id,
            file_size: next_used_block.file_size,
            is_free: false,
        });
        if used_block_idx != usize::MAX {
            touched_used_block_indexes.insert(used_block_idx);
        }
    }

    DiskMap { blocks: new_blocks }
}

fn compact_disk_map_without_fragmentation(disk_map: &DiskMap) -> DiskMap {
    let mut new_blocks = disk_map.blocks.clone();

    let mut rev_idx = new_blocks.len() - 1;
    'outer: while rev_idx > 0 {
        let rev_block = new_blocks[rev_idx];
        if rev_block.is_free {
            rev_idx -= 1;
            continue 'outer;
        }

        let mut idx = 0;
        'inner: while idx < rev_idx {
            let block = new_blocks[idx];
            if !block.is_free {
                idx += 1;
                continue 'inner;
            }

            if block.file_size >= rev_block.file_size {
                for k in 0..rev_block.file_size {
                    new_blocks[idx + k] = FileBlock {
                        file_id: rev_block.file_id,
                        file_size: rev_block.file_size,
                        is_free: false,
                    };
                    new_blocks[rev_idx - k] = FileBlock { file_id: -1, file_size: rev_block.file_size, is_free: true };
                }

                rev_idx -= rev_block.file_size;
                idx += rev_block.file_size;

                // Update file size of free blocks that have not been filled
                loop {
                    let free_block = new_blocks[idx];
                    if !free_block.is_free {
                        break;
                    }

                    new_blocks[idx].file_size = free_block.file_size - rev_block.file_size;
                    idx += 1;
                }
                continue 'outer;
            }

            idx += 1;
        }

        rev_idx -= 1;
    }

    DiskMap { blocks: new_blocks }
}

fn calculate_checksum(disk_map: &DiskMap) -> i64 {
    let mut checksum = 0;

    for (idx, entry) in disk_map.blocks.iter().enumerate() {
        if !entry.is_free {
            checksum += (idx as i64) * entry.file_id;
        }
    }

    checksum
}

fn problem1(input: &DiskMap) {
    let compacted_map = compact_disk_map_with_fragmentation(input);
    let checksum = calculate_checksum(&compacted_map);

    println!("Problem 1 -> {}", checksum);
}

fn problem2(input: &DiskMap) {
    let compacted_map = compact_disk_map_without_fragmentation(input);
    let checksum = calculate_checksum(&compacted_map);

    println!("Problem 2 -> {}", checksum);
}

fn main() {
    let input = parse_input::<DiskMap>("input", parse_input_entries);

    problem1(&input);
    problem2(&input);
}
