use std::collections::HashMap;
use std::fs::read_to_string;

use crate::{read_to_vec_vec_char, write_char_to_coord};

#[test]
fn test_calc_interferences() {
    assert_eq!(
        calc_interferences(&"../data/input_8a.txt".to_string(), false),
        14
    );
    assert_eq!(
        calc_interferences(&"../data/input_8d.txt".to_string(), false),
        351
    );
    assert_eq!(
        calc_interferences(&"../data/input_8e.txt".to_string(), true),
        9
    );

    assert_eq!(
        calc_interferences(&"../data/input_8a.txt".to_string(), true),
        34
    );
    assert_eq!(
        calc_interferences(&"../data/input_8d.txt".to_string(), true),
        1259
    );
}
fn calc_interferences(path: &String, amplify: bool) -> u32 {
    let (signal_coords, width, height) = find_antenna_positions(path.to_owned());
    let mut out_map = read_to_vec_vec_char(path);
    let mut count = 0;
    signal_coords.iter().for_each(|(_, signal_coord)| {
        find_interferences(signal_coord, width, height, amplify)
            .iter()
            .for_each(|coord| {
                let (x, y) = *coord;
                if out_map[y as usize][x as usize] != '#' {
                    write_char_to_coord(&mut out_map, *coord, '#');
                    count += 1;
                }
            });
    });

    count
}
#[test]
fn test_find_antenna_positions() {
    let (map, _, _) = find_antenna_positions("../data/input_8a.txt".to_string());
    assert_eq!(map.get(&'0').unwrap(), &[(8, 1), (5, 2), (7, 3), (4, 4)]);
    assert_eq!(map.get(&'A').unwrap(), &[(6, 5), (8, 8), (9, 9)]);
}
pub fn find_antenna_positions(path: String) -> (HashMap<char, Vec<(u32, u32)>>, u32, u32) {
    let mut ret: HashMap<char, Vec<(u32, u32)>> = HashMap::new();
    let (mut x, mut y) = (0, 0);
    match read_to_string(&path) {
        Ok(lines) => {
            let mut width = 0;
            lines.lines().for_each(|line| {
                width = line.len() as u32;
                line.chars().for_each(|c| {
                    match ret.get_mut(&c) {
                        Some(vec) => {
                            if c != '.' {
                                vec.push((x, y))
                            }
                        }
                        None => {
                            ret.insert(c, [(x, y)].to_vec());
                        }
                    };
                    x += 1;
                });
                x = 0;
                y += 1;
            });
            (ret, width, lines.lines().count() as u32)
        }
        Err(e) => {
            panic!("{}", format!("error reading file: '{}' '{e:?}'", &path));
        }
    }
}

#[test]
fn test_find_interferences() {
    let (map, width, height) = find_antenna_positions("../data/input_8b.txt".to_string());
    let interferences = find_interferences(&map.get(&'a').unwrap(), width, height, false);
    assert_eq!(interferences, [(3, 1), (6, 7)]);

    let (map, width, height) = find_antenna_positions("../data/input_8c.txt".to_string());

    let interferences = find_interferences(&map.get(&'a').unwrap(), width, height, false);
    assert_eq!(interferences, [(0, 2), (3, 1), (6, 7), (2, 6)]);
}
pub fn find_interferences(
    input: &Vec<(u32, u32)>,
    max_x: u32,
    max_y: u32,
    amplify: bool,
) -> Vec<(u32, u32)> {
    let mut ret: Vec<(u32, u32)> = input
        .iter()
        .map(|a| {
            input
                .iter()
                .map(|b| {
                    if a != b {
                        let (ax, ay) = *a;
                        let (bx, by) = *b;
                        let (dx, dy) = (ax as i64 - bx as i64, ay as i64 - by as i64);

                        let (mut x, mut y) = (ax as i64 + dx, ay as i64 + dy);
                        println!("{a:?} {b:?} D {dx}:{dy} {}:{}", x, y);

                        let mut ret: Vec<Option<(u32, u32)>> = Vec::new();
                        while !(x < 0 || y < 0 || x >= max_x as i64 || y >= max_y as i64) {
                            ret.push(Some((x as u32, y as u32)));
                            if !amplify {
                                x = -1;
                            } else {
                                if input
                                    .iter()
                                    .find(|(p, q)| {
                                        println!("{p}:{q} == {x}:{y}");
                                        *p as i64 == x && *q as i64 == y
                                    })
                                    .is_some()
                                {
                                    ret.push(Some(*a));
                                    ret.push(Some(*b));
                                }
                                (x, y) = (x as i64 + dx, y as i64 + dy);
                            }
                        }
                        ret
                    } else {
                        [None].to_vec()
                    }
                })
                .flatten()
                .filter(|&e| e != None)
                .map(|e| e.unwrap())
                .collect::<Vec<(u32, u32)>>()
        })
        .flatten()
        .collect();

    if amplify && input.len() > 2 {
        input.iter().for_each(|&e| ret.push(e));
    }
    ret
}
