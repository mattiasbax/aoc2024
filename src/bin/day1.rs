use std::collections::HashMap;

const INPUT: &str = include_str!("../../input/day1.txt");

fn day1a() -> i32 {
    let (mut vec1, mut vec2): (Vec<_>, Vec<_>) = INPUT
        .lines()
        .map(|line| line.split_once("   ").unwrap())
        .map(|(part1, part2)| (part1.parse::<i32>().unwrap(), part2.parse::<i32>().unwrap()))
        .unzip();

    vec1.sort();
    vec2.sort();

    vec1.iter().zip(&vec2).map(|(a, b)| (a - b).abs()).sum()
}

#[test]
fn test_day1a() {
    assert_eq!(day1a(), 1110981);
}

fn day1b() -> i32 {
    let (vec1, vec2): (Vec<_>, Vec<_>) = INPUT
        .lines()
        .map(|line| line.split_once("   ").unwrap())
        .map(|(part1, part2)| (part1.parse::<i32>().unwrap(), part2.parse::<i32>().unwrap()))
        .unzip();

    let mut counts = HashMap::new();

    for num in &vec2 {
        *counts.entry(num).or_insert(0) += 1;
    }

    let mut sum = 0;
    for num in &vec1 {
        if let Some(count) = counts.get(num) {
            sum += num * count;
        }
    }
    sum
}

#[test]
fn test_day1b() {
    assert_eq!(day1b(), 24869388);
}

fn main() {
    let sum = day1a();
    let similarities = day1b();
    println!("day1: {}, day2: {}", sum, similarities);
}
