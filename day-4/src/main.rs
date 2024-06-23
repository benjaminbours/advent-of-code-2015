use std::fs;

fn main() {
    part_1();
    part_2();
}

fn part_1() {
    let input = fs::read_to_string("./day-4/input.txt").unwrap();

    // (0..) is an infinite iterator that starts at 0
    let result = (0..)
        .find(|i| {
            let digest = md5::compute(format!("{}{}", input, i));
            // :x is the format specifier for hexadecimal
            format!("{:x}", digest).starts_with("00000")
        })
        .unwrap();

    println!("Part 1 result: {}", result);
}

fn part_2() {
    let input = fs::read_to_string("./day-4/input.txt").unwrap();

    // (0..) is an infinite iterator that starts at 0
    let result = (0..)
        .find(|i| {
            let digest = md5::compute(format!("{}{}", input, i));
            // :x is the format specifier for hexadecimal
            format!("{:x}", digest).starts_with("000000")
        })
        .unwrap();

    println!("Part 2 result: {}", result);
}
