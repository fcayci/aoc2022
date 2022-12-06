use std::fs;

fn form_cargo(s: String) -> Vec<Vec<char>> {
    // TODO: how to get this?
    // let clen = (crates.lines()[0].len() + 1)/3;
    let clen = 9;
    let mut cargo: Vec<Vec<char>> = vec![Vec::new(); clen];
    for line in s.lines() {
        // println!("{:?}", line);
        for (i, c) in line.chars().enumerate().skip(1).step_by(4) {
            match c as u8 {
                b'A'..=b'Z' => cargo[(i-1)/4].push(c),
                1..=9 => break,
                _ => continue,
            }
        }
    }

    // Reverse the lists
    for cr in &mut cargo {
        cr.reverse();
    }
    cargo
}

trait Move {
    fn move9000(&mut self, src: usize, dst: usize, num: usize);
    fn move9001(&mut self, src: usize, dst: usize, num: usize);
    fn peek_top(&mut self) -> String;
}

impl Move for Vec<Vec<char>> {
    fn move9000(&mut self, src: usize, dst: usize, num: usize) {
        for _ in 0..num {
            let c = match self[src].pop() {
                Some(x) => x,
                None => break,
            };

            self[dst].push(c);
        }
    }

    fn move9001(&mut self, src: usize, dst: usize, num: usize) {
        let mut v = Vec::new();
        for _ in 0..num {
            let c = match self[src].pop() {
                Some(x) => x,
                None => break,
            };

            v.push(c);
        }

        v.reverse();
        self[dst].append(&mut v);
    }

    fn peek_top(&mut self) -> String {
        let mut s = String::new();
        for c in self {
            match c.last() {
                Some(x) => s.push(*x),
                None => continue,
            }
        }

        s
    }
}

fn main() {
    let file = fs::read_to_string("05.txt").unwrap();

    let mut split = file.split("\n\n");
    let crates = split.next().unwrap();
    let mut cargo = form_cargo(crates.to_string());

    let moves: Vec<(usize, usize, usize)> = split.next().unwrap()
        .lines()
        .map(|line| { let mut s = line.split(" ");
            (
            s.nth(1).unwrap().parse::<usize>().unwrap(),
            s.nth(1).unwrap().parse::<usize>().unwrap() - 1,
            s.nth(1).unwrap().parse::<usize>().unwrap() - 1
            )
        })
        .collect();

    // println!("Before");
    // for cr in &cargo {
    //     println!("=> {:?}", cr);
    // }

    let mut cargo2 = cargo.clone();

    for m in &moves {
        cargo.move9000(m.1, m.2, m.0);
    }

    for m in &moves {
        cargo2.move9001(m.1, m.2, m.0);
    }

    // println!("After");
    // for cr in &cargo {
    //     println!("=> {:?}", cr);
    // }

    println!("Top Crates Part1: {}", cargo.peek_top());
    println!("Top Crates Part2: {}", cargo2.peek_top());
}
