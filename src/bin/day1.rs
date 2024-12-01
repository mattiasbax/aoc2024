use std::collections::HashMap;

fn parse_input() -> (Vec<i32>, Vec<i32>) {
    include_str!("../../input/day1.txt")
        .lines()
        .map(|line| line.split_once("   ").unwrap())
        .map(|(part1, part2)| (part1.parse::<i32>().unwrap(), part2.parse::<i32>().unwrap()))
        .unzip()
}

fn day1a(mut vec1: Vec<i32>, mut vec2: Vec<i32>) -> u32 {
    vec1.sort_unstable();
    vec2.sort_unstable();
    vec1.into_iter().zip(vec2).map(|(a, b)| a.abs_diff(b)).sum()
}

#[test]
fn test_day1a() {
    let (vec1, vec2) = parse_input();
    assert_eq!(day1a(vec1, vec2), 1110981);
}

fn day1b(vec1: &Vec<i32>, vec2: &Vec<i32>) -> i32 {
    let counts = vec2.iter().fold(HashMap::new(), |mut acc, num| {
        *acc.entry(num).or_insert(0) += 1;
        acc
    });

    vec1.iter()
        .filter_map(|num| counts.get(num).map(|count| num * count))
        .sum()
}

#[test]
fn test_day1b() {
    let (vec1, vec2) = parse_input();
    assert_eq!(day1b(&vec1, &vec2), 24869388);
}

fn main() {
    let start = std::time::Instant::now();
    let (vec1, vec2) = parse_input();
    let start_b = std::time::Instant::now();

    let similarities = day1b(&vec1, &vec2);
    let duration_b = start_b.elapsed();

    let start_a = std::time::Instant::now();
    let sum = day1a(vec1, vec2);
    let duration_a = start_a.elapsed();
    let duration = start.elapsed();
    println!("day1a: {}, time: {:?}", sum, duration_a);
    println!("day1b: {}, time: {:?}", similarities, duration_b);
    println!("Total time: {:?}", duration);
}
