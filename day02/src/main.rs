use std::fs;

fn main() {
    let file: String = fs::read_to_string("02.txt").unwrap();

    let rps1: u32 = file
        .lines()
        .filter_map(|m|
            match m {
                "A X" => Some(3 + 1),
                "A Y" => Some(6 + 2),
                "A Z" => Some(0 + 3),
                "B X" => Some(0 + 1),
                "B Y" => Some(3 + 2),
                "B Z" => Some(6 + 3),
                "C X" => Some(6 + 1),
                "C Y" => Some(0 + 2),
                "C Z" => Some(3 + 3),
                _ => panic!("Should not exist")
            })
        .sum();

    let rps2: u32 = file
        .lines()
        .filter_map(|m|
            match m {
                "A X" => Some(0 + 3),
                "B X" => Some(0 + 1),
                "C X" => Some(0 + 2),
                "A Y" => Some(3 + 1),
                "B Y" => Some(3 + 2),
                "C Y" => Some(3 + 3),
                "A Z" => Some(6 + 2),
                "B Z" => Some(6 + 3),
                "C Z" => Some(6 + 1),
                _ => panic!("Should not exist"),
            })
        .sum();

    println!("part1: {}", rps1);
    println!("part2: {}", rps2);
}
