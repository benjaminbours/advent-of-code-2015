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

fn detect_action(line: &str) -> Action {
    if line.contains(Action::TurnOn.as_str()) {
        Action::TurnOn
    } else if line.contains(Action::TurnOff.as_str()) {
        Action::TurnOff
    } else {
        Action::Toggle
    }
}

fn update_grid(
    grid: &mut [[i32; 1000]; 1000],
    action: Action,
    start_coordinates: [i32; 2],
    end_coordinates: [i32; 2],
) {
    // Define a closure that determines the new value based on the action
    let update_value = |current_value: i32| -> i32 {
        match action {
            Action::TurnOn => 1,
            Action::TurnOff => 0,
            Action::Toggle => {
                if current_value == 1 {
                    0
                } else {
                    1
                }
            }
        }
    };

    // Single loop structure to apply the action
    for x in start_coordinates[0]..=end_coordinates[0] {
        for y in start_coordinates[1]..=end_coordinates[1] {
            let current_value = grid[x as usize][y as usize];
            grid[x as usize][y as usize] = update_value(current_value);
        }
    }
}

fn part_1() {
    let input = fs::read_to_string("./day-6/input.txt").unwrap();
    let mut grid = [[0; 1000]; 1000];

    for (_i, line) in input.lines().enumerate() {
        let action = detect_action(line);
        let [start_str_coordinates, _, end_str_coordinates]: [&str; 3] = line
            .strip_prefix(format!("{} ", action.as_str()).as_str())
            .unwrap()
            .split(' ')
            .collect::<Vec<_>>()
            .try_into()
            .expect("Expected exactly three elements");
        let start_coordinates = parse_coordinates(start_str_coordinates);
        let end_coordinates = parse_coordinates(end_str_coordinates);
        update_grid(&mut grid, action, start_coordinates, end_coordinates);
    }

    let result = grid.iter().flatten().fold(0, |acc, elem| acc + elem);
    println!("Part 1 result: {}", result);
}

fn part_2() {}
