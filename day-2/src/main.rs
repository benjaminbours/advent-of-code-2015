use std::fs;

fn main() {
    part_1();
    part_2();
}

fn part_1() {
    let input = fs::read_to_string("./day-2/input.txt").unwrap();
    let result = input.lines().fold(0, |acc, line| {
        let dimensions: Vec<i32> = line.split('x').map(|x| x.parse().unwrap()).collect();
        let l = dimensions[0];
        let w = dimensions[1];
        let h = dimensions[2];
        let area = 2 * l * w + 2 * w * h + 2 * h * l;
        let side_areas = [l * w, w * h, h * l];
        let slack = side_areas.iter().min().unwrap();
        acc + area + slack
    });

    println!("Part 1 result: {} square feet", result);
}

fn part_2() {
    let input = fs::read_to_string("./day-2/input.txt").unwrap();
    let result = input.lines().fold(0, |acc, line| {
        let dimensions: Vec<i32> = line.split('x').map(|x| x.parse().unwrap()).collect();
        let l = dimensions[0];
        let w = dimensions[1];
        let h = dimensions[2];
        let mut sorted = dimensions.clone();
        sorted.sort();
        let ribbon = 2 * sorted[0] + 2 * sorted[1];
        let bow = l * w * h;
        acc + ribbon + bow
    });

    println!("Part 2 result: {} feet of ribbon", result);
}
