fn is_safe(v: &Vec<i32>) -> bool {
    if v.len() < 2 {
        return true;
    }

    let increasing = v[1] > v[0];

    for i in 1..v.len() {
        let diff = v[i] - v[i - 1];
        if increasing {
            if diff < 1 || diff > 3 {
                return false;
            }
        } else {
            if diff > -1 || diff < -3 {
                return false;
            }
        }
    }
    return true;
}

fn day1() -> usize {
    let vecs: Vec<Vec<i32>> = include_str!("../../input/day2.txt")
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    vecs.into_iter().filter(|vec| is_safe(vec)).count()
}

fn main() {
    let sum = day1();
    println!("day1a: {sum}");
}
