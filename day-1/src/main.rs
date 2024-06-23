use std::fs;

fn main() {
    part_1();
    part_2();
}

fn part_1() {
    let input = fs::read_to_string("./day-1/input.txt").unwrap();
    let result = input.chars().fold(0, |acc, elem| match elem {
        '(' => acc + 1,
        ')' => acc - 1,
        _ => acc,
    });

    println!("Part 1 result: {}", result);
}

fn part_2() {
    let input = fs::read_to_string("./day-1/input.txt").unwrap();
    let mut floor = 0;
    let mut position = 0;

    for (i, c) in input.chars().enumerate() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => (),
        }

        if floor == -1 {
            position = i + 1;
            break;
        }
    }

    println!("Part 2 result: {}", position);
}
