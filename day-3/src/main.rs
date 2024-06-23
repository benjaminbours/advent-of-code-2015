use std::fs;

fn main() {
    part_1();
    part_2();
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

fn part_2() {
    let input = fs::read_to_string("./day-3/input.txt").unwrap();
    let mut santa_history = vec![(0, 0)];
    let mut robot_santa_history = vec![(0, 0)];

    for (i, c) in input.chars().enumerate() {
        let history = if i % 2 == 0 {
            &mut santa_history
        } else {
            &mut robot_santa_history
        };

        let (x, y) = history.last().unwrap().clone();
        let (new_x, new_y) = match c {
            '^' => (x, y + 1),
            'v' => (x, y - 1),
            '>' => (x + 1, y),
            '<' => (x - 1, y),
            _ => (x, y),
        };

        history.push((new_x, new_y));
    }

    santa_history.extend(robot_santa_history);

    let unique_houses = santa_history
        .iter()
        .collect::<std::collections::HashSet<_>>()
        .len();

    println!("Part 2 result: {} unique houses", unique_houses);
}
