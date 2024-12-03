use regex::Regex;

fn get_line_sum(input: &str) -> i32 {
    let re = Regex::new(r"mul\(\d+,\d+\)").unwrap();
    re.find_iter(input)
        .filter_map(|mat| {
            mat.as_str()
                .to_string()
                .trim_start_matches("mul(")
                .trim_end_matches(")")
                .split_once(",")
                .map(|(s1, s2)| s1.parse::<i32>().unwrap() * s2.parse::<i32>().unwrap())
        })
        .sum::<i32>()
}

#[test]
fn test_get_line_sum() {
    let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    assert_eq!(get_line_sum(input), 161);
}

fn day3a() -> i32 {
    include_str!("../../input/day3.txt")
        .lines()
        .map(get_line_sum)
        .sum()
}

fn main() {
    let sum = day3a();
    print!("day 1a: {}", sum)
}
