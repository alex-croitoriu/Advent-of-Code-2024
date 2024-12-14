use std::fs::File;
use std::io::{BufRead, BufReader};

const PKG_NAME: &str = env!("CARGO_PKG_NAME");

pub fn is_valid(v: Vec<Vec<char>>) -> bool {
    ((v[0][0] == 'M' && v[2][2] == 'S') || (v[0][0] == 'S' && v[2][2] == 'M')) &&
    ((v[2][0] == 'M' && v[0][2] == 'S') || (v[2][0] == 'S' && v[0][2] == 'M')) && 
    v[1][1] == 'A'
}

pub fn solve() {
    let mut ans = 0;

    if let Ok(file) = File::open(format!("inputs/{PKG_NAME}.txt")) {
        let lines: Vec<String> = BufReader::new(file).lines().map(|l| l.unwrap()).collect();

        let mut table: Vec<Vec<char>> = Vec::new();

        let (n, m) = (lines.len(), lines[0].len());
        
        for line in lines {
            table.push(line.chars().collect::<Vec<char>>());
        }

        for i in 0..n - 2 {
            for j in 0..m - 2 {
                let mut v: Vec<Vec<char>> = Vec::new();
                
                for k in 0..3 {
                    v.push(vec![
                        table[i + k][j],
                        table[i + k][j + 1],
                        table[i + k][j + 2]
                    ]);
                }

                if is_valid(v) {
                    ans += 1;
                }
            }
        }
    }

    println!("{ans}");
}
