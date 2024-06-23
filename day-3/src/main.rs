use std::fs;

fn main() {
    part_1();
}

fn part_1() {
    let input = fs::read_to_string("./day-3/input.txt").unwrap();
    let mut history = vec![(0, 0)];

    for (i, c) in input.chars().enumerate() {
        let (x, y) = history[i];
        let (new_x, new_y) = match c {
            '^' => (x, y + 1),
            'v' => (x, y - 1),
            '>' => (x + 1, y),
            '<' => (x - 1, y),
            _ => (x, y),
        };

        history.push((new_x, new_y));
    }

    let unique_houses = history
        .iter()
        .collect::<std::collections::HashSet<_>>()
        .len();

    println!("Part 1 result: {} unique houses", unique_houses);
}
