use std::fs;

const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
const DISALLOWED: [&str; 4] = ["ab", "cd", "pq", "xy"];

fn main() {
    part_1();
    part_2();
}

fn part_1() {
    let input = fs::read_to_string("./day-5/input.txt").unwrap();
    let result = input.lines().fold(0, |acc, elem| {
        let mut vowels = 0;
        let mut double = false;
        let mut disallowed = false;
        let mut last_char = ' ';

        for c in elem.chars() {
            if VOWELS.contains(&c) {
                vowels += 1;
            }
            if c == last_char {
                double = true;
            }
            if DISALLOWED.contains(&format!("{}{}", last_char, c).as_str()) {
                disallowed = true;
            }
            last_char = c;
        }

        if vowels >= 3 && double && !disallowed {
            acc + 1
        } else {
            acc
        }
    });

    println!("Part 1 result: {}", result);
}

fn part_2() {
    let input = fs::read_to_string("./day-5/input.txt").unwrap();
    let result = input.lines().fold(0, |acc, elem| {
        let mut has_valid_pairs = false;
        let mut history: Vec<String> = Vec::new();
        let mut last_char = ' ';
        let mut antepenultimate_char = ' ';
        let mut allowed = false;

        for c in elem.chars() {
            let pair = format!("{}{}", last_char, c);
            if history.last().unwrap_or(&String::new()) != &pair {
                if history.contains(&pair) {
                    has_valid_pairs = true;
                } else {
                    history.push(pair);
                }
            }

            if antepenultimate_char == c && last_char != c {
                allowed = true;
            }

            antepenultimate_char = last_char;
            last_char = c;
        }

        if allowed && has_valid_pairs {
            acc + 1
        } else {
            acc
        }
    });

    println!("Part 2 result: {}", result);
}
