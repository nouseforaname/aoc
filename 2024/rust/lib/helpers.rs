use std::fs::{read_to_string, File};
use std::io::{self, BufRead};
use std::path::Path;
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
