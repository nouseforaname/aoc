use std::collections::HashMap;

use regex::Regex;

use crate::{
    distance, is_series_safe, read_book_printing_data, read_column_data_to_vec,
    read_data_to_vec_of_tuples, read_lines, read_ordered_list_data_to_vec,
};
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
        ret.push((left.parse().unwrap(), right.parse().unwrap()));
    }
    return ret;
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
fn test_count_safe_rector_reports() {
    let list = read_ordered_list_data_to_vec("../data/input_2.tsv").to_vec();
    let reactor_levels = count_safe_reactor_levels(&list, false);
    assert_eq!(reactor_levels, 680);

    let list = read_ordered_list_data_to_vec("../data/input_2.tsv").to_vec();
    let reactor_levels = count_safe_reactor_levels(&list, true);
    assert_eq!(reactor_levels, 710);
}
fn count_safe_reactor_levels(rows: &Vec<Vec<u32>>, dampener: bool) -> usize {
    rows.iter()
        .filter(|row| is_series_safe(&row, dampener))
        .count()
}

pub fn sum_of_tuple_multiplications(input: &Vec<(u32, u32)>) -> u64 {
    let mut sum: u64 = 0;
    input.iter().for_each(|(left, right)| {
        sum += *left as u64 * *right as u64;
    });
    return sum;
}
#[test]
fn test_sum_of_tuple_multiplication() {
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
        distance_of_all_elements(&mut list[0].to_owned(), &mut list[1].to_owned()),
        2176849
    );
}
pub fn distance_of_all_elements(list_a: &mut Vec<u32>, list_b: &mut Vec<u32>) -> u64 {
    list_a.sort();
    list_b.sort();
    let mut sum: u64 = 0;
    for (index, element_a) in list_a.iter().enumerate() {
        let element_b = list_b[index];
        sum += distance(*element_a, element_b)
    }
    return sum;
}

pub fn similarity_score_of_all_elements(list_a: &Vec<u32>, list_b: &Vec<u32>) -> u64 {
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

#[test]
fn test_similarity() {
    let list = read_column_data_to_vec("../data/input_1.tsv");
    assert!(similarity_score_of_all_elements(&list[0], &list[1]) == 23384288);
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
    // clean, offending element positions
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
