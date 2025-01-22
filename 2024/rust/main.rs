use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {}

fn distance(element_a: u32, element_b: u32) -> u64 {
    if element_a > element_b {
        return (element_a - element_b) as u64;
    }
    (element_b - element_a) as u64
}
fn distance_of_all_elements(list: &Vec<Vec<u32>>) -> u64 {
    let mut list_a: Vec<u32> = list.get(0).unwrap().to_vec();
    let mut list_b: Vec<u32> = list.get(1).unwrap().to_vec();
    list_a.sort();
    list_b.sort();
    let mut sum: u64 = 0;
    for (index, element_a) in list_a.iter().enumerate() {
        let element_b = list_b[index];
        sum += distance(*element_a, element_b)
    }
    return sum;
}
fn similarity_score_of_all_elements(list: &Vec<Vec<u32>>) -> u64 {
    let mut list_a: Vec<u32> = list.get(0).unwrap().to_vec();
    let mut list_b: Vec<u32> = list.get(1).unwrap().to_vec();
    list_a.sort();
    list_b.sort();
    let mut sim_score = 0;

    for number in list_a {
        let times_present = list_b
            .iter()
            .filter(|element| **element == number)
            .collect::<Vec<&u32>>()
            .len();
        sim_score += (times_present as u32) * number;
    }
    return sim_score.into();
}

fn check_reactor_levels(rows: &Vec<Vec<u32>>) -> Vec<bool> {
    let mut ret: Vec<bool> = Vec::new();
    'outer: for row in rows.to_vec() {
        let data = row.to_vec();
        let mut only_rising = true;
        let mut only_falling = true;
        let length = row.len() - 1;
        for current in 0..length {
            let next = current + 1;
            let distance = distance(*data.get(current).unwrap(), *data.get(next).unwrap());
            if distance < 1 || distance > 3 {
                ret.push(false);
                continue 'outer;
            }

            if only_rising && row[current] > row[current + 1 as usize] {
                only_rising = false;
            }
            if only_falling && row[current] < row[current + 1 as usize] {
                only_falling = false;
            }
            if !only_rising && !only_falling {
                ret.push(false);
                continue 'outer;
            }
        }
        ret.push(true);
    }
    return ret;
}
fn count_safe_reactor_reports(reports: &Vec<bool>) -> u32 {
    return reports
        .to_vec()
        .iter()
        .filter(|element| **element)
        .collect::<Vec<_>>()
        .len() as u32;
}
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn read_ordered_list_data_to_vec(filename: &str) -> Vec<Vec<u32>> {
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
fn read_column_data_to_vec(filename: &str) -> Vec<Vec<u32>> {
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_distance() {
        let list = [[3, 4, 2, 1, 3, 3].to_vec(), [4, 3, 5, 3, 9, 3].to_vec()].to_vec();
        assert!(distance_of_all_elements(&list) == 11);
        let list = read_column_data_to_vec("../data/input_1.tsv");
        assert!(distance_of_all_elements(&list) == 2176849);
    }

    #[test]
    fn test_similarity() {
        let list = read_column_data_to_vec("../data/input_1.tsv");
        assert!(similarity_score_of_all_elements(&list) == 23384288);
    }

    #[test]
    fn reading_list() {
        let list = read_ordered_list_data_to_vec("../data/input_2.tsv").to_vec();
        assert!(list.len() == 1000);
        let list = read_ordered_list_data_to_vec("../data/input_2a.tsv").to_vec();
        assert!(list.len() == 6);
        let list = read_ordered_list_data_to_vec("../data/input_2b.tsv").to_vec();
        assert!(list.len() == 10);
    }
    #[test]
    fn reactor_level_returns() {
        let list = read_ordered_list_data_to_vec("../data/input_2a.tsv").to_vec();
        let reactor_levels = check_reactor_levels(&list).to_vec();
        assert!(reactor_levels.get(0).unwrap());
        assert!(!reactor_levels.get(1).unwrap());
        assert!(!reactor_levels.get(2).unwrap());
        assert!(!reactor_levels.get(3).unwrap());
        assert!(!reactor_levels.get(4).unwrap());
        assert!(reactor_levels.get(5).unwrap());
        assert!(count_safe_reactor_reports(&reactor_levels) == 2);
    }

    //let reactor_levels = check_reactor_levels(&list).to_vec();
    //assert!(count_safe_reactor_reports(&reactor_levels) == 680)
}
