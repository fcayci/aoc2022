use std::fs;

fn contain(a: (u32, u32), b: (u32, u32)) -> bool {
    (a.0 <= b.0 && a.1 >= b.1) ||
    (a.0 >= b.0 && a.1 <= b.1)
}

fn overlap(a: (u32, u32), b: (u32, u32)) -> bool {
    (a.0 <= b.0 && a.1 >= b.0) ||
    (a.0 <= b.1 && a.1 >= b.1) ||
    (b.0 <= a.0 && b.1 >= a.0) ||
    (b.0 <= a.1 && b.1 >= a.1)
}

fn main() {

    let file = fs::read_to_string("04.txt").unwrap();

    let r1: (u32, u32) = file.lines()
        .map(|line| {
            let v = line.split([',', '-'])
                .map(|s| s.trim().parse::<u32>().unwrap())
                .collect::<Vec<u32>>();
            let a: (u32, u32) = (v[0], v[1]);
            let b: (u32, u32) = (v[2], v[3]);
            (
                contain(a, b) as u32,
                overlap(a, b) as u32,
            )
        })
        .fold((0, 0), |x, y| (x.0 + y.0, x.1 + y.1));

    println!("part1: {}, part2: {}", r1.0, r1.1);
}
