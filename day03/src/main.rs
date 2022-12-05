use std::fs;
use std::collections::HashSet;

// Let's use constants
const LOWERA: u32 = 1;
const UPPERA: u32 = 27;

// TODO: Why does not work with a?
fn score(&a: &char) -> u32 {

    let a = a as u8;
    match a {
        // Using hex, tho we can change u8
        // 0x41..=0x5a => a - 38,
        // 0x61..=0x7a => a - 96,
        //
        // Using bytes
        b'A'..=b'Z' => (a - b'A') as u32 + UPPERA,
        b'a'..=b'z' => (a - b'a') as u32 + LOWERA,
        _ => panic!("Should not happen"),
    }
}

fn main() {
    let file = fs::read_to_string("03.txt").unwrap();

    let r1: u32 = file.lines()
        .map(|c| {
            let p1 = &c[..c.len()/2];
            let p2 = &c[c.len()/2..];
            let p1: HashSet<char> = p1.chars().collect();
            let p2: HashSet<char> = p2.chars().collect();
            p1.intersection(&p2)
                .copied()
                .collect::<Vec<char>>()
                .iter()
                .map(|p| score(p))
                .sum::<u32>()
        })
        .sum();

    let mut r2 = file.lines();

    let mut v = Vec::new();
    while let (Some(p1), Some(p2), Some(p3)) = (r2.next(), r2.next(), r2.next()) {
        let p1: HashSet<char> = p1.chars().collect();
        let p2: HashSet<char> = p2.chars().collect();
        let p3: HashSet<char> = p3.chars().collect();

        let p1: HashSet<char> = p1.intersection(&p2).copied().collect();
        let p2: HashSet<char> = p1.intersection(&p3).copied().collect();
        v.push(p2.iter().map(|p| score(p)).sum::<u32>());
    }

    println!("part1: {}", r1);
    println!("part2: {}", v.iter().sum::<u32>());
}
