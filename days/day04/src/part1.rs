use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;

const PKG_NAME: &str = env!("CARGO_PKG_NAME");

pub fn matches(s: &String) -> i32 {
    let mut ans = 0;
    let re = vec![Regex::new(r"XMAS").unwrap(), Regex::new(r"SAMX").unwrap()];

    for r in re {
        ans += r.captures_iter(s.as_str()).collect::<Vec<_>>().len() as i32;
    }

    ans
}

pub fn solve() {
    let mut ans = 0;

    if let Ok(file) = File::open(format!("inputs/{PKG_NAME}.txt")) {
        let lines: Vec<String> = BufReader::new(file).lines().map(|l| l.unwrap()).collect();

        let mut table: Vec<Vec<char>> = Vec::new();

        let n = lines.len();
        let m = lines[0].len();
        
        for line in lines {
            table.push(line.chars().collect::<Vec<char>>());
        }

        for line in &table {
            ans += matches(&line.iter().collect::<String>());
        }

        for i in 0..m {
            let mut column: String = String::from("");

            for j in 0..n {
                column.push(table[j][i]);
            }

            ans += matches(&column);
        }

        for i in 0..m {
            let (mut x, mut y) = (0, i);
            let mut diagonal: String = String::from("");

            while x < n && y < m {
                diagonal.push(table[x][y]);
                x += 1;
                y += 1;
            }

            ans += matches(&diagonal);
        }

        for i in 1..n {
            let (mut x, mut y) = (i, 0);
            let mut diagonal: String = String::from("");

            while x < n && y < m {
                diagonal.push(table[x][y]);
                x += 1;
                y += 1;
            }

            ans += matches(&diagonal); 
        }

        for i in 0..m {
            let (mut x, mut y) = (0, i as isize);
            let mut diagonal: String = String::from("");

            while x < n && y >= 0 {
                diagonal.push(table[x][y as usize]);
                x += 1;
                y -= 1;
            }

            ans += matches(&diagonal);
        }

        for i in 1..n {
            let (mut x, mut y) = (i, (m - 1) as isize);
            let mut diagonal: String = String::from("");

            while x < n && y >= 0 {
                diagonal.push(table[x][y as usize]);
                x += 1;
                y -= 1;
            }

            ans += matches(&diagonal);
        }
    }

    println!("{ans}");
}
