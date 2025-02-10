use std::collections::HashMap;
use std::fs::{read_to_string, File};
use std::io::{self, BufRead};
use std::path::Path;

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
