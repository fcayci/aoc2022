use std::fs;
use std::collections::HashSet;

#[derive(Debug)]
struct SlidingWindow {
    window: Vec<char>,
    idx: usize,
    len: usize,
}

impl SlidingWindow {
    fn push(&mut self, c: char) {
        self.idx += 1;
        if self.idx > self.len - 1 {
            self.idx = 0;
        }
        self.window[self.idx] = c;
        // println!("Pushing: {:?}", &self);
    }

    fn check_start(&self) -> bool
    {
        let mut set = HashSet::new();
        self.window.iter().all(|x| set.insert(x))
    }
}

fn main() {
    let code = fs::read_to_string("06.txt").unwrap();

    let mut sw = SlidingWindow{
        window: vec!['d'; 4],
        idx: 0,
        len: 4,
    };

    for (i, c) in code.chars().enumerate() {
        sw.push(c);
        if sw.check_start() {
            println!("Part1 marker: {}", i+1);
            break;
        }
    }

    let mut sw = SlidingWindow{
        window: vec!['d'; 14],
        idx: 0,
        len: 14,
    };

    for (i, c) in code.chars().enumerate() {
        sw.push(c);
        if sw.check_start() {
            println!("Part2 marker: {}", i+1);
            break;
        }
    }
}
