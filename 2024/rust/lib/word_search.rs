use crate::{
    extract_vec_from_2d_vec, find_line_hits, position_of_char, read_lines, read_to_vec_vec_char,
};

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
                let column = extract_vec_from_2d_vec(
                    input,
                    (column_index as u32, 0),
                    crate::DOWN,
                    num_columns,
                    -1,
                );

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
        let start = (x as u32, y as u32);
        let line = extract_vec_from_2d_vec(&input, start, direction, needle.len() as i32, -1);
        hits += find_line_hits(&line, &needle);
        hits += find_line_hits(&line, &needle_reverse);
    }
    let y = num_rows as i32;
    let x_range = 1..=(num_columns - (needle.len() - 1) as i32);
    for x in x_range {
        let start = (x as u32, y as u32);

        let line = extract_vec_from_2d_vec(&input, start, direction, needle.len() as i32, -1);
        hits += find_line_hits(&line, &needle);
        hits += find_line_hits(&line, &needle_reverse);
    }
    let y = 0;
    let x_range = (0..=(num_columns - (needle.len() - 1) as i32)).rev();
    let direction = (1, 1);
    for x in x_range {
        let start = (x as u32, y as u32);
        let line = extract_vec_from_2d_vec(&input, start, direction, needle.len() as i32, -1);

        hits += find_line_hits(&line, &needle);
        hits += find_line_hits(&line, &needle_reverse);
    }
    let y_range = 1..=(num_rows - (needle.len() - 1)) as i32;
    let x: i32 = 0;
    for y in y_range {
        let start = (x as u32, y as u32);
        let line = extract_vec_from_2d_vec(&input, start, direction, needle.len() as i32, -1);
        hits += find_line_hits(&line, &needle);
        hits += find_line_hits(&line, &needle_reverse);
    }

    // https://github.com/rust-lang/rust/issues/70925 descending ranges dont work.
    return hits as u16;
}
    #[test]
    fn test_scan_test_with_find_line_hits() {
        let input: Vec<Vec<char>> = read_to_vec_vec_char("../data/input_4a.txt");
        let hits = scan(&input, "XMAS".to_string());
        assert_eq!(hits, 18);
    }

    #[test]
    fn test_word_search() {
        let mut input: Vec<Vec<char>> = Vec::new();
        if let Ok(lines) = read_lines("../data/input_4a.txt") {
            for line in lines.map_while(Result::ok) {
                let chars: Vec<char> = line.chars().map(|c| c).collect();
                input.push(chars);
            }
        }
        assert_eq!(
            extract_vec_from_2d_vec(&input, (0, 0), (1, 1), 4, -1),
            "MSXMAXSAMX".chars().collect::<Vec<char>>(),
        );
        assert_eq!(
            extract_vec_from_2d_vec(&input, (1,0), (1, 1), 4, -1),
            "MASAMXXAM".chars().collect::<Vec<char>>()
        );
        assert_eq!(
            extract_vec_from_2d_vec(&input, (9, 9), (1, -1), 4, -1),
            "".chars().collect::<Vec<char>>()
        );
        assert_eq!(
            extract_vec_from_2d_vec(&input, (0, 9), (1, -1), 4, -1),
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
pub fn find_cross_hits(haystack: &Vec<Vec<char>>, needle: &Vec<char>) -> u16 {
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
            let possible_match = extract_vec_from_2d_vec(
                haystack,
                ((x_start - 1) as u32, (y_start - 1) as u32),
                (1, 1),
                3,
                3,
            );
            if &possible_match == needle || possible_match == needle_reverse {
                let possible_match_2 = extract_vec_from_2d_vec(
                    haystack,
                    ((x_start - 1) as u32, (y_start + 1) as u32),
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
    #[test]
    fn test_find_cross() {
        let input: Vec<Vec<char>> = read_to_vec_vec_char("../data/input_4c.txt");
        assert_eq!(
            9,
            find_cross_hits(&input, &"MAS".chars().collect::<Vec<char>>())
        );

        let input: Vec<Vec<char>> = read_to_vec_vec_char("../data/input_4b.txt");
        assert_eq!(
            2029,
            find_cross_hits(&input, &"MAS".chars().collect::<Vec<char>>())
        );
    }
