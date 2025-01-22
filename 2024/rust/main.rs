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
#[derive(Debug)]
enum Direction {
    UP,
    DOWN,
}
// 2nd return element contains either the offending element or the length of the vec if the series is safe
fn is_series_safe(row: &Vec<u32>, direction: &Direction, dampener: bool) -> (bool, usize) {
    let length = row.len() - 1;
    let distance_range = 1..4;
    let mut offender = 0;
    let mut is_safe = true;
    for current in 0..length {
        let next = current + 1;
        match direction {
            Direction::UP => {
                if row[current] > row[next] {
                    is_safe = false;
                    offender = current;
                }
            }
            Direction::DOWN => {
                if row[current] < row[next] {
                    is_safe = false;
                    offender = current;
                }
            }
        }
        if !distance_range.contains(&distance(row[current], row[next])) {
            // if we're at the 2nd to last element, and every other check passed, then the offending distance is introduced by the last element
            if current == length - 1 {
                offender = next;
            } else {
                offender = current;
            }
            is_safe = false;
        }
        if !is_safe {
            if dampener {
                let mut row_without_current = row.clone();
                row_without_current.remove(current);
                let (safe_without_current, _) =
                    is_series_safe(&row_without_current, direction, false);

                if safe_without_current {
                    return (safe_without_current, current);
                }

                let mut row_without_next = row.clone();
                row_without_next.remove(next);
                let (safe_without_next, _) = is_series_safe(&row_without_next, direction, false);
                if safe_without_next {
                    return (safe_without_next, next);
                }
            }
            break;
        }
    }
    if is_safe {
        return (is_safe, row.len());
    }
    return (is_safe, offender);
}
fn check_reactor_levels(rows: &Vec<Vec<u32>>, dampener: bool) -> Vec<bool> {
    let mut ret: Vec<bool> = Vec::new();
    for row in rows.to_vec() {
        let mut safe: bool;
        (safe, _) = is_series_safe(&row.to_vec(), &Direction::UP, dampener);
        if safe {
            ret.push(true);
            continue;
        }
        (safe, _) = is_series_safe(&row.to_vec(), &Direction::DOWN, dampener);
        if safe {
            ret.push(true);
            continue;
        }
        ret.push(false);
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
        //let list = read_ordered_list_data_to_vec("../data/input_2b.tsv").to_vec();
        //assert!(list.len() == 10);
    }
    #[test]
    fn reactor_level_returns() {
        let list = read_ordered_list_data_to_vec("../data/input_2a.tsv").to_vec();
        let reactor_levels = check_reactor_levels(&list, false).to_vec();
        assert_eq!(reactor_levels.get(0).unwrap(), &true);
        assert_eq!(reactor_levels.get(1).unwrap(), &false);
        assert_eq!(reactor_levels.get(2).unwrap(), &false);
        assert_eq!(reactor_levels.get(3).unwrap(), &false);
        assert_eq!(reactor_levels.get(4).unwrap(), &false);
        assert_eq!(reactor_levels.get(5).unwrap(), &true);
        assert_eq!(count_safe_reactor_reports(&reactor_levels), 2);

        let reactor_levels = check_reactor_levels(&list, true).to_vec();
        assert_eq!(reactor_levels.get(0).unwrap(), &true);
        assert_eq!(reactor_levels.get(1).unwrap(), &false);
        assert_eq!(reactor_levels.get(2).unwrap(), &false);
        assert_eq!(reactor_levels.get(3).unwrap(), &true);
        assert_eq!(reactor_levels.get(4).unwrap(), &true);
        assert_eq!(reactor_levels.get(5).unwrap(), &true);
        assert!(count_safe_reactor_reports(&reactor_levels) == 4);
    }

    #[test]
    fn test_is_series_safe() {
        // clean down series with proper distance
        let test = [7, 6, 4, 2, 1].to_vec();
        assert!(is_series_safe(&test, &Direction::DOWN, false) == (true, test.len()));
        assert_eq!(is_series_safe(&test, &Direction::UP, false), (false, 0));
        // clean up series with proper distance
        let test = [1, 2, 4, 6, 9].to_vec();
        assert!(is_series_safe(&test, &Direction::UP, false) == (true, test.len()));
        assert!(is_series_safe(&test, &Direction::DOWN, false) == (false, 0));

        //first element bad in up series
        let test = [5, 4, 7, 8, 9].to_vec();
        assert!(is_series_safe(&test, &Direction::UP, false) == (false, 0));
        assert!(is_series_safe(&test, &Direction::DOWN, false) == (false, 1));

        let mut test_remove = test.clone();
        test_remove.remove(0);
        assert!(is_series_safe(&test_remove, &Direction::UP, false) == (true, test_remove.len()));

        ////first element bad in down series
        let test = [4, 4, 3, 2, 1].to_vec();
        assert!(is_series_safe(&test, &Direction::UP, false) == (false, 0));
        assert!(is_series_safe(&test, &Direction::DOWN, false) == (false, 0));

        let mut test_remove = test.clone();
        test_remove.remove(0);
        assert!(is_series_safe(&test_remove, &Direction::UP, false) == (false, 0));
        assert!(is_series_safe(&test_remove, &Direction::DOWN, false) == (true, test_remove.len()));

        //last element bad with too high distance up series
        let test = [1, 3, 6, 7, 19].to_vec();
        assert_eq!(
            is_series_safe(&test, &Direction::UP, false),
            (false, test.len() - 1)
        );

        //last element bad with too high distance down series
        let test = [33, 31, 29, 27, 19].to_vec();
        assert_eq!(
            is_series_safe(&test, &Direction::DOWN, false),
            (false, test.len() - 1)
        );

        //middle element bad with direction
        let test = [1, 3, 2, 4, 5].to_vec();
        assert_eq!(is_series_safe(&test, &Direction::UP, false), (false, 1));

        let list = read_ordered_list_data_to_vec("../data/input_2.tsv").to_vec();
        let reactor_levels = check_reactor_levels(&list, false).to_vec();
        assert_eq!(count_safe_reactor_reports(&reactor_levels), 680);

        let list = read_ordered_list_data_to_vec("../data/input_2.tsv").to_vec();
        let reactor_levels = check_reactor_levels(&list, true).to_vec();
        assert_eq!(count_safe_reactor_reports(&reactor_levels), 710);
    }
}
