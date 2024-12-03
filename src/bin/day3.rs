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
    let instructions: Vec<String> = re
        .find_iter(input)
        .map(|mat| mat.as_str().to_string())
        .collect();

    let mut enabled = true;

    let mut sum = 0;
    for instruction in instructions {
        if enabled {
            match instruction.as_str() {
                "don't()" => enabled = false,
                "do()" => {}
                _ => {
                    if let Some((s1, s2)) = instruction
                        .trim_start_matches("mul(")
                        .trim_end_matches(")")
                        .split_once(",")
                    {
                        sum += s1.parse::<i32>().unwrap() * s2.parse::<i32>().unwrap();
                    }
                }
            }
        } else {
            if instruction == "do()" {
                enabled = true;
            }
        }
    }

    sum
}

#[test]
fn test_get_line_sum() {
    let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    let re = Regex::new(r"mul\(\d+,\d+\)").unwrap();
    assert_eq!(get_line_sum(&input, &re), 161);
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
