use std::collections::HashMap;
use std::fs::{read_to_string, File};
use std::io::{self, BufRead};
use std::path::Path;

#[test]
fn test_write_char_to_coord() {
    let mut test = [
        ['a', 'x'].to_vec(),
        ['b', 'x'].to_vec(),
        ['c', 'x'].to_vec(),
    ]
    .to_vec();
    let test2 = [
        ['d', 'x'].to_vec(),
        ['e', 'x'].to_vec(),
        ['f', 'x'].to_vec(),
    ]
    .to_vec();
    assert!(write_char_to_coord(&mut test, (0, 0), 'd'));
    assert!(write_char_to_coord(&mut test, (0, 1), 'e'));
    assert!(write_char_to_coord(&mut test, (0, 2), 'f'));

    // out of bounds write shouldn't be possible
    assert!(!write_char_to_coord(&mut test, (2, 2), 'f'));

    assert_eq!(test, test2);
}
pub fn write_char_to_coord(input: &mut Vec<Vec<char>>, coord: (u32, u32), c: char) -> bool {
    let (x, y) = coord;

    match input.get_mut(y as usize) {
        Some(line) => match line.get_mut(x as usize) {
            Some(elem) => {
                *elem = c;
            }
            None => return false,
        },
        None => return false,
    }

    return true;
}

#[test]
fn test_is_series_safe() {
    // clean down series with proper distance
    let test = [7, 6, 4, 2, 1].to_vec();
    assert!(is_series_safe(&test, false));

    // clean up series with proper distance
    let test = [1, 2, 4, 6, 9].to_vec();
    assert!(is_series_safe(&test, false));

    //first element bad in up series
    let test = [5, 4, 7, 8, 9].to_vec();
    assert_eq!(is_series_safe(&test, false), false);
    //works with dampener
    assert_eq!(is_series_safe(&test, true), true);

    ////first element bad in down series
    let test = [3, 4, 3, 2, 1].to_vec();
    assert_eq!(is_series_safe(&test, false), false);
    //works with dampener
    assert_eq!(is_series_safe(&test, true), true);

    //last element bad with too high distance up series
    let test = [1, 3, 6, 7, 19].to_vec();
    assert_eq!(is_series_safe(&test, false), false);
    //works with dampener
    assert_eq!(is_series_safe(&test, true), true);

    //last element bad with too high distance down series
    let test = [33, 31, 29, 27, 19].to_vec();
    //works with dampener
    assert_eq!(is_series_safe(&test, false), false);

    //middle element bad with direction
    let test = [1, 3, 2, 4, 5].to_vec();
    assert_eq!(is_series_safe(&test, false), false);
    //works with dampener
    assert_eq!(is_series_safe(&test, true), true);

    // two bad distances,
    let test = [1, 1, 2, 3, 4, 4].to_vec();
    assert_eq!(is_series_safe(&test, true), false);

    // two direction changes
    let test = [1, 3, 2, 5, 4, 6].to_vec();
    assert_eq!(is_series_safe(&test, true), false);

    // bad distance and direction change
    let test = [1, 4, 2, 5, 9].to_vec();
    assert_eq!(is_series_safe(&test, true), false);

    let list = read_ordered_list_data_to_vec("../data/input_2a.tsv").to_vec();
    assert_eq!(is_series_safe(&list[0], false), true);
    assert_eq!(is_series_safe(&list[1], false), false);
    assert_eq!(is_series_safe(&list[2], false), false);
    assert_eq!(is_series_safe(&list[3], false), false);
    assert_eq!(is_series_safe(&list[4], false), false);
    assert_eq!(is_series_safe(&list[5], false), true);

    assert_eq!(is_series_safe(&list[0], true), true);
    assert_eq!(is_series_safe(&list[1], true), false);
    assert_eq!(is_series_safe(&list[2], true), false);
    assert_eq!(is_series_safe(&list[3], true), true);
    assert_eq!(is_series_safe(&list[4], true), true);
    assert_eq!(is_series_safe(&list[5], true), true);
}

pub fn is_series_safe(row: &Vec<u32>, dampener: bool) -> bool {
    let mut possible_offenders = Vec::new();
    let mut sorted = row.is_sorted_by(|a, b| {
        let sorted = a < b && 4 > b - a && b - a > 0;
        if !sorted {
            let offender_index = row.iter().position(|&el| el == *a).unwrap();
            possible_offenders.extend([offender_index, offender_index + 1]);
        }
        sorted
    });
    if sorted {
        return true;
    }
    sorted = row.is_sorted_by(|a, b| {
        let sorted = a > b && 4 > a - b && a - b > 0;
        if !sorted {
            let offender_index = row.iter().position(|&el| el == *a).unwrap();
            possible_offenders.extend([offender_index, offender_index + 1]);
        }
        sorted
    });

    if sorted {
        return true;
    }

    if dampener {
        possible_offenders.sort_unstable();
        possible_offenders.dedup();
        return possible_offenders
            .iter()
            .find(|&e| {
                let mut row = row.clone();
                row.remove(*e);
                is_series_safe(&row, false)
            })
            .is_some();
    }

    return false;
}

pub const UP: (i32, i32) = (0, -1);
pub const DOWN: (i32, i32) = (0, 1);
pub const LEFT: (i32, i32) = (-1, 0);
pub const RIGHT: (i32, i32) = (1, 0);

pub fn extract_vec_from_2d_vec(
    input: &Vec<Vec<char>>,
    start: (u32, u32),
    direction: (i32, i32),
    min_length: i32,
    max_length: i32,
) -> Vec<char> {
    let (mut x, mut y) = start;
    let (mut x, mut y) = (x as i64, y as i64);

    let mut ret: Vec<char> = Vec::new();
    let mut extracted_char_count = 0;
    let num_rows = input.len();
    let (width, height) = (input.first().unwrap().len(), input.len());
    if x as usize >= width || y as usize >= height {
        return [].to_vec();
    }
    match direction {
        // line
        RIGHT => {
            let line = input.get(y as usize).unwrap().clone().to_vec();
            return line[x as usize..line.len()].to_vec();
        }
        LEFT => {
            let line = input.get(y as usize).unwrap().clone().to_vec();
            let mut ret = line[0..=x as usize].to_vec().clone();
            ret.reverse();
            return ret;
        }
        // diagonal // column
        (delta_x, delta_y) => loop {
            match input.get(y as usize) {
                Some(row) => {
                    let row_length = row.len();
                    match row.get(x as usize) {
                        Some(c) => {
                            ret.push(*c);
                            extracted_char_count += 1;
                            y += delta_y as i64;
                            x += delta_x as i64;
                            if extracted_char_count == max_length {
                                break;
                            }
                            if y < 0 || x < 0 {
                                break;
                            }
                            if y as u32 >= num_rows as u32 || x as u32 >= row_length as u32 {
                                break;
                            }
                        }
                        None => {
                            break;
                        }
                    }
                }
                None => {
                    break;
                }
            }
        },
    }
    if extracted_char_count < min_length {
        return [].to_vec();
    }
    return ret;
}
#[test]
fn test_find_line_hits() {
    let haystack = "SAMXXCXMAS".chars().collect();
    let needle = "XMAS".chars().collect();

    assert_eq!(find_line_hits(&haystack, &needle), 1);

    let haystack = "XMAS".chars().collect();
    assert_eq!(find_line_hits(&haystack, &needle), 1);

    let haystack = "XMASXX".chars().collect();
    assert_eq!(find_line_hits(&haystack, &needle), 1);

    let haystack = ['M', 'S', 'X', 'M', 'A', 'X', 'S', 'A', 'M', 'X'].to_vec();
    let needle = ['S', 'A', 'M', 'X'].to_vec();

    assert_eq!(find_line_hits(&haystack, &needle), 1);
}
pub fn find_line_hits(haystack: &Vec<char>, needle: &Vec<char>) -> u16 {
    if haystack.len() < needle.len() {
        return 0;
    }
    if haystack.len() == needle.len() {
        if haystack == needle {
            return 1;
        }
        return 0;
    }
    let first_char = needle.first().unwrap();
    let needle_max_index = needle.len() - 1;
    let haystack_max_index = haystack.len() - 1;
    let mut hits = 0;

    let positions = position_of_char(haystack, first_char);
    for position in positions {
        if position + needle_max_index > haystack_max_index {
            continue;
        }
        let partial = &haystack[position..=position + needle_max_index];

        if partial == needle {
            hits += 1;
        }
    }
    return hits;
}

#[test]
fn test_read_book_printing_data() {
    let (rules, updates) = read_book_printing_data("../data/input5a.txt");

    assert_eq!(rules.len(), 6);
    assert_eq!(updates.len(), 6);

    assert_eq!(rules.get(&75).unwrap(), &[29, 53, 47, 61, 13].to_vec());
    assert_eq!(updates.last().unwrap(), &[97, 13, 75, 29, 47].to_vec())
}
pub fn read_book_printing_data(path: &str) -> (HashMap<i32, Vec<u16>>, Vec<Vec<u16>>) {
    let mut rules = HashMap::<i32, Vec<u16>>::new();
    let mut updates = Vec::new();

    if let Ok(lines) = read_lines(path) {
        for line in lines.map_while(Result::ok) {
            if line == "".to_string() {
                continue;
            }
            if !line.contains('|') {
                let elements: Vec<u16> = line
                    .split(',')
                    .map(|element| element.parse::<u16>().unwrap())
                    .collect();
                if elements.len() > 0 {
                    updates.push(elements);
                }
                continue;
            }
            let elements: Vec<i32> = line
                .split('|')
                .map(|element| element.parse::<i32>().unwrap())
                .collect();

            if elements.len() == 2 {
                let key = elements.first().unwrap();
                let value = elements.last().unwrap();

                match rules.get_mut(key) {
                    Some(val) => {
                        val.push(*value as u16);
                    }
                    None => {
                        rules.insert(*key, [*value as u16].to_vec());
                    }
                }
                continue;
            }
        }
    }

    return (rules, updates);
}

#[test]
fn test_read_data_to_vec_of_tuples() {
    let ret = read_data_to_vec_of_tuples("../data/input_7a.txt".to_string());
    assert_eq!(ret.len(), 9);
    let (sum, elements) = ret.first().unwrap();
    assert_eq!(sum, &190);
    assert_eq!(elements, &[10, 19].to_vec());
    let (sum, elements) = ret.last().unwrap();
    assert_eq!(sum, &292);
    assert_eq!(elements, &[11, 6, 16, 20].to_vec());
}

pub fn read_data_to_vec_of_tuples(path: String) -> Vec<(u64, Vec<u64>)> {
    let mut ret = Vec::new();
    if let Ok(lines) = read_to_string(path) {
        ret = lines
            .lines()
            .filter_map(|line| line.split_once(":"))
            .map(|(sum, numbers)| {
                let sum = sum.parse().unwrap();
                let numbers = numbers
                    .split_whitespace()
                    .map(|e| e.parse().unwrap())
                    .collect();
                return (sum, numbers);
            })
            .collect();
    }

    return ret;
}
pub fn position_of_char(haystack: &Vec<char>, c: &char) -> Vec<usize> {
    let mut all_elements = haystack.iter();
    let mut positions = Vec::new();
    let mut offset = 0;
    loop {
        match all_elements.position(|element| element == c) {
            Some(pos) => {
                offset += pos;
                positions.push(offset);
                offset += 1;
            }
            None => break,
        }
    }
    return positions;
}
pub fn read_column_data_to_vec(filename: &str) -> Vec<Vec<u32>> {
    let mut ret = Vec::<Vec<u32>>::new();
    let mut init: bool = true;
    if let Ok(lines) = read_lines(filename) {
        for line in lines.map_while(Result::ok) {
            let data = line.split_whitespace();
            for (index, column) in data.enumerate() {
                let column_val = column.parse().unwrap();
                if init {
                    let mut column_vec = Vec::<u32>::new();
                    column_vec.push(column_val);
                    ret.push(column_vec);
                } else {
                    ret.get_mut(index).unwrap().push(column_val);
                }
            }
            init = false;
        }
    };
    return ret;
}

#[test]
fn reading_list() {
    let list = read_ordered_list_data_to_vec("../data/input_2.tsv").to_vec();
    assert!(list.len() == 1000);
    let list = read_ordered_list_data_to_vec("../data/input_2a.tsv").to_vec();
    assert!(list.len() == 6);
}
pub fn read_ordered_list_data_to_vec(filename: &str) -> Vec<Vec<u32>> {
    let mut ret = Vec::<Vec<u32>>::new();

    if let Ok(lines) = read_lines(filename) {
        for line in lines.map_while(Result::ok) {
            let mut data = line.split_whitespace();
            let mut line_data: Vec<u32> = Vec::new();
            'inner: loop {
                match data.next() {
                    Some(val) => {
                        line_data.push(val.parse().unwrap());
                    }
                    None => break 'inner,
                }
            }
            ret.push(line_data);
        }
    };
    return ret;
}
pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
pub fn read_to_vec_vec_char(path: &str) -> Vec<Vec<char>> {
    let mut ret = Vec::new();

    if let Ok(lines) = read_lines(path) {
        for line in lines.map_while(Result::ok) {
            let chars: Vec<char> = line.chars().map(|c| c).collect();
            ret.push(chars);
        }
    }

    return ret;
}
#[test]
fn test_distance() {
    assert_eq!(distance(0, 0), 0);
    assert_eq!(distance(1, 0), 1);
    assert_eq!(distance(2, 0), 2);
    assert_eq!(distance(0, 0), 0);
    assert_eq!(distance(0, 1), 1);
    assert_eq!(distance(0, 2), 2);
}
pub fn distance(element_a: u32, element_b: u32) -> u64 {
    if element_a > element_b {
        return (element_a - element_b) as u64;
    }
    (element_b - element_a) as u64
}
