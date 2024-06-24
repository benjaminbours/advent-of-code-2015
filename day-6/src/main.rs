use std::fs;

fn main() {
    part_1();
    part_2();
}

enum Action {
    TurnOn,
    TurnOff,
    Toggle,
}

impl Action {
    fn as_str(&self) -> &'static str {
        match self {
            Action::TurnOn => "turn on",
            Action::TurnOff => "turn off",
            Action::Toggle => "toggle",
        }
    }
}

fn parse_coordinates(str_coordinates: &str) -> [i32; 2] {
    str_coordinates
        .split(',')
        .map(|num_str| num_str.parse().expect("Expected a number"))
        .collect::<Vec<_>>()
        .try_into()
        .expect("Expected exactly two elements")
}

fn part_1() {
    let input = fs::read_to_string("./day-6/input.txt").unwrap();
    let mut grid = [[0; 1000]; 1000];

    for (_i, line) in input.lines().enumerate() {
        match line {
            _ if line.contains(Action::TurnOn.as_str()) => {
                // turn on
                let parts: Vec<&str> = line.split(' ').collect();
                let start_coordinates = parse_coordinates(parts[2]);
                let end_coordinates = parse_coordinates(parts[4]);

                for x in start_coordinates[0]..=end_coordinates[0] {
                    for y in start_coordinates[1]..=end_coordinates[1] {
                        grid[x as usize][y as usize] = 1;
                    }
                }
            }
            _ if line.contains(Action::TurnOff.as_str()) => {
                // turn off
                let parts: Vec<&str> = line.split(' ').collect();
                let start_coordinates = parse_coordinates(parts[2]);
                let end_coordinates = parse_coordinates(parts[4]);

                for x in start_coordinates[0]..=end_coordinates[0] {
                    for y in start_coordinates[1]..=end_coordinates[1] {
                        grid[x as usize][y as usize] = 0;
                    }
                }
            }
            _ if line.contains(Action::Toggle.as_str()) => {
                // toggle
                let parts: Vec<&str> = line.split(' ').collect();
                let start_coordinates = parse_coordinates(parts[1]);
                let end_coordinates = parse_coordinates(parts[3]);

                for x in start_coordinates[0]..=end_coordinates[0] {
                    for y in start_coordinates[1]..=end_coordinates[1] {
                        grid[x as usize][y as usize] = if grid[x as usize][y as usize] == 1 {
                            0
                        } else {
                            1
                        };
                    }
                }
            }
            _ => (),
        }
    }

    let result = grid.iter().flatten().fold(0, |acc, elem| acc + elem);
    println!("Part 1 result: {}", result);
}

fn part_2() {}
