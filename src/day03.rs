use std::collections::HashSet;

pub(crate) fn first(input: &str) -> i64 {
    let direction = (3, 1);
    let (x, y, trees) = parse_tree_locations(input);

    get_trees_for_slope(direction, &trees, x, y)
}


pub(crate) fn second(input: &str) -> i64 {
    let directions = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2), ];
    let (x, y, trees) = parse_tree_locations(input);

    directions.iter()
        .map(|dir| get_trees_for_slope(*dir, &trees, x, y))
        .fold(1, |acc, val| acc * val)
}

fn parse_tree_locations(input: &str) -> (i64, i64, HashSet<(i64, i64)>) {
    (
        input.split('\n').next().unwrap().len() as i64,
        input.split('\n').count() as i64,
        input.split('\n')
            .enumerate()
            .flat_map(|(y, line)|
                line.chars()
                    .enumerate()
                    .filter(|(_, char)| *char == '#')
                    .map(move |(x, _)| (x as i64, y as i64))
            )
            .collect()
    )
}

fn get_trees_for_slope(direction: (i64, i64), trees: &HashSet<(i64, i64)>, max_x: i64, max_y: i64) -> i64 {
    let mut pos = (0, 0);
    let mut trees_encountered = 0;

    for _ in 0..max_y {
        // println!("{}:{} -> {}", pos.0, pos.1, trees.contains(&pos));
        if trees.contains(&pos) {
            trees_encountered += 1;
        }
        pos.0 += direction.0;
        pos.0 %= max_x;
        pos.1 += direction.1;
    }

    trees_encountered
}

#[cfg(test)]
mod test {
    use super::{first, parse_tree_locations, get_trees_for_slope};

    const INPUT: &str =
        "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";

    #[test]
    fn first_1() {
        assert_eq!(first(INPUT), 7)
    }

    #[test]
    fn get_trees_for_slope_1_1() {
        let (x, y, trees) = parse_tree_locations(INPUT);
        assert_eq!(get_trees_for_slope((1, 1), &trees, x, y), 2)
    }

    #[test]
    fn get_trees_for_slope_3_1() {
        let (x, y, trees) = parse_tree_locations(INPUT);
        assert_eq!(get_trees_for_slope((3, 1), &trees, x, y), 7)
    }

    #[test]
    fn get_trees_for_slope_5_1() {
        let (x, y, trees) = parse_tree_locations(INPUT);
        assert_eq!(get_trees_for_slope((5, 1), &trees, x, y), 3)
    }

    #[test]
    fn get_trees_for_slope_7_1() {
        let (x, y, trees) = parse_tree_locations(INPUT);
        assert_eq!(get_trees_for_slope((7, 1), &trees, x, y), 4)
    }

    #[test]
    fn get_trees_for_slope_1_2() {
        let (x, y, trees) = parse_tree_locations(INPUT);
        assert_eq!(get_trees_for_slope((1, 2), &trees, x, y), 2)
    }
}
