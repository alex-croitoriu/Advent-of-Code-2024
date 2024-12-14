use std::fs::File;
use std::io::{BufRead, BufReader};

const PKG_NAME: &str = env!("CARGO_PKG_NAME");

pub fn concatenate(mut x: i128, y: i128) -> i128 {
    let digits = y.to_string().len();
    for _ in 0..digits {
        x *= 10;
    }
    return x + y;
}

pub fn can_reach_target(target: i128, k: usize, numbers: &Vec<i128>, computed: i128) -> bool {
    let mut ans = false;
    if k == numbers.len() {
        if computed == target {
            return true;
        }
    }
    else if computed < target {
        ans |= can_reach_target(target, k + 1, numbers, computed + numbers[k]);
        ans |= can_reach_target(target, k + 1, numbers, computed * numbers[k]);
        ans |= can_reach_target(target, k + 1, numbers, concatenate(computed, numbers[k]));
    }
    return ans;
}

pub fn solve() {
    let mut ans = 0;

    if let Ok(file) = File::open(format!("inputs/{PKG_NAME}.txt")) {
        let lines: Vec<String> = BufReader::new(file).lines().map(|l| l.unwrap()).collect();
        
        for line in lines {
            let mut it = line.split(':').map(|s| s.trim());
            let target = it.next().unwrap().parse::<i128>().unwrap();
            let numbers: Vec<i128> = it.next().unwrap().split_whitespace().map(|s| s.parse::<i128>().unwrap()).collect();
            
            if can_reach_target(target, 0, &numbers, 0) {
                ans += target;
            }
        }
    }

    println!("{ans}");
}
