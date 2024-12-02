fn is_safe(v: &Vec<i32>) -> bool {
    if v.len() < 2 {
        return true;
    }

    let increasing = v[1] > v[0];

    for i in 1..v.len() {
        let diff = v[i] - v[i - 1];
        if (increasing && (diff < 1 || diff > 3)) || (!increasing && (diff > -1 || diff < -3)) {
            return false;
        }
    }
    true
}

fn is_safer(v: &Vec<i32>) -> bool {
    if is_safe(v) {
        return true;
    }

    for i in 0..v.len() {
        let slice = v
            .into_iter()
            .enumerate()
            .filter(|&(idx, _)| idx != i)
            .map(|(_, &val)| val)
            .collect();

        if is_safe(&slice) {
            return true;
        }
    }
    false
}

fn parse_input() -> Vec<Vec<i32>> {
    include_str!("../../input/day2.txt")
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect()
        })
        .collect()
}

fn day2a(vecs: &Vec<Vec<i32>>) -> usize {
    vecs.into_iter().filter(|vec| is_safe(vec)).count()
}

fn day2b(vecs: &Vec<Vec<i32>>) -> usize {
    vecs.into_iter().filter(|vec| is_safer(vec)).count()
}

fn main() {
    let vecs = parse_input();
    let sum_a = day2a(&vecs);
    println!("day2a: {sum_a}");
    let sum_b = day2b(&vecs);
    println!("day2b: {sum_b}");
}
