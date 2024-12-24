use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;

const PKG_NAME: &str = env!("CARGO_PKG_NAME");

const DIR: [(isize, isize); 4]  = [(-1, 0), (0, 1), (1, 0), (0, -1)];

pub fn inside(x: isize, y: isize, n: usize, m: usize) -> bool {
    x >= 0 && y >= 0 && x < (n as isize) && y < (m as isize)
}

pub fn solve() {
    let mut ans = 0;

    if let Ok(file) = File::open(format!("inputs/{PKG_NAME}.txt")) {
        let lines: Vec<String> = BufReader::new(file).lines().map(|l| l.unwrap()).collect();

        let mut table: Vec<Vec<char>> = Vec::new();

        let (n, m) = (lines.len(), lines[0].len());

        let mut visited: HashSet<(isize, isize)> = HashSet::new();
        let mut k = 0;

        let (mut x, mut y) = (0, 0);
        
        for line in lines {
            table.push(line.chars().collect::<Vec<char>>());
        }

        for i in 0..n {
            for j in 0..m {
                if table[i][j] == '^' {
                    (x, y) = (i as isize, j as isize);
                    break;
                }
            }
        }

        while inside(x, y, n, m) {
            visited.insert((x, y));
            let (xn, yn) = (x + DIR[k].0, y + DIR[k].1);

            if inside(xn, yn, n, m) {
                if table[xn as usize][yn as usize] == '#' {
                    k = (k + 1) % 4;
                }
            }

            x += DIR[k].0;
            y += DIR[k].1;
        }

        ans = visited.len();
    }

    println!("{ans}");
}
