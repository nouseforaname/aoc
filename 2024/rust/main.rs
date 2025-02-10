mod lib;
use lib::helpers::*;
use regex::Regex;
use std::collections::HashMap;
use std::fs::read_to_string;

// - binary string as array of operations
#[test]
fn test_check_if_calculation_is_doable() {
    let numbers = [10, 19].to_vec();
    assert!(check_if_calculation_is_doable((190, &numbers), true));
    assert!(check_if_calculation_is_doable(
        (156, &[15, 6].to_vec()),
        true
    ));
    assert!(check_if_calculation_is_doable(
        (3267, &[81, 40, 27].to_vec()),
        true
    ));
    assert!(check_if_calculation_is_doable(
        (292, &[11, 6, 16, 20].to_vec()),
        true
    ));

    let input = read_data_to_vec_of_tuples("../data/input_7a.txt".to_string());
    let mut res = 0;
    for (sum, numbers) in input.iter() {
        if check_if_calculation_is_doable((*sum, numbers), true) {
            res += sum;
        }
    }
    assert_eq!(res, 11387);

    let mut res = 0;
    let input = read_data_to_vec_of_tuples("../data/input_7b.txt".to_string());

    println!("--------starting");
    let start = std::time::Instant::now();
    for (sum, numbers) in input.iter() {
        if check_if_calculation_is_doable((*sum, numbers), true) {
            res += sum;
        }
    }
    let end = start.elapsed();
    println!("numbers took {end:.2?}");
    assert_eq!(res, 104824810233437);
}

fn check_if_calculation_is_doable(input: (u64, &Vec<u64>), with_or_operator: bool) -> bool {
    let (sum, numbers) = input;
    let result = &mut [*numbers.first().unwrap()].to_vec();

    numbers[1..].iter().for_each(|b| {
        *result = result
            .iter()
            .map(|a| {
                let mut res = [a * b, a + b].to_vec();
                if with_or_operator {
                    res.push(format!("{a}{b}").parse().unwrap());
                }
                res
            })
            .flatten()
            .filter(|&e| e <= sum)
            .collect::<Vec<u64>>();
    });
    match result.iter().find(|&e| *e == sum) {
        Some(_) => {
            return true;
        }
        None => {
            return false;
        }
    };
}

fn main() {}

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
fn read_data_to_vec_of_tuples(path: String) -> Vec<(u64, Vec<u64>)> {
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
fn find_start(input: &Vec<Vec<char>>) -> (i32, i32) {
    let (x, y) = (0, 0);
    let len = input.len();
    for y in 0..len {
        let line = input.get(y).unwrap();
        let positions = position_of_char(&line, &'^');
        if positions.len() > 0 {
            return (*positions.get(0).unwrap() as i32, y as i32);
        }
    }

    return (x, y);
}
fn next_coord(start: (i32, i32), direction: (i32, i32)) -> (i32, i32) {
    return add_offset_to_coord(start, direction, 1);
}
fn next_direction(direction: (i32, i32)) -> (i32, i32) {
    match direction {
        UP => return RIGHT,
        DOWN => return LEFT,
        RIGHT => return DOWN,
        LEFT => return UP,
        _ => todo!(),
    }
}
fn add_offset_to_coord(start: (i32, i32), direction: (i32, i32), distance: i32) -> (i32, i32) {
    let (x, y) = start;
    match direction {
        UP => return (x, y - distance),
        DOWN => return (x, y + distance),
        RIGHT => return (x + distance, y),
        LEFT => return (x - distance, y),
        _ => todo!(),
    }
}
fn direction_to_string(direction: (i32, i32)) -> String {
    match direction {
        DOWN => "DOWN".to_string(),
        UP => "UP".to_string(),
        LEFT => "LEFT".to_string(),
        RIGHT => "RIGHT".to_string(),
        _ => "".to_string(),
    }
}

fn write_char_to_coord(input: &mut Vec<Vec<char>>, coord: (i32, i32), c: char) -> bool {
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
const UP: (i32, i32) = (0, -1);
const DOWN: (i32, i32) = (0, 1);
const LEFT: (i32, i32) = (-1, 0);
const RIGHT: (i32, i32) = (1, 0);
fn can_place_obstacle(input: &Vec<Vec<char>>, coord: (i32, i32)) -> bool {
    let (x, y) = coord;

    match input.get(y as usize) {
        Some(line) => match line.get(x as usize) {
            Some(c) => c == &'.',
            None => false,
        },
        None => false,
    }
}
fn find_path_and_loops(
    start: (i32, i32),
    direction: (i32, i32),
    input: &mut Vec<Vec<char>>,
) -> (i32, i32) {
    let mut possible_loops = 0;
    let mut found_exit = false;
    let mut current_position = start.clone();
    let mut hit_obstacles = Vec::new();
    let mut direction = direction.clone();

    while !found_exit {
        let (coord, mut distance) = find_next_obstacle(current_position, direction, input);

        match coord {
            Some(val) => {
                println!(
                    "found obstacle {coord:?} {}",
                    direction_to_string(direction)
                );
                hit_obstacles.push((val, direction));
            }
            None => {
                println!(
                    "found exit {current_position:?} {}",
                    direction_to_string(direction)
                );
                found_exit = true;
            }
        }
        let next_dir = next_direction(direction);
        while distance > 0 {
            if can_place_obstacle(input, next_coord(current_position, direction)) {
                match find_next_obstacle(current_position, next_dir, input) {
                    (Some(blocker_location), _) => {
                        // placing blocker in front would make us hit another blocker

                        // if that blocker was already hit from the same direction before, that's a loop
                        if hit_obstacles.contains(&(blocker_location, next_dir)) {
                            possible_loops += 1;
                            println!("direct confirmation, {possible_loops}");
                        } else {
                            if write_char_to_coord(
                                input,
                                next_coord(current_position, direction),
                                'O',
                            ) {
                                assert!(write_char_to_coord(
                                    input,
                                    next_coord(current_position, direction),
                                    '#'
                                ));
                                if check_for_loop(input, current_position, next_dir) {
                                    possible_loops += 1;
                                    println!("walking confirmation, {possible_loops}");
                                } else {
                                    println!("disproven by walking");
                                };
                                assert!(write_char_to_coord(
                                    input,
                                    next_coord(current_position, direction),
                                    '.'
                                ));
                            };
                        }
                    }
                    (None, _) => {
                        // no obstacle in that direction, so placing a blocker in front would just exit the maze
                    }
                }
            }
            write_char_to_coord(input, current_position, 'X');
            distance -= 1;
            current_position = next_coord(current_position, direction);
        }
        direction = next_direction(direction);
    }

    let mut path_length = 0;

    for y in 0..input.len() {
        path_length += input
            .get(y)
            .unwrap()
            .iter()
            .filter(|element| *element == &'X')
            .collect::<Vec<&char>>()
            .len();
    }

    return (path_length as i32, possible_loops);
}
fn check_for_loop(input: &Vec<Vec<char>>, start: (i32, i32), direction: (i32, i32)) -> bool {
    let mut obstacles = [].to_vec();

    let mut start = start.clone();
    let mut direction = direction.clone();
    loop {
        match find_next_obstacle(start, direction, input) {
            (Some(location), distance) => {
                let obstacle = (location, direction);
                if obstacles.contains(&obstacle) {
                    println!("i've this this before {obstacle:?}");
                    return true;
                }
                obstacles.push(obstacle);
                start = add_offset_to_coord(start, direction, distance as i32);
                direction = next_direction(direction);
            }
            (None, _) => {
                return false;
            }
        };
    }
}

fn find_next_obstacle(
    start: (i32, i32),
    direction: (i32, i32),
    input: &Vec<Vec<char>>,
) -> (Option<(i32, i32)>, u32) {
    let path = extract_string(input, start, direction, 1 as i32, -1 as i32);
    match position_of_char(&path, &'#').first() {
        Some(distance) => {
            return (
                Some(add_offset_to_coord(start, direction, *distance as i32)),
                *distance as u32 - 1,
            );
        }
        None => {
            //println!("found exit in {direction:?} from {start:?}");
            return (None, path.len() as u32);
        }
    }
}

fn find_and_fix_broken_book_updates(
    rules: &HashMap<i32, Vec<u16>>,
    input: &Vec<Vec<u16>>,
) -> Vec<Vec<u16>> {
    let mut ret: Vec<Vec<u16>> = Vec::new();

    for update in input.iter() {
        let (clean, _, _) = check_update_ordering(&rules, update);
        if !clean {
            ret.push(fix_bad_book_update(&rules, &update));
        }
    }

    return ret;
}
fn fix_bad_book_update(rules: &HashMap<i32, Vec<u16>>, input: &Vec<u16>) -> Vec<u16> {
    let mut clean = false;
    let mut ret = input.clone();
    let mut offender: Option<i32>;
    let mut rule: Option<i32>;
    while !clean {
        (clean, offender, rule) = check_update_ordering(rules, &ret);
        match (offender, rule) {
            (Some(offender), Some(rule)) => {
                let left = ret.iter().position(|element| *element as i32 == offender);

                let right = ret.iter().position(|element| *element as i32 == rule);
                match (left, right) {
                    (Some(left), Some(right)) => ret.swap(left, right),
                    _ => continue,
                }
            }
            _ => continue,
        }
    }
    return ret;
}
fn calculate_book_middle_page_sum(rules: &HashMap<i32, Vec<u16>>, input: &Vec<Vec<u16>>) -> u32 {
    let mut sum: u32 = 0;
    for update in input.iter() {
        let (clean, _, _) = check_update_ordering(&rules, update);
        if clean {
            sum += *update.get(update.len() / 2).unwrap() as u32;
        }
    }
    return sum;
}
fn check_update_ordering(
    rules: &HashMap<i32, Vec<u16>>,
    input: &Vec<u16>,
) -> (bool, Option<i32>, Option<i32>) {
    for (index, element) in input.iter().enumerate() {
        match rules.get(&(*element as i32)) {
            Some(rule) => {
                for check_element in rule {
                    match input.iter().position(|element| element == check_element) {
                        Some(check_element_index) => {
                            if check_element_index < index {
                                return (false, Some(*element as i32), Some(*check_element as i32));
                            }
                        }
                        None => {}
                    }
                }
            }
            None => {}
        }
    }
    return (true, None, None);
}

fn read_book_printing_data(path: &str) -> (HashMap<i32, Vec<u16>>, Vec<Vec<u16>>) {
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
fn scan(input: &Vec<Vec<char>>, needle: String) -> u16 {
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
                let column = extract_string(input, (column_index, 0), DOWN, num_columns, -1);

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

fn distance_of_all_elements(list_a: &mut Vec<u32>, list_b: &mut Vec<u32>) -> u64 {
    list_a.sort();
    list_b.sort();
    let mut sum: u64 = 0;
    for (index, element_a) in list_a.iter().enumerate() {
        let element_b = list_b[index];
        sum += distance(*element_a, element_b)
    }
    return sum;
}
fn similarity_score_of_all_elements(list_a: &Vec<u32>, list_b: &Vec<u32>) -> u64 {
    let mut score: u64 = 0;
    list_a
        .iter()
        .map(|&a| {
            a as u64
                * list_b
                    .iter()
                    .filter(|&b| a == *b)
                    .collect::<Vec<&u32>>()
                    .len() as u64
        })
        .for_each(|element| score += element);
    return score;
}

fn sum_of_tuple_multiplications(input: &Vec<(u32, u32)>) -> u64 {
    let mut sum: u64 = 0;
    input.iter().for_each(|(left, right)| {
        sum += *left as u64 * *right as u64;
    });
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
fn is_series_safe(row: &Vec<u32>, direction: (i32, i32), dampener: bool) -> (bool, usize) {
    let length = row.len() - 1;
    let distance_range = 1..4;
    let mut offender = 0;
    let mut is_safe = true;
    for current in 0..length {
        let next = current + 1;
        match direction {
            UP => {
                if row[current] > row[next] {
                    is_safe = false;
                    offender = current;
                }
            }
            DOWN => {
                if row[current] < row[next] {
                    is_safe = false;
                    offender = current;
                }
            }
            _ => todo!(),
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
        (safe, _) = is_series_safe(&row.to_vec(), UP, dampener);
        if safe {
            ret.push(true);
            continue;
        }
        (safe, _) = is_series_safe(&row.to_vec(), DOWN, dampener);
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

#[cfg(test)]
mod tests {
    use super::*;

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
    #[test]
    fn test_find_path_and_loops() {
        let mut maze = read_to_vec_vec_char("../data/input_6a.txt");
        let (_, loops) = find_path_and_loops((4, 6), UP, &mut maze);
        assert_eq!(loops, 6);

        let mut maze = read_to_vec_vec_char("../data/input_6b.txt");
        let (ret, loops) = find_path_and_loops((4, 6), UP, &mut maze);
        assert_eq!(ret, 41);
        assert_eq!(loops, 7);
        let mut maze = read_to_vec_vec_char("../data/input_6c.txt");
        let start = find_start(&maze);
        assert_eq!(start, (47, 49));
        let (ret, loops) = find_path_and_loops(start, UP, &mut maze);
        assert_eq!(ret, 5444);
        assert_eq!(loops, 1946);
    }
    #[test]
    fn test_find_next_obstacle() {
        let maze = read_to_vec_vec_char("../data/input_6e.txt");

        let start = (0, 1);
        let (position, distance) = find_next_obstacle(start, RIGHT, &maze);
        assert_eq!(position.unwrap(), (9, 1));
        assert_eq!(distance, 8);

        let start = (8, 1);
        let (position, distance) = find_next_obstacle(start, DOWN, &maze);
        assert_eq!(position.unwrap(), (8, 9));
        assert_eq!(distance, 7);

        let start = (8, 8);
        let (position, distance) = find_next_obstacle(start, LEFT, &maze);
        assert_eq!(position.unwrap(), (2, 8));
        assert_eq!(distance, 5);

        let start = (1, 8);
        let (position, distance) = find_next_obstacle(start, UP, &maze);
        assert_eq!(position.unwrap(), (1, 0));
        assert_eq!(distance, 7);
    }
    #[test]
    fn test_check_for_loop() {
        let input = read_to_vec_vec_char("../data/input_6e.txt");

        assert_eq!(check_for_loop(&input, (1, 9), UP), true);
        assert_eq!(check_for_loop(&input, (3, 9), UP), true);
    }
    #[test]
    fn test_find_start() {
        let maze = read_to_vec_vec_char("../data/input_6d.txt");
        assert_eq!(find_start(&maze), (4, 6));
        let maze = read_to_vec_vec_char("../data/input_6a.txt");
        assert_eq!(find_start(&maze), (4, 7));
    }
    #[test]
    fn test_escape_maze_path_calc() {
        let maze = read_to_vec_vec_char("../data/input_6a.txt");
        let (blocker_coord, distance) = find_next_obstacle((4, 1), UP, &maze);
        assert_eq!(blocker_coord.unwrap(), (4, 0));
        assert_eq!(distance, 0);
        let (blocker_coord, distance) = find_next_obstacle((4, 1), RIGHT, &maze);
        assert_eq!(blocker_coord.unwrap(), (9, 1));
        assert_eq!(distance, 4);

        let (blocker_coord, distance) = find_next_obstacle((8, 1), DOWN, &maze);
        assert_eq!(blocker_coord.unwrap(), (8, 7));
        assert_eq!(distance, 5);

        let (blocker_coord, distance) = find_next_obstacle((8, 6), LEFT, &maze);
        assert_eq!(blocker_coord.unwrap(), (1, 6));
        assert_eq!(distance, 6);

        let (blocker_coord, distance) = find_next_obstacle((2, 6), UP, &maze);
        assert_eq!(blocker_coord.unwrap(), (2, 3));
        assert_eq!(distance, 2);

        let (blocker_coord, distance) = find_next_obstacle((2, 4), RIGHT, &maze);
        assert_eq!(blocker_coord.unwrap(), (7, 4));
        assert_eq!(distance, 4);

        let (blocker_coord, distance) = find_next_obstacle((6, 4), DOWN, &maze);
        assert_eq!(blocker_coord.unwrap(), (6, 9));
        assert_eq!(distance, 4);

        let (blocker_coord, distance) = find_next_obstacle((6, 8), LEFT, &maze);
        assert_eq!(blocker_coord.unwrap(), (0, 8));
        assert_eq!(distance, 5);

        let (blocker_coord, distance) = find_next_obstacle((1, 8), UP, &maze);
        assert_eq!(blocker_coord.unwrap(), (1, 6));
        assert_eq!(distance, 1);

        let (blocker_coord, distance) = find_next_obstacle((1, 7), RIGHT, &maze);
        assert_eq!(blocker_coord.unwrap(), (8, 7));
        assert_eq!(distance, 6);

        let (blocker_coord, distance) = find_next_obstacle((7, 7), DOWN, &maze);
        assert_eq!(blocker_coord, None);
        assert_eq!(distance, 3);

        let mut maze = read_to_vec_vec_char("../data/input_6b.txt");
        let (ret, _) = find_path_and_loops((4, 6), UP, &mut maze);
        assert_eq!(ret, 41);

        let mut maze = read_to_vec_vec_char("../data/input_6c.txt");
        let (ret, _) = find_path_and_loops((47, 49), UP, &mut maze);
        assert_eq!(ret, 5444);
    }
    #[test]
    fn test_fix_bad_book_update() {
        let (rules, _updates) = read_book_printing_data("../data/input5a.txt");
        assert_eq!(
            fix_bad_book_update(&rules, &[75, 97, 47, 61, 53].to_vec()),
            [97, 75, 47, 61, 53]
        );
        assert_eq!(
            fix_bad_book_update(&rules, &[61, 13, 29].to_vec()),
            [61, 29, 13]
        );
        assert_eq!(
            fix_bad_book_update(&rules, &[97, 13, 75, 29, 47].to_vec()),
            [97, 75, 47, 29, 13]
        );
    }
    #[test]
    fn test_find_and_fix_broken_book_updates() {
        let (rules, updates) = read_book_printing_data("../data/input5a.txt");
        let fixed_updates = find_and_fix_broken_book_updates(&rules, &updates);
        assert_eq!(123, calculate_book_middle_page_sum(&rules, &fixed_updates));

        let (rules, updates) = read_book_printing_data("../data/input_5b.txt");
        assert_eq!(calculate_book_middle_page_sum(&rules, &updates), 5091);

        let fixed_updates = find_and_fix_broken_book_updates(&rules, &updates);
        assert_eq!(4681, calculate_book_middle_page_sum(&rules, &fixed_updates));
    }
    #[test]
    fn test_calculate_middle_page_sum() {
        let (rules, updates) = read_book_printing_data("../data/input5a.txt");
        assert_eq!(calculate_book_middle_page_sum(&rules, &updates), 143);
    }
    #[test]
    fn test_check_update_ordering() {
        let (rules, updates) = read_book_printing_data("../data/input5a.txt");

        assert_eq!(
            check_update_ordering(&rules, updates.first().unwrap()),
            (true, None, None)
        );
        assert_eq!(
            check_update_ordering(&rules, updates.get(1).unwrap()),
            (true, None, None)
        );
        assert_eq!(
            check_update_ordering(&rules, updates.get(2).unwrap()),
            (true, None, None)
        );
        assert_eq!(
            check_update_ordering(&rules, updates.get(3).unwrap()),
            (false, Some(97), Some(75))
        );
        assert_eq!(
            check_update_ordering(&rules, updates.get(4).unwrap()),
            (false, Some(29), Some(13))
        );
    }
    #[test]
    fn test_read_book_printing_data() {
        let (rules, updates) = read_book_printing_data("../data/input5a.txt");

        assert_eq!(rules.len(), 6);
        assert_eq!(updates.len(), 6);

        assert_eq!(rules.get(&75).unwrap(), &[29, 53, 47, 61, 13].to_vec());
        assert_eq!(updates.last().unwrap(), &[97, 13, 75, 29, 47].to_vec())
    }
    #[test]
    fn test_distance_of_all_elements() {
        assert_eq!(
            distance_of_all_elements(
                &mut [3, 4, 2, 1, 3, 3].to_vec(),
                &mut [4, 3, 5, 3, 9, 3].to_vec()
            ),
            11
        );
        let list = read_column_data_to_vec("../data/input_1.tsv");
        assert_eq!(
            distance_of_all_elements(&mut list[0].to_owned(), &mut list[1].to_owned()), 2176849
        );
    }

    #[test]
    fn test_similarity() {
        let list = read_column_data_to_vec("../data/input_1.tsv");
        assert!(similarity_score_of_all_elements(&list[0], &list[1]) == 23384288);
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
        assert!(is_series_safe(&test, DOWN, false) == (true, test.len()));
        assert_eq!(is_series_safe(&test, UP, false), (false, 0));
        // clean up series with proper distance
        let test = [1, 2, 4, 6, 9].to_vec();
        assert!(is_series_safe(&test, UP, false) == (true, test.len()));
        assert!(is_series_safe(&test, DOWN, false) == (false, 0));

        //first element bad in up series
        let test = [5, 4, 7, 8, 9].to_vec();
        assert!(is_series_safe(&test, UP, false) == (false, 0));
        assert!(is_series_safe(&test, DOWN, false) == (false, 1));

        let mut test_remove = test.clone();
        test_remove.remove(0);
        assert!(is_series_safe(&test_remove, UP, false) == (true, test_remove.len()));

        ////first element bad in down series
        let test = [4, 4, 3, 2, 1].to_vec();
        assert!(is_series_safe(&test, UP, false) == (false, 0));
        assert!(is_series_safe(&test, DOWN, false) == (false, 0));

        let mut test_remove = test.clone();
        test_remove.remove(0);
        assert!(is_series_safe(&test_remove, UP, false) == (false, 0));
        assert!(is_series_safe(&test_remove, DOWN, false) == (true, test_remove.len()));

        //last element bad with too high distance up series
        let test = [1, 3, 6, 7, 19].to_vec();
        assert_eq!(is_series_safe(&test, UP, false), (false, test.len() - 1));

        //last element bad with too high distance down series
        let test = [33, 31, 29, 27, 19].to_vec();
        assert_eq!(is_series_safe(&test, DOWN, false), (false, test.len() - 1));

        //middle element bad with direction
        let test = [1, 3, 2, 4, 5].to_vec();
        assert_eq!(is_series_safe(&test, UP, false), (false, 1));

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
            extract_string(&input, RIGHT, (1, 1), 4, -1),
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
