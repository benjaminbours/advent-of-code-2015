use std::fs;

fn main() {
    part_1();
}

fn part_1() {
    let input = fs::read_to_string("./day-1/input.txt").unwrap();
    let result = input.chars().fold(0, |acc, elem| match elem {
        '(' => acc + 1,
        ')' => acc - 1,
        _ => acc,
    });

    println!("Result: {}", result);
}
