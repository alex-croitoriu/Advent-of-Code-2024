use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;

const PKG_NAME: &str = env!("CARGO_PKG_NAME");

const DIR: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

pub fn inside(x: isize, y: isize, n: usize, m: usize) -> bool {
    x >= 0 && y >= 0 && x < (n as isize) && y < (m as isize)
}

pub fn dfs(x: isize, y: isize, n: usize, m: usize, area: &mut i32, perimeter: &mut i32, map: &Vec<Vec<char>>, reached: &mut HashSet<(isize, isize)>) {
    *area += 1;
    reached.insert((x, y));

    for k in 0..4 {
        let (xn, yn) = (x + DIR[k].0, y + DIR[k].1);
        if inside(xn, yn, n, m) && map[xn as usize][yn as usize] == map[x as usize][y as usize] {
            if reached.get(&(xn, yn)).is_none() {
                dfs(xn, yn, n, m, area, perimeter, map, reached);
            }
        }
        else {
            *perimeter += 1;
        }
    }
}

pub fn solve() {
    let mut ans = 0;

    if let Ok(file) = File::open(format!("inputs/{PKG_NAME}.txt")) {
        let lines: Vec<String> = BufReader::new(file).lines().map(|l| l.unwrap()).collect();
        
        let mut map: Vec<Vec<char>> = Vec::new();
        let mut reached: HashSet<(isize, isize)> = HashSet::new();

        let (n, m) = (lines.len(), lines[0].len());

        for line in lines {
            map.push(line.chars().collect::<Vec<char>>());
        }

        for i in 0..n {
            for j in 0..m {
                if reached.get(&(i as isize, j as isize)).is_none() {
                    let (mut area, mut perimeter) = (0, 0);
                    dfs(i as isize, j as isize, n, m, &mut area, &mut perimeter, &map, &mut reached);
                    ans += area * perimeter;
                }
            }
        }
    }

    println!("{ans}");
}
