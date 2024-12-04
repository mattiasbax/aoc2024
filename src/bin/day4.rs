use grid::*;

fn day4a(table: Grid<char>, words: &str) -> i32 {
    println!("{:?}", table);
    0
}

#[test]
fn day4a_test() {
    let input = "MMMSXXMASM\nMSAMXMSMSA\nAMXSXMAAMM\nMSAMASMSMX\nXMASAMXAMM\nXXAMMXXAMA\nSMSMSASXSS\nSAXAMASAAA\nMAMMMXMMMM\nMXMXAXMASX";
    let grid: Grid<char> = Grid::from_vec(
        input.lines().flat_map(|line| line.chars()).collect(),
        input.lines().count(),
    );

    assert_eq!(18, day4a(grid, "XMAS"));
}

fn main() {
    let input = "MMMSXXMASM\nMSAMXMSMSA\nAMXSXMAAMM\nMSAMASMSMX\nXMASAMXAMM\nXXAMMXXAMA\nSMSMSASXSS\nSAXAMASAAA\nMAMMMXMMMM\nMXMXAXMASX";
    let grid: Grid<char> = Grid::from_vec(
        input.lines().flat_map(|line| line.chars()).collect(),
        input.lines().count(),
    );

    day4a(grid, "XMAS");
    println!("day 4: ")
}

// use crate::utils::*;

// const DIRS: &[C<i32>] =
//     &[C(-1, -1), C(-1, 0), C(-1, 1), C(0, -1), C(0, 1), C(1, -1), C(1, 0), C(1, 1)];

// fn get(grid: &Grid<u8, i32>, i: C<i32>) -> u8 {
//     *grid.get(i).unwrap_or(&0)
// }

// pub fn part1(input: &str) -> usize {
//     let grid: Grid<u8, i32> = input.bytes().collect();
//     grid.idx_iter()
//         .filter(|(_, &v)| v == b'X')
//         .map(|(i, _)| {
//             DIRS.iter()
//                 .filter(|&d| (1..).zip(b"MAS").all(|(c, v)| get(&grid, i + *d * c) == *v))
//                 .count()
//         })
//         .sum()
// }

// pub fn part2(input: &str) -> usize {
//     let grid: Grid<u8, i32> = input.bytes().collect();
//     grid.idx_iter()
//         .filter(|(_, &v)| v == b'A')
//         .filter(|(i, _)| {
//             let (ul, lr) = (get(&grid, i + C(-1, -1)), get(&grid, i + C(1, 1)));
//             let (ur, ll) = (get(&grid, i + C(-1, 1)), get(&grid, i + C(1, -1)));
//             matches!(&[ul, ur, ll, lr], b"MMSS" | b"MSMS" | b"SMSM" | b"SSMM")
//         })
//         .count()
// }
