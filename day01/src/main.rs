use std::fs;

fn main() {
    let mut total_cals: Vec<u32> = fs::read_to_string("01.txt").unwrap()
        .split("\n\n")
        .map(|cals| cals.lines()
            .map(|c| c.parse::<u32>().unwrap())
            .sum())
        .collect();

    total_cals.sort();
    total_cals.reverse();

    println!("part1: {}", total_cals[0]);
    println!("part2: {:?}", &total_cals[..3].iter().sum::<u32>());
}
