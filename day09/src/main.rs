use itertools::Itertools;
use std::collections::BTreeMap;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct File {
    file_id: u64,
    file_size: u64,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DiskSpace {
    FreeSpace(u64),
    File(File),
}

pub fn parse_input(input: &str) -> Vec<DiskSpace> {
    let mut disk = Vec::new();
    let mut file = true;
    let mut file_id = 0;
    for c in input.chars() {
        if let Some(num) = c.to_digit(10) {
            if file {
                disk.extend(std::iter::repeat(DiskSpace::File(File { file_id, file_size: num as u64 })).take(num as usize));
                file_id += 1;
            } else {
                disk.extend(std::iter::repeat(DiskSpace::FreeSpace(num as u64)).take(num as usize));
            }
            file = !file;
        }
    }

    println!("{}", disk.len());

    disk
}

pub fn part_1(input: &[DiskSpace]) -> u64 {
    let mut disk = input.to_owned();

    let free_space_indices = disk.iter().enumerate().filter_map(|(i, e)| matches!(e, DiskSpace::FreeSpace(_)).then_some(i)).collect::<Vec<_>>();
    let file_indices_backwards = disk.iter().enumerate().filter_map(|(i, e)| matches!(e, DiskSpace::File(_)).then_some(i)).rev().collect::<Vec<_>>();

    let number_to_swap = free_space_indices.iter().take_while(|&&i| i < file_indices_backwards.len()).count();

    for (a, b) in free_space_indices.iter().zip(file_indices_backwards.iter()).take(number_to_swap) {
        disk.swap(*a, *b);
    }

    // println!("{:?}", disk);

    disk.iter().enumerate()
        .filter_map(|(i , f)| match f {
            DiskSpace::FreeSpace(_) => None,
            DiskSpace::File(file) => Some(file.file_id * i as u64),
        }).sum()
}

pub fn part_2(input: &[DiskSpace]) -> u64 {
    let mut disk = input.to_owned();

    let chunks = disk.iter().chunk_by(|e| *e);

    let mut index = 0;
    let mut disk_entries: BTreeMap<usize, (DiskSpace, usize)> = BTreeMap::new();
    for (key, chunk) in &chunks {
        let length = chunk.count();
        disk_entries.insert(index, (*key, length));
        index += length;
    }

    // println!("{:?}", disk_entries);
    // println!();

    let files_to_try_moving = disk_entries.iter().rev()
    .filter_map(|(i, (f, l))| match f {
        DiskSpace::FreeSpace(_) => None,
        DiskSpace::File(file) => Some((*i, (*file, *l))),
    }).collect::<Vec<_>>();

    for (file_location, (file, file_size)) in files_to_try_moving {
        let new_location = disk_entries.iter()
            .filter_map(|(i, (f, l))| (matches!(f, DiskSpace::FreeSpace(_)) && *l >= file_size && *i < file_location).then_some((*i, *l))).next();
        if let Some((index, free_size)) = new_location {
            // move the file
            // println!("Moving file {} from {} to {}", file.file_id, file_location, index);

            disk_entries.insert(file_location, (DiskSpace::FreeSpace(file_size as u64), file_size));
            disk_entries.insert(index, (DiskSpace::File(file), file_size));
            if free_size > file_size {
                disk_entries.insert(index + file_size, (DiskSpace::FreeSpace((free_size - file_size) as u64), free_size - file_size));
            }
        }
        // println!("{:?}", disk_entries);
        // println!();
    }

    let disk_vector = disk_entries.into_iter()
        .flat_map(|(_, (f, l))| std::iter::repeat(f).take(l)).collect::<Vec<_>>();

    // println!("{:?}", disk_vector);
        disk_vector.iter().enumerate()
        .filter_map(|(i , f)| match f {
            DiskSpace::FreeSpace(_) => None,
            DiskSpace::File(file) => Some(file.file_id * i as u64),
        }).sum()
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