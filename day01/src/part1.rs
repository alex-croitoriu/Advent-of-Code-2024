use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solve() {
    let mut ans = 0;

    if let Ok(file) = File::open("day01/input.txt") {
        let lines: Vec<String> = BufReader::new(file).lines().map(|l| l.unwrap()).collect();
        
        let mut a: Vec<i32> = Vec::new();
        let mut b: Vec<i32> = Vec::new();

        for line in lines {
            let mut it = line.split_whitespace().map(|s| s.trim().parse::<i32>().unwrap_or_default());
            let (x, y) = (it.next().unwrap(), it.next().unwrap());

            a.push(x);
            b.push(y);
        }

        a.sort();
        b.sort();

        for i in 0..a.len() {
            ans += i32::abs(a[i] - b[i]);
        }
    }
    println!("{ans}");
}
