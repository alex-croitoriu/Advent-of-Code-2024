use std::fs::File;
use std::io::{BufRead, BufReader};

const PKG_NAME: &str = env!("CARGO_PKG_NAME");

pub fn is_safe(a: Vec<i32>) -> bool {
    let mut increasing: bool = true;

    let diff = i32::abs(a[0] - a[1]);
    if diff < 1 || diff > 3 {
        return false
    }

    if a[0] > a[1] {
        increasing = false;
    }

    for i in 1..a.len() - 1 {
        let diff = a[i] - a[i + 1];
        if (increasing && diff > 0) || (!increasing && diff < 0) {
            return false;
        }

        let abs_diff = i32::abs(diff);
        if abs_diff < 1 || abs_diff > 3 {
            return false;
        }
    }

    return true
}
pub fn solve() {
    let mut ans = 0;

    if let Ok(file) = File::open(format!("inputs/{PKG_NAME}.txt")) {
        let lines: Vec<String> = BufReader::new(file).lines().map(|l| l.unwrap()).collect();
        
        for line in lines {
            let numbers = line.split_whitespace().map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>();

            if is_safe(numbers) {
                ans += 1
            }
        }
    }

    println!("{ans}");
}
