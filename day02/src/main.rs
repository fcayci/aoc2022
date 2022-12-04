use std::fs;

enum Shape {
    Rock = 1,
    Paper = 2,
    Scissors = 3
}

enum State {
    Loss = 0,
    Draw = 3,
    Win = 6
}

fn shape_mapping(c: &str) -> Shape {
    match c {
        "A" | "X" => Shape::Rock,
        "B" | "Y" => Shape::Paper,
        "C" | "Z" => Shape::Scissors,
        // TODO: Figure out how to error handle properly
        _ => panic!("Not expected, also fixme!!")
    }
}

fn state_mapping(c: &str) -> State {
    match c {
        "X" => State::Loss,
        "Y" => State::Draw,
        "Z" => State::Win,
        // TODO: Figure out how to error handle properly
        _ => panic!("Not expected, also fixme!!")
    }
}

fn play_round(opp: &str, me: &str) -> u32 {
    let (opp, me) = (shape_mapping(opp), shape_mapping(me));

    let st = match(&opp, &me) {
        (Shape::Rock, Shape::Rock) => State::Draw,
        (Shape::Rock, Shape::Paper) => State::Win,
        (Shape::Rock, Shape::Scissors) => State::Loss,
        (Shape::Paper, Shape::Rock) => State::Loss,
        (Shape::Paper, Shape::Paper) => State::Draw,
        (Shape::Paper, Shape::Scissors) => State::Win,
        (Shape::Scissors, Shape::Rock) => State::Win,
        (Shape::Scissors, Shape::Paper) => State::Loss,
        (Shape::Scissors, Shape::Scissors) => State::Draw,
    };

    score(me, st)

}

fn play_rev_round(opp: &str, st: &str) -> u32 {
    let (opp, st) = (shape_mapping(opp), state_mapping(st));

    let me = match(&opp, &st) {
        (Shape::Rock, State::Loss) => Shape::Scissors,
        (Shape::Rock, State::Draw) => Shape::Rock,
        (Shape::Rock, State::Win)  => Shape::Paper,
        (Shape::Paper, State::Loss) => Shape::Rock,
        (Shape::Paper, State::Draw) => Shape::Paper,
        (Shape::Paper, State::Win)  => Shape::Scissors,
        (Shape::Scissors, State::Loss) => Shape::Paper,
        (Shape::Scissors, State::Draw) => Shape::Scissors,
        (Shape::Scissors, State::Win)  => Shape::Rock,
    };

    score(me, st)

}

fn score(me: Shape, st: State) -> u32 {
    me as u32 + st as u32
}

fn main() {
    let file: String = fs::read_to_string("02.txt").unwrap();

    let r1: u32 = file.lines()
        .map(|r|  {
            let rd: Vec<&str> = r.split(" ").collect();
            play_round(&rd[0], &rd[1])
        })
        .collect::<Vec<u32>>()
        .iter()
        .sum();

    let r2: u32 = file.lines()
        .map(|r|  {
            let rd: Vec<&str> = r.split(" ").collect();
            play_rev_round(&rd[0], &rd[1])
        })
        .collect::<Vec<u32>>()
        .iter()
        .sum();


    println!("part1: {}", r1);
    println!("part2: {}", r2);
}
