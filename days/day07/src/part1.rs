use std::fs::File;
use std::io::{BufRead, BufReader};

const PKG_NAME: &str = env!("CARGO_PKG_NAME");

pub fn can_reach_target(target: i64, k: usize, numbers: &Vec<i64>, computed: i64) -> bool {
    let mut ans = false;
    if k == numbers.len() {
        if computed == target {
            return true;
        }
    }
    else {
        ans |= can_reach_target(target, k + 1, numbers, computed + numbers[k]);
        ans |= can_reach_target(target, k + 1, numbers, computed * numbers[k]);
    }
    return ans;
}

pub fn solve() {
    let mut ans = 0;

    if let Ok(file) = File::open(format!("inputs/{PKG_NAME}.txt")) {
        let lines: Vec<String> = BufReader::new(file).lines().map(|l| l.unwrap()).collect();
        
        for line in lines {
            let mut it = line.split(':').map(|s| s.trim());
            let target = it.next().unwrap().parse::<i64>().unwrap();
            let numbers: Vec<i64> = it.next().unwrap().split_whitespace().map(|s| s.parse::<i64>().unwrap()).collect();
            
            if can_reach_target(target, 0, &numbers, 0) {
                ans += target;
            }
        }
    }

    println!("{ans}");
}
