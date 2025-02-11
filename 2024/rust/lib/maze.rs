use crate::{extract_vec_from_2d_vec, position_of_char, read_to_vec_vec_char, write_char_to_coord};
use crate::{DOWN, LEFT, RIGHT, UP};

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

pub fn find_path_and_loops(
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
                            //println!("direct confirmation, {possible_loops}");
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
                                    //println!("walking confirmation, {possible_loops}");
                                } else {
                                    //println!("disproven by walking");
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
    let unique_fields_walked = input
        .iter()
        .map(|line| line.iter().filter(|&e| *e == 'X').count() as i32)
        .sum();
    return (unique_fields_walked, possible_loops);
}

pub fn can_place_obstacle(input: &Vec<Vec<char>>, coord: (i32, i32)) -> bool {
    let (x, y) = coord;

    match input.get(y as usize) {
        Some(line) => match line.get(x as usize) {
            Some(c) => c == &'.',
            None => false,
        },
        None => false,
    }
}

#[test]
pub fn test_find_start() {
    let maze = read_to_vec_vec_char("../data/input_6d.txt");
    assert_eq!(find_start(&maze), (4, 6));
    let maze = read_to_vec_vec_char("../data/input_6a.txt");
    assert_eq!(find_start(&maze), (4, 7));
}
pub fn find_start(input: &Vec<Vec<char>>) -> (i32, i32) {
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

pub fn next_coord(start: (i32, i32), direction: (i32, i32)) -> (i32, i32) {
    return add_offset_to_coord(start, direction, 1);
}

pub fn next_direction(direction: (i32, i32)) -> (i32, i32) {
    match direction {
        UP => return RIGHT,
        DOWN => return LEFT,
        RIGHT => return DOWN,
        LEFT => return UP,
        _ => todo!(),
    }
}

pub fn add_offset_to_coord(start: (i32, i32), direction: (i32, i32), distance: i32) -> (i32, i32) {
    let (x, y) = start;
    match direction {
        UP => return (x, y - distance),
        DOWN => return (x, y + distance),
        RIGHT => return (x + distance, y),
        LEFT => return (x - distance, y),
        _ => todo!(),
    }
}
pub fn direction_to_string(direction: (i32, i32)) -> String {
    match direction {
        DOWN => "DOWN".to_string(),
        UP => "UP".to_string(),
        LEFT => "LEFT".to_string(),
        RIGHT => "RIGHT".to_string(),
        _ => "".to_string(),
    }
}

pub fn check_for_loop(input: &Vec<Vec<char>>, start: (i32, i32), direction: (i32, i32)) -> bool {
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

pub fn find_next_obstacle(
    start: (i32, i32),
    direction: (i32, i32),
    input: &Vec<Vec<char>>,
) -> (Option<(i32, i32)>, u32) {
    let path = extract_vec_from_2d_vec(input, start, direction, 1 as i32, -1 as i32);
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
