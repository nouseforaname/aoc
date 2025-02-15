#[test]
fn test_calc_checksum() {
    assert_eq!(
        calc_checksum(
            &"0,0,9,9,8,1,1,1,8,8,8,2,7,7,7,3,3,3,6,4,4,6,5,5,5,5,6,6"
                .split(',')
                .map(|e| match e.parse() {
                    Ok(v) => Some(v),
                    _ => None,
                })
                .collect::<Vec<Option<u32>>>()
        ),
        1928
    );

    let now = std::time::Instant::now();
    let input = std::fs::read_to_string("../data/input_9a.txt")
        .unwrap()
        .trim()
        .to_string();
    let disk_map = create_disk_map(input);
    let compact_disk_map = compact_disk_map(&disk_map);
    assert_eq!(calc_checksum(&compact_disk_map), 6331212425418);
    println!("{:?}", now.elapsed());
}
pub fn calc_checksum(input: &Vec<Option<u32>>) -> u128 {
    input
        .into_iter()
        .enumerate()
        .map_while(|(index, c)| match c {
            Some(v) => Some(index as u128 * *v as u128),
            None => Some(0 as u128),
        })
        .sum()
}

#[test]
fn test_compact_disk_map() {
    assert_eq!(
        compact_disk_map(
            &"0,.,.,1,1,1,.,.,.,.,2,2,2,2,2"
                .split(',')
                .map(|e| match e.parse() {
                    Ok(v) => Some(v),
                    _ => None,
                })
                .collect::<Vec<Option<u32>>>()
        ),
        "0,2,2,1,1,1,2,2,2"
            .split(',')
            .map(|e| match e.parse() {
                Ok(v) => Some(v),
                _ => None,
            })
            .collect::<Vec<Option<u32>>>()
    );
    assert_eq!(
        compact_disk_map(
            &"0,0,.,.,.,1,1,1,.,.,.,2,.,.,.,3,3,3,.,4,4,.,5,5,5,5,.,6,6,6,6,.,7,7,7,.,8,8,8,8,9,9"
                .split(',')
                .map(|e| match e.parse() {
                    Ok(v) => Some(v),
                    _ => None,
                })
                .collect::<Vec<Option<u32>>>()
        ),
        "0,0,9,9,8,1,1,1,8,8,8,2,7,7,7,3,3,3,6,4,4,6,5,5,5,5,6,6"
            .split(',')
            .map(|e| match e.parse() {
                Ok(v) => Some(v),
                _ => None,
            })
            .collect::<Vec<Option<u32>>>()
    );
}
pub fn compact_disk_map(input: &Vec<Option<u32>>) -> Vec<Option<u32>> {
    let mut output = input.clone();
    let mut free_index = 0;
    let t = input
        .into_iter()
        .enumerate()
        .filter_map(|(i, c)| match c {
            Some(c) => Some((i, *c)),
            _ => None,
        })
        .rev()
        .collect::<Vec<(usize, u32)>>();

    for (index, _) in t.iter() {
        let x = output[free_index..].iter().position(|&e| e == None);
        match x {
            Some(val) => {
                free_index += val;
                if free_index < *index {
                    output.swap(*index, free_index);
                    continue;
                }
                break;
            }
            None => {
                break;
            }
        }
    }
    output.into_iter().filter(|e| e.is_some()).collect()
}
#[test]
fn test_block_compact_disk_map() {
    assert_eq!(
        block_compact_disk_map(
            "0,.,.,1,1,1,.,2,2,.,3,3,3,3,3"
                .split(',')
                .map(|e| match e.parse() {
                    Ok(v) => Some(v),
                    _ => None,
                })
                .collect::<Vec<Option<u32>>>()
        ),
        "0,2,2,1,1,1,.,.,.,.,3,3,3,3,3"
            .split(',')
            .map(|e| match e.parse() {
                Ok(v) => Some(v),
                _ => None,
            })
            .collect::<Vec<Option<u32>>>()
    );

    // 0,0,.,.,.,1,1,1,.,.,.,2,.,.,.,3,3,3,.,4,4,.,5,5,5,5,.,6,6,6,6,.,7,7,7,.,8,8,8,8,9,9
    // 0,0,9,9,2,1,1,1,7,7,7,.,4,4,.,3,3,3,.,.,.,.,5,5,5,5,.,6,6,6,6,.,.,.,.,.,8,8,8,8,.,.
    assert_eq!(
        block_compact_disk_map(
            "0,0,.,.,.,1,1,1,.,.,.,2,.,.,.,3,3,3,.,4,4,.,5,5,5,5,.,6,6,6,6,.,7,7,7,.,8,8,8,8,9,9"
                .split(',')
                .map(|e| match e.parse() {
                    Ok(v) => Some(v),
                    _ => None,
                })
                .collect::<Vec<Option<u32>>>()
        ),
        "0,0,9,9,2,1,1,1,7,7,7,.,4,4,.,3,3,3,.,.,.,.,5,5,5,5,.,6,6,6,6,.,.,.,.,.,8,8,8,8,.,."
            .split(',')
            .map(|e| match e.parse() {
                Ok(v) => Some(v),
                _ => None,
            })
            .collect::<Vec<Option<u32>>>()
    );

    assert_eq!(
        calc_checksum(
            &"0,0,9,9,2,1,1,1,7,7,7,.,4,4,.,3,3,3,.,.,.,.,5,5,5,5,.,6,6,6,6,.,.,.,.,.,8,8,8,8,.,."
                .split(',')
                .map(|e| match e.parse::<u32>() {
                    Ok(v) => Some(v),
                    _ => None,
                })
                .collect::<Vec<Option<u32>>>()
        ),
        2858
    );
}
pub fn block_compact_disk_map(input: Vec<Option<u32>>) -> Vec<Option<u32>> {
    let mut input = input.clone();

    let mut start = 0;
    let mut free_block_ranges: Vec<(usize, usize)> = Vec::new();
    let mut index = input.len() - 1;
    let mut last_element = input.last().unwrap();
    let mut end = index;
    let moveable_block: Vec<(usize, usize, u32)> = input[..input.len() - 1]
        .iter()
        .rev()
        .map(|e| {
            index -= 1;
            match (e, last_element) {
                (e, last) if e == last => None,
                (e, _) if e != last_element => {
                    start = index + 1;
                    let (add_start, add_end) = (start, end);

                    let add = last_element;
                    last_element = e;
                    end = index;
                    match add {
                        Some(v) => Some((add_start, add_end, *v)),
                        None => {
                            free_block_ranges.push((add_start, add_end));
                            None
                        }
                    }
                }
                (_, _) => None,
            }
        })
        .filter_map(|e| e)
        .collect();

    free_block_ranges = free_block_ranges.into_iter().rev().collect();
    for block in moveable_block {
        let block_length = (block.0..=block.1).count();
        match free_block_ranges
            .iter()
            .position(|free| (free.0..=free.1).count() >= block_length)
        {
            Some(range_index) => {
                let mut rel_index = 0;
                let range = free_block_ranges.get_mut(range_index).unwrap();
                for i in block.0..=block.1 {
                    rel_index = i - block.0;
                    let target = range.0 + rel_index;
                    if target < i {
                        input.swap(target, i);
                    } else {
                        // arrived at defragged area
                        break;
                    }
                }

                if range.1 == range.0 + rel_index {
                    free_block_ranges.remove(range_index);
                } else {
                    range.0 += rel_index + 1;
                }
            }
            None => {}
        };
    }
    input
}
#[test]
fn test_create_disk_map() {
    assert_eq!(
        create_disk_map("12345".to_string()),
        "0,.,.,1,1,1,.,.,.,.,2,2,2,2,2"
            .split(',')
            .map(|e| match e.parse() {
                Ok(v) => Some(v),
                _ => None,
            })
            .collect::<Vec<Option<u32>>>()
    );
    let now = std::time::Instant::now();
    assert_eq!(
        create_disk_map("2333133121414131402".to_string()),
        "0,0,.,.,.,1,1,1,.,.,.,2,.,.,.,3,3,3,.,4,4,.,5,5,5,5,.,6,6,6,6,.,7,7,7,.,8,8,8,8,9,9"
            .split(',')
            .map(|e| match e.parse() {
                Ok(v) => Some(v),
                _ => None,
            })
            .collect::<Vec<Option<u32>>>()
    );
    println!("took {}", now.elapsed().as_millis());
}
pub fn create_disk_map(input: String) -> Vec<Option<u32>> {
    let mut file = false;
    let mut file_index: u32 = 0;
    input
        .chars()
        .map(|c| {
            let l: u32 = u32::from(c) - 48; // converting u32 from char will convert to ascii table num
            file = !file;
            let mut t = None;
            if file {
                t = Some(file_index);
                file_index += 1;
            }
            (0..l as usize).map(|_| t).collect::<Vec<Option<u32>>>()
        })
        .flatten()
        .collect()
}
