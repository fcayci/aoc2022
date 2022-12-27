use std::fs;

fn visible_trees_linear_scan(v: &Vec<Vec<u8>>) -> usize {
    let mut ones = v.clone();

    // zero out the array
    for r in 0..ones.len() {
        for c in 0..ones[r].len() {
            ones[r][c] = 0;
        }
    }

    // right to left
    for r in 0..v.len() {
        let mut max_height = 0;
        for c in 0..v[r].len() {
            if v[r][c] > max_height || c == 0 {
                max_height = v[r][c];
                ones[r][c] = 1;
            }
        }
    }

    // left to right
    for r in 0..v.len() {
        let mut max_height = 0;
        for c in (0..v[r].len()).rev() {
            if v[r][c] > max_height || c == v[r].len()-1 {
                max_height = v[r][c];
                ones[r][c] = 1;
            }
        }
    }

    // top to bottom
    // will not work if not square
    for c in 0..v.len() {
        let mut max_height = 0;
        for r in 0..v[c].len() {
            if v[r][c] > max_height || r == 0 { 
                max_height = v[r][c];
                ones[r][c] = 1;
            }
        }
    }

    // bottom to top
    // will not work if not square
    for c in 0..v.len() {
        let mut max_height = 0;
        for r in (0..v[c].len()).rev() {
            if v[r][c] > max_height || r == v[c].len()-1 { 
                max_height = v[r][c];
                ones[r][c] = 1;
            }
        }
    }

    // sum all elements
    let mut sums: Vec<usize> = vec![0; v[0].len()];
    for r in ones {
        for (i, x) in r.into_iter().enumerate() {
            sums[i] += x as usize;
        }
    }

    sums.iter().sum::<usize>()

}


fn main() {
    let trees = fs::read_to_string("08.txt").unwrap();
    let mut v: Vec<Vec<u8>> = Vec::new();

    let split = trees.trim().split("\n");

    // construct the array
    for row in split {
        let r: Vec<u8> = row.chars().map(|s| s.to_digit(10).unwrap() as u8).collect();
        // println!("{:?}", r);
        v.push(r);
    }

    let part1 = visible_trees_linear_scan(&v);

    println!("{}", part1);
}
