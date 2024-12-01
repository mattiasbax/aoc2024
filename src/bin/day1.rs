const INPUT: &str = include_str!("../../input/day1.txt");

pub fn day1a() {
    let lines = INPUT.lines();

    let mut vec1 = Vec::new();
    let mut vec2 = Vec::new();

    for line in lines {
        let (part1, part2) = line.split_once(" ").unwrap();
        let num1 = part1.parse::<i32>().unwrap();
        vec1.push(num1);
        let num2 = part2.parse::<i32>().unwrap();
        vec2.push(num2);
    }

    vec1.sort();
    vec2.sort();

    let sum_diff: i32 = vec1.iter().zip(&vec2).map(|(a, b)| (a - b).abs()).sum();
    println!("Sum of differences: {}", sum_diff);
}

fn main() {
    day1a();
}
