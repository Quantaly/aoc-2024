use anyhow::Result;
use aoc_2024::input_buf_read;
use std::io::Read;

// fn print_disk(disk: &[Option<usize>]) {
//     let line: String = disk
//         .iter()
//         .map(|block| {
//             block
//                 .map(|file_id| format!("{file_id}"))
//                 .unwrap_or_else(|| String::from("."))
//         })
//         .collect();
//     eprintln!("{line}");
// }

fn filesystem_checksum(disk: &[Option<u64>]) -> u64 {
    disk.iter()
        .enumerate()
        .filter_map(|(i, block)| block.map(|file_id| (i as u64) * file_id))
        .sum()
}

#[derive(Debug)]
struct Chunk {
    index: usize,
    len: usize,
}

fn main() -> Result<()> {
    let mut disk = Vec::new();
    let mut file_block_indices = Vec::new();
    let mut free_block_indices = Vec::new();
    let mut file_chunks = Vec::new();
    let mut free_chunks = Vec::new();

    let mut next_file_id = 0;
    for (b, is_file) in input_buf_read()?
        .bytes()
        .zip([true, false].into_iter().cycle())
    {
        let b = b?;
        if (b'0'..=b'9').contains(&b) {
            let block_count = b - b'0';
            if is_file {
                let file_id = next_file_id;
                next_file_id += 1;
                file_chunks.push(Chunk {
                    index: disk.len(),
                    len: block_count.into(),
                });
                for _ in 0..block_count {
                    file_block_indices.push(disk.len());
                    disk.push(Some(file_id));
                }
            } else {
                free_chunks.push(Chunk {
                    index: disk.len(),
                    len: block_count.into(),
                });
                for _ in 0..block_count {
                    free_block_indices.push(disk.len());
                    disk.push(None);
                }
            }
        }
    }

    let mut disk_1 = disk.clone();
    for (file_index, free_index) in file_block_indices.into_iter().rev().zip(free_block_indices) {
        if free_index > file_index {
            break;
        }
        let file_id = disk_1[file_index].take().unwrap();
        _ = disk_1[free_index].insert(file_id);
    }

    println!("{}", filesystem_checksum(&disk_1));

    let mut disk_2 = disk;
    for file_chunk in file_chunks.into_iter().rev() {
        for free_chunk in free_chunks.iter_mut() {
            if free_chunk.index < file_chunk.index && free_chunk.len >= file_chunk.len {
                let (a, b) = disk_2.split_at_mut(file_chunk.index);
                a[free_chunk.index..(free_chunk.index + file_chunk.len)]
                    .swap_with_slice(&mut b[..file_chunk.len]);
                free_chunk.len -= file_chunk.len;
                free_chunk.index += file_chunk.len;
                break;
            }
        }
    }

    println!("{}", filesystem_checksum(&disk_2));

    Ok(())
}
