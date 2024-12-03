use regex::Regex;

fn get_line_sum(input: &str, re: &Regex) -> i32 {
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

fn get_line_sum_accurately(input: &str, re: &Regex) -> i32 {
    let mut enabled = true;
    re.find_iter(input).fold(0, |acc, mat| match mat.as_str() {
        "don't()" => {
            enabled = false;
            acc
        }
        "do()" => {
            enabled = true;
            acc
        }
        _ if enabled => {
            if let Some((s1, s2)) = mat
                .as_str()
                .trim_start_matches("mul(")
                .trim_end_matches(")")
                .split_once(",")
            {
                acc + s1.parse::<i32>().unwrap() * s2.parse::<i32>().unwrap()
            } else {
                acc
            }
        }
        _ => acc,
    })
}

fn day3a(input: &str) -> i32 {
    let re = Regex::new(r"mul\(\d+,\d+\)").unwrap();
    get_line_sum(&input, &re)
}

fn day3b(input: &str) -> i32 {
    let re = Regex::new(r"mul\(\d+,\d+\)|do\(\)|don't\(\)").unwrap();
    get_line_sum_accurately(&input, &re)
}

fn main() {
    let input = include_str!("../../input/day3.txt").replace("\n", "");
    let sum = day3a(&input);
    print!("day 1a: {}", sum);
    let sum = day3b(&input);
    print!("day 1b: {}", sum);
}
