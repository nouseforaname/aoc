#[test]
fn test_read_vec_op_u32() {
    let ret = read_vec_vec_op_u32("../data/input_10a.txt");

    assert_eq!(ret[0][0].unwrap(), 1);
    assert_eq!(ret[0][1].unwrap(), 0);
    assert_eq!(ret[6][6].unwrap(), 1);
    assert_eq!(ret[6][5].unwrap(), 0);

    let ret = read_vec_vec_op_u32("../data/input_10b.txt");
    assert!(ret[0][0].is_none());
    assert!(ret[0][3].is_some());
    assert!(ret[1][3].is_some());
    assert!(ret[2][3].is_some());
    assert!(ret[3].iter().all(|e| e.is_some()));
}

fn read_vec_vec_op_u32(path: &str) -> Vec<Vec<Option<u32>>> {
    return std::fs::read_to_string(path)
        .unwrap()
        .trim()
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match format!("{c}").parse::<u32>() {
                    Ok(val) => Some(val),
                    _ => None,
                })
                .collect()
        })
        .collect();
}
#[test]
fn test_find_entry_points() {
    let input = read_vec_vec_op_u32("../data/input_10a.txt");
    let ret = find_entry_ponts(&input);
    assert_eq!(ret.len(), 2);
    assert_eq!(ret[0], (1, 0));
    assert_eq!(&input[0][1].unwrap(), &0);
    assert_eq!(ret[1], (5, 6));
    assert_eq!(&input[6][5].unwrap(), &0);
    let input = read_vec_vec_op_u32("../data/input_10b.txt");
    let ret = find_entry_ponts(&input);
    assert_eq!(ret, [(3, 0)]);
    assert_eq!(input[0][3].unwrap(), 0);

    let map = read_vec_vec_op_u32("../data/input_10d.txt");
    assert_eq!(find_entry_ponts(&map).len(), 9);
    assert_eq!(
        find_entry_ponts(&map),
        [
            (2, 0),
            (4, 0),
            (4, 2),
            (6, 4),
            (2, 5),
            (5, 5),
            (0, 6),
            (6, 6),
            (1, 7)
        ]
        .to_vec()
    );
}
fn find_entry_ponts(map: &Vec<Vec<Option<u32>>>) -> Vec<(usize, usize)> {
    map.iter()
        .enumerate()
        .map(|(y, line)| {
            line.iter()
                .enumerate()
                .filter_map(|(x, e)| {
                    if e.is_some_and(|e| e == 0) {
                        Some((x, y))
                    } else {
                        None
                    }
                })
                .collect::<Vec<(usize, usize)>>()
        })
        .flatten()
        .collect()
}

#[test]
fn test_get_surrounding() {
    assert_eq!(
        get_surrounding_coords((0, 0), 3, 3),
        [(1, 0), (0, 1)].to_vec()
    );
    assert_eq!(
        get_surrounding_coords((1, 1), 3, 3),
        [(2, 1), (0, 1), (1, 2), (1, 0)].to_vec()
    );
    assert_eq!(
        get_surrounding_coords((1, 0), 3, 3),
        [(2, 0), (0, 0), (1, 1)].to_vec()
    );
    assert_eq!(
        get_surrounding_coords((0, 1), 3, 3),
        [(1, 1), (0, 2), (0, 0)].to_vec()
    );
    assert_eq!(
        get_surrounding_coords((2, 1), 3, 3),
        [(1, 1), (2, 2), (2, 0)].to_vec()
    );
    assert_eq!(
        get_surrounding_coords((2, 2), 3, 3),
        [(1, 2), (2, 1)].to_vec()
    );
}
fn get_surrounding_coords(
    location: (usize, usize),
    width: usize,
    height: usize,
) -> Vec<(usize, usize)> {
    let (x, y) = location;
    [
        (x as i64 + 1, y as i64),
        (x as i64 - 1, y as i64),
        (x as i64, y as i64 + 1),
        (x as i64, y as i64 - 1),
    ]
    .into_iter()
    .filter(|(x, y)| !(*x < 0 || *x as usize >= width || *y < 0 || *y as usize >= height))
    .map(|(x, y)| (x as usize, y as usize))
    .collect()
}
fn walk(map: &Vec<Vec<Option<u32>>>, location: (usize, usize)) -> Vec<(usize, usize)> {
    let height = map.len();
    let width = map[0].len();
    let (x, y) = location;
    let current = map[y][x].unwrap();
    get_surrounding_coords(location, width, height)
        .into_iter()
        .filter(|(x, y)| match map[*y][*x] {
            Some(val) => val == current + 1,
            None => false,
        })
        .collect()
}
#[test]
fn test_find_trails() {
    let map = read_vec_vec_op_u32("../data/input_10b.txt");
    assert_eq!(find_trails(&map, false), 2);

    let map = read_vec_vec_op_u32("../data/input_10a.txt");
    assert_eq!(find_trails(&map, false), 3);
    let map = read_vec_vec_op_u32("../data/input_10c.txt");
    assert_eq!(find_trails(&map, false), 4);

    let map = read_vec_vec_op_u32("../data/input_10d.txt");
    assert_eq!(find_trails(&map, false), 36);

    let map = read_vec_vec_op_u32("../data/input_10f.txt");
    assert_eq!(find_trails(&map, true), 3);

    let map = read_vec_vec_op_u32("../data/input_10g.txt");
    assert_eq!(find_trails(&map, true), 13);

    let map = read_vec_vec_op_u32("../data/input_10h.txt");
    assert_eq!(find_trails(&map, true), 227);

    let map = read_vec_vec_op_u32("../data/input_10e.txt");
    assert_eq!(find_trails(&map, false), 782);

    let map = read_vec_vec_op_u32("../data/input_10d.txt");
    assert_eq!(find_trails(&map, true), 81);

    let map = read_vec_vec_op_u32("../data/input_10e.txt");
    assert_eq!(find_trails(&map, false), 782);

    assert_eq!(find_trails(&map, true), 1694);
}
fn find_trails(map: &Vec<Vec<Option<u32>>>, unique_path: bool) -> u64 {
    find_entry_ponts(map)
        .into_iter()
        .map(|location| {
            let mut sub_trails = 0;
            let mut next_coords = walk(&map, location);
            let mut trail_ends: std::collections::HashMap<(usize, usize), bool> =
                std::collections::HashMap::new();
            while next_coords.len() > 0 {
                next_coords = next_coords
                    .iter()
                    .map(|location| {
                        let test = walk(&map, *location);
                        test
                    })
                    .flatten()
                    .filter(|(x, y)| {
                        if map[*y][*x].unwrap() == 9 {
                            trail_ends.insert((*x, *y), true);
                            sub_trails += 1;
                            false
                        } else {
                            true
                        }
                    })
                    .collect();
            }
            if !unique_path {
                trail_ends.keys().count() as u64
            } else {
                sub_trails
            }
        })
        .sum()
}
