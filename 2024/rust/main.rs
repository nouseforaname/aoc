use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
fn main() {}

fn scan(input: &Vec<Vec<char>>, needle: String, scanner: fn(&Vec<char>, &Vec<char>) -> u16) -> u16 {
    let mut hits = 0;
    let num_rows = input.len() - 1;
    let needle_reverse: Vec<char> = needle.chars().rev().collect();
    let needle: Vec<char> = needle.chars().collect();
    let mut num_columns: i32 = 0;
    for (line_index, line) in input.iter().enumerate() {
        let line_index = line_index as i32;

        if line_index == 0 {
            num_columns = (line.len() - 1) as i32;
            for column_index in 0..=num_columns {
                // scan vertically on first pass only
                let column = extract_string(input, (column_index, 0), (0, 1), num_columns, -1);

                hits += find_line_hits(&column, &needle);
                hits += find_line_hits(&column, &needle_reverse);
            }
        }
        hits += find_line_hits(&line, &needle);
        hits += find_line_hits(&line, &needle_reverse);
    }

    let y_range = (needle.len() - 1) as i32..=num_rows as i32;
    let direction = (1, -1);
    let x: i32 = 0;
    for y in y_range {
        let start = (x, y);
        let line = extract_string(&input, start, direction, needle.len() as i32, -1);
        hits += find_line_hits(&line, &needle);
        hits += find_line_hits(&line, &needle_reverse);
    }
    let y = num_rows as i32;
    let x_range = 1..=(num_columns - (needle.len() - 1) as i32);
    for x in x_range {
        let start = (x, y);

        let line = extract_string(&input, start, direction, needle.len() as i32, -1);
        hits += find_line_hits(&line, &needle);
        hits += find_line_hits(&line, &needle_reverse);
    }
    let y = 0;
    let x_range = (0..=(num_columns - (needle.len() - 1) as i32)).rev();
    let direction = (1, 1);
    for x in x_range {
        let start = (x, y);
        let line = extract_string(&input, start, direction, needle.len() as i32, -1);

        hits += find_line_hits(&line, &needle);
        hits += find_line_hits(&line, &needle_reverse);
    }
    let y_range = 1..=(num_rows - (needle.len() - 1)) as i32;
    let x: i32 = 0;
    for y in y_range {
        let start = (x, y);
        let line = extract_string(&input, start, direction, needle.len() as i32, -1);
        hits += find_line_hits(&line, &needle);
        hits += find_line_hits(&line, &needle_reverse);
    }

    // https://github.com/rust-lang/rust/issues/70925 descending ranges dont work.
    return hits as u16;
}
fn find_cross_hits(haystack: &Vec<Vec<char>>, needle: &Vec<char>) -> u16 {
    let center_index = needle.len() / 2;
    let mut hits = 0;
    let center_char = &needle[center_index];
    let mut needle_reverse: Vec<char> = needle.clone();
    needle_reverse.reverse();

    let line_offset = center_index;

    // first and last line cannot contain the center character.
    for y_start in line_offset..haystack.len() - line_offset {
        let line = haystack.get(y_start).unwrap();
        let possible_hits_in_line = position_of_char(line, center_char);

        for x_start in possible_hits_in_line {
            if x_start == 0 {
                continue;
            }
            let possible_match = extract_string(
                haystack,
                ((x_start - 1) as i32, (y_start - 1) as i32),
                (1, 1),
                3,
                3,
            );
            let (x, y) = (x_start - 1, y_start - 1);
            if &possible_match == needle || possible_match == needle_reverse {
                let possible_match_2 = extract_string(
                    haystack,
                    ((x_start - 1) as i32, (y_start + 1) as i32),
                    (1, -1),
                    3,
                    3,
                );
                if &possible_match_2 == needle || possible_match_2 == needle_reverse {
                    hits += 1;
                }
            }
        }
    }

    return hits;
}
fn find_line_hits(haystack: &Vec<char>, needle: &Vec<char>) -> u16 {
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
fn position_of_char(haystack: &Vec<char>, c: &char) -> Vec<usize> {
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
fn extract_string(
    input: &Vec<Vec<char>>,
    start: (i32, i32),
    direction: (i32, i32),
    min_length: i32,
    max_length: i32,
) -> Vec<char> {
    let (mut x, mut y) = start;
    let mut ret: Vec<char> = Vec::new();
    let mut extracted_char_count = 0;
    let num_rows = input.len();
    match direction {
        // line
        (1, 0) => {
            ret = (*input.get(y as usize).unwrap().clone()).to_vec();
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
                            y += delta_y;
                            x += delta_x;
                            if extracted_char_count == max_length {
                                break;
                            }
                            if y < 0 || x < 0 {
                                break;
                            }
                            if y >= num_rows as i32 || x >= row_length as i32 {
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
fn sum_of_tuple_multiplications(input: &Vec<(u32, u32)>) -> u64 {
    let mut sum: u64 = 0;
    for (left, right) in input {
        sum += *left as u64 * *right as u64;
    }
    return sum;
}
fn parse_string_for_mul_instructions(input: &String, filter: Option<Regex>) -> Vec<(u32, u32)> {
    let mut ret = Vec::new();
    let mut filtered_input = input.clone();
    match filter {
        Some(filter) => {
            for (_, [content, _endtag]) in filter.captures_iter(input).map(|cap| cap.extract()) {
                filtered_input = filtered_input.replace(content, "")
            }
        }
        None => {}
    }

    let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
    for (_, [left, right]) in re.captures_iter(&filtered_input).map(|cap| cap.extract()) {
        let first: u32 = left.parse().unwrap();
        let second: u32 = right.parse().unwrap();
        ret.push((first, second));
    }
    return ret;
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
    }
    #[test]
    fn test_find_cross() {
        let mut input: Vec<Vec<char>> = Vec::new();
        if let Ok(lines) = read_lines("../data/input_4c.txt") {
            for line in lines.map_while(Result::ok) {
                let chars: Vec<char> = line.chars().map(|c| c).collect();
                input.push(chars);
            }
        }
        assert_eq!(
            9,
            find_cross_hits(&input, &"MAS".chars().collect::<Vec<char>>())
        );
        let mut input: Vec<Vec<char>> = Vec::new();
        if let Ok(lines) = read_lines("../data/input_4b.txt") {
            for line in lines.map_while(Result::ok) {
                let chars: Vec<char> = line.chars().map(|c| c).collect();
                input.push(chars);
            }
        }
        //too low
        assert_eq!(
            2029,
            find_cross_hits(&input, &"MAS".chars().collect::<Vec<char>>())
        );
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
    #[test]
    fn vector_element_multiplier() {
        let input = [(2, 4), (5, 5), (11, 8), (8, 5)].to_vec();
        assert_eq!(sum_of_tuple_multiplications(&input), 161);

        if let Ok(lines) = read_lines("../data/input_3.txt") {
            for line in lines.map_while(Result::ok) {
                let recovered_numbers = parse_string_for_mul_instructions(&line, None);
                assert_eq!(recovered_numbers.len(), 721);
                assert_eq!(sum_of_tuple_multiplications(&recovered_numbers), 189600467);

                let filter = Some(Regex::new(r"don't\(\)(.*?)(do\(\)|$)").unwrap());
                let recovered_numbers = parse_string_for_mul_instructions(&line, filter);
                assert_eq!(recovered_numbers.len(), 407);
                assert_eq!(sum_of_tuple_multiplications(&recovered_numbers), 107069718);
            }
        }
    }
    #[test]
    fn test_parse_mul_instructions() {
        let input =
            String::from("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))");

        assert_eq!(
            parse_string_for_mul_instructions(&input, None),
            [(2, 4), (5, 5), (11, 8), (8, 5)].to_vec()
        );
        let input = String::from(
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))don't()mul(88,66)",
        );
        let filter = Some(Regex::new(r"don't\(\)(.*?)($|do\(\))").unwrap());
        assert_eq!(
            parse_string_for_mul_instructions(&input, filter),
            [(2, 4), (8, 5)].to_vec()
        );
    }
    #[test]
    fn test_scan_test_with_find_line_hits() {
        let mut input: Vec<Vec<char>> = Vec::new();
        if let Ok(lines) = read_lines("../data/input_4a.txt") {
            for line in lines.map_while(Result::ok) {
                let chars: Vec<char> = line.chars().map(|c| c).collect();
                input.push(chars);
            }
        }
        let hits = scan(&input, "XMAS".to_string());
        assert_eq!(hits, 18);
    }
    #[test]
    fn test_word_search() {
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

        let mut input: Vec<Vec<char>> = Vec::new();
        if let Ok(lines) = read_lines("../data/input_4a.txt") {
            for line in lines.map_while(Result::ok) {
                let chars: Vec<char> = line.chars().map(|c| c).collect();
                input.push(chars);
            }
        }
        assert_eq!(
            extract_string(&input, (0, 0), (1, 1), 4, -1),
            "MSXMAXSAMX".chars().collect::<Vec<char>>(),
        );
        assert_eq!(
            extract_string(&input, (1, 0), (1, 1), 4, -1),
            "MASAMXXAM".chars().collect::<Vec<char>>()
        );
        assert_eq!(
            extract_string(&input, (9, 9), (1, -1), 4, -1),
            "".chars().collect::<Vec<char>>()
        );
        assert_eq!(
            extract_string(&input, (0, 9), (1, -1), 4, -1),
            "MAXMMMMASM".chars().collect::<Vec<char>>()
        );

        let mut input: Vec<Vec<char>> = Vec::new();
        if let Ok(lines) = read_lines("../data/input_4b.txt") {
            for line in lines.map_while(Result::ok) {
                let chars: Vec<char> = line.chars().map(|c| c).collect();
                input.push(chars);
            }
        }
        let hits = scan(&input, "XMAS".to_string());
        assert_eq!(hits, 2567);
    }
}
