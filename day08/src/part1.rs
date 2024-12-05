use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solve() {
    let mut ans = 0;

    if let Ok(file) = File::open("input.txt") {
        let lines: Vec<String> = BufReader::new(file).lines().map(|l| l.unwrap()).collect();
        
        for line in lines {
            
        }
    }

    println!("{ans}");
}
