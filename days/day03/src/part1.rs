use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;

const PKG_NAME: &str = env!("CARGO_PKG_NAME");

pub fn solve() {
    let mut ans = 0;

    if let Ok(file) = File::open(format!("inputs/{PKG_NAME}.txt")) {
        let lines: Vec<String> = BufReader::new(file).lines().map(|l| l.unwrap()).collect();

        let re = Regex::new(r"mul\([0-9]+,[0-9]+\)").unwrap();
        
        for line in lines {
            let matches = re.find_iter(line.as_str());
            for mat in matches {
                ans += mat.as_str().split('(').nth(1).unwrap().split(')').nth(0).unwrap().split(',').fold(1, |acc, x: &str| acc * x.parse::<i32>().unwrap());
            }
        }
    }

    println!("{ans}");
}
