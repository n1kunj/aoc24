use crate::day_output::DayOutput;

#[derive(Debug, Copy, Clone)]
struct File {
    id: u32,
    len: u32,
}

#[derive(Debug, Copy, Clone)]
struct Space {
    len: u32,
}

#[derive(Debug, Copy, Clone)]
enum Entry {
    File(File),
    Space(Space),
}

#[derive(Debug, Copy, Clone)]
enum Block {
    File(u32),
    Space,
}

fn entries_to_bm(entries: &[Entry]) -> Vec<Block> {
    let mut bm = Vec::<Block>::new();
    for entry in entries.iter() {
        match entry {
            Entry::File(file) => {
                for _ in 0..file.len {
                    bm.push(Block::File(file.id));
                }
            }
            Entry::Space(space) => {
                for _ in 0..space.len {
                    bm.push(Block::Space);
                }
            }
        }
    }
    bm
}

fn calc_checksum(blockmap: &[Block]) -> usize {
    let mut checksum = 0usize;
    for (i, block) in blockmap.iter().enumerate() {
        match block {
            Block::File(id) => checksum += i * *id as usize,
            Block::Space => (),
        }
    }
    checksum
}

pub fn main(input: &str, output: &mut DayOutput) {
    let mut entries = Vec::<Entry>::new();
    let mut is_file = true;
    let mut file_idx = 0u32;
    for char in input.chars() {
        match is_file {
            true => {
                let entry = Entry::File(File {
                    id: file_idx,
                    len: char.to_digit(10).unwrap(),
                });
                file_idx += 1;
                entries.push(entry);
            }
            false => entries.push(Entry::Space(Space {
                len: char.to_digit(10).unwrap(),
            })),
        };

        is_file = !is_file;
    }
    let entries = entries;

    let mut part1_bm = entries_to_bm(&entries);

    let mut cursor = 0usize;
    for i in (0..part1_bm.len()).rev() {
        if cursor >= i {
            break;
        }
        match part1_bm[i] {
            Block::File(id) => loop {
                match part1_bm[cursor] {
                    Block::File(_) => {
                        cursor += 1;
                        if cursor >= i {
                            break;
                        }
                    }
                    Block::Space => {
                        part1_bm[cursor] = Block::File(id);
                        part1_bm[i] = Block::Space;
                        cursor += 1;
                        break;
                    }
                }
            },
            Block::Space => {
                // Do nothing.
            }
        }
    }

    let part1 = calc_checksum(&part1_bm);

    output.part1(part1 as i64);

    let mut part2_ents = entries.clone();
    let mut i = part2_ents.len();
    loop {
        i -= 1;
        let mut cursor = 0;
        if cursor >= i {
            break;
        }
        match part2_ents[i] {
            Entry::File(file) => loop {
                match part2_ents[cursor] {
                    Entry::File(_) => {
                        cursor += 1;
                        if cursor >= i {
                            break;
                        }
                    }
                    Entry::Space(space) => {
                        if space.len >= file.len {
                            let spare_space = space.len - file.len;
                            part2_ents[i] = Entry::Space(Space { len: file.len });
                            part2_ents[cursor] = Entry::File(file);
                            cursor += 1;
                            if spare_space > 0 {
                                part2_ents.insert(cursor, Entry::Space(Space { len: spare_space }));
                                i += 1;
                            }
                            break;
                        } else {
                            cursor += 1;
                            if cursor >= i {
                                break;
                            }
                        }
                    }
                }
            },
            Entry::Space(_space) => {
                // Do nothing.
            }
        }
    }

    let part2 = calc_checksum(&entries_to_bm(&part2_ents));
    output.part2(part2 as i64);
}
