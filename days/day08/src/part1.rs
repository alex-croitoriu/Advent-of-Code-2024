use std::fs::File;
use std::io::{BufRead, BufReader};

const PKG_NAME: &str = env!("CARGO_PKG_NAME");

pub fn solve() {
    let mut ans = 0;

    if let Ok(file) = File::open(format!("inputs/{PKG_NAME}.txt")) {
        let lines: Vec<String> = BufReader::new(file).lines().map(|l| l.unwrap()).collect();
        
        for line in lines {
            
        }
    }

    println!("{ans}");
}
