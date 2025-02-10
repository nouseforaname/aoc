use crate::position_of_char;
pub fn write_char_to_coord(input: &mut Vec<Vec<char>>, coord: (i32, i32), c: char) -> bool {
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
pub const UP: (i32, i32) = (0, -1);
pub const DOWN: (i32, i32) = (0, 1);
pub const LEFT: (i32, i32) = (-1, 0);
pub const RIGHT: (i32, i32) = (1, 0);
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
    let maze = crate::read_to_vec_vec_char("../data/input_6d.txt");
    assert_eq!(find_start(&maze), (4, 6));
    let maze = crate::read_to_vec_vec_char("../data/input_6a.txt");
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
