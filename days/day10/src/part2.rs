use std::fs::File;
use std::io::{BufRead, BufReader};

const PKG_NAME: &str = env!("CARGO_PKG_NAME");

const DIR: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

pub fn inside(x: isize, y: isize, n: usize, m: usize) -> bool {
    x >= 0 && y >= 0 && x < (n as isize) && y < (m as isize)
}

pub fn dfs(x: isize, y: isize, n: usize, m: usize, ans: &mut i32, map: &Vec<Vec<i32>>) {
    if map[x as usize][y as usize] == 9 {
        *ans += 1;
        return;
    }
    for k in 0..4 {
        let (xn, yn) = (x + DIR[k].0, y + DIR[k].1);
        if inside(xn, yn, n, m) && map[xn as usize][yn as usize] == map[x as usize][y as usize] + 1 {
            dfs(xn, yn, n, m, ans, map);
        }
    }
}

pub fn solve() {
    let mut ans = 0;

    if let Ok(file) = File::open(format!("inputs/{PKG_NAME}.txt")) {
        let lines: Vec<String> = BufReader::new(file).lines().map(|l| l.unwrap()).collect();

        let mut map: Vec<Vec<i32>> = Vec::new();

        let (n, m) = (lines.len(), lines[0].len());

        for line in lines {
            map.push(line.chars().map(|ch| ch.to_digit(10).unwrap() as i32).collect::<Vec<i32>>());
        }

        for i in 0..n {
            for j in 0..m {
                if map[i][j] == 0 {
                    dfs(i as isize, j as isize, n, m, &mut ans, &map);
                }
            }
        }
    }

    println!("{ans}");
}
