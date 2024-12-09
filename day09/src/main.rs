use itertools::Itertools;
use std::collections::BTreeMap;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct File {
    file_id: u64,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DiskEntry {
    FreeSpace,
    File(File),
}

pub fn parse_input(input: &str) -> Vec<DiskEntry> {
    let mut disk = Vec::new();
    let mut file = true;
    let mut file_id = 0;
    for c in input.chars() {
        if let Some(num) = c.to_digit(10) {
            if file {
                disk.extend(
                    std::iter::repeat(DiskEntry::File(File { file_id })).take(num as usize),
                );
                file_id += 1;
            } else {
                disk.extend(std::iter::repeat(DiskEntry::FreeSpace).take(num as usize));
            }
            file = !file;
        }
    }

    println!("{}", disk.len());

    disk
}

pub fn part_1(input: &[DiskEntry]) -> u64 {
    let mut disk = input.to_owned();

    let free_space_indices = disk
        .iter()
        .enumerate()
        .filter_map(|(i, e)| matches!(e, DiskEntry::FreeSpace).then_some(i))
        .collect::<Vec<_>>();
    let file_indices_backwards = disk
        .iter()
        .enumerate()
        .filter_map(|(i, e)| matches!(e, DiskEntry::File(_)).then_some(i))
        .rev()
        .collect::<Vec<_>>();

    let number_to_swap = free_space_indices
        .iter()
        .take_while(|&&i| i < file_indices_backwards.len())
        .count();

    for (a, b) in free_space_indices
        .iter()
        .zip(file_indices_backwards.iter())
        .take(number_to_swap)
    {
        disk.swap(*a, *b);
    }

    disk.iter()
        .enumerate()
        .filter_map(|(i, f)| match f {
            DiskEntry::FreeSpace => None,
            DiskEntry::File(file) => Some(file.file_id * i as u64),
        })
        .sum()
}

pub fn part_2(disk: &[DiskEntry]) -> u64 {
    let chunks = disk.iter().chunk_by(|e| *e);

    let mut index = 0;
    // build a map of start index -> (entry, size)
    let mut disk_entries: BTreeMap<usize, (DiskEntry, usize)> = BTreeMap::new();
    for (key, chunk) in &chunks {
        let length = chunk.count();
        disk_entries.insert(index, (*key, length));
        index += length;
    }

    let files_to_try_moving = disk_entries
        .iter()
        .rev()
        .filter_map(|(i, (f, l))| match f {
            DiskEntry::FreeSpace => None,
            DiskEntry::File(file) => Some((*i, (*file, *l))),
        })
        .collect::<Vec<_>>();

    let mut gaps = disk_entries
        .iter()
        .filter_map(|(gap_location, (gap, gap_size))| {
            matches!(gap, DiskEntry::FreeSpace).then_some((*gap_location, *gap_size))
        })
        .collect::<BTreeMap<_, _>>();

    for (file_location, (file, file_size)) in files_to_try_moving {
        let new_location = gaps.iter().find(|&(gap_location, gap_size)| {
            gap_size >= &file_size && gap_location < &file_location
        });

        if let Some((&gap_location, &gap_size)) = new_location {
            // move the file
            disk_entries.insert(file_location, (DiskEntry::FreeSpace, file_size));
            disk_entries.insert(gap_location, (DiskEntry::File(file), file_size));
            gaps.remove(&gap_location);
            if gap_size > file_size {
                disk_entries.insert(
                    gap_location + file_size,
                    (DiskEntry::FreeSpace, gap_size - file_size),
                );
                gaps.insert(gap_location + file_size, gap_size - file_size);
            }
        }
    }

    disk_entries
        .values()
        .flat_map(|(f, l)| std::iter::repeat(f).take(*l))
        .enumerate()
        .filter_map(|(i, f)| match f {
            DiskEntry::FreeSpace => None,
            DiskEntry::File(file) => Some(file.file_id * i as u64),
        })
        .sum()
}

fn main() {
    let file = include_str!("../input.txt");
    let input = parse_input(file);
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

#[test]
pub fn test() {
    let input = "2333133121414131402";
    let disk = parse_input(input);
    assert_eq!(part_1(&disk), 1928);
    assert_eq!(part_2(&disk), 2858);
}
