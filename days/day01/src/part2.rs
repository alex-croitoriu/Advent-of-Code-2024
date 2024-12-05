use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

const PKG_NAME: &str = env!("CARGO_PKG_NAME");

pub fn solve() {
    let mut ans = 0;

    if let Ok(file) = File::open(format!("inputs/{PKG_NAME}.txt")) {
        let lines: Vec<String> = BufReader::new(file).lines().map(|l| l.unwrap()).collect();
        
        let mut a: Vec<i32> = Vec::new();
        let mut b: Vec<i32> = Vec::new();

        let mut m: HashMap<i32, i32> = HashMap::new();

        for line in lines {
            let mut it = line.split_whitespace().map(|s| s.trim().parse::<i32>().unwrap());
            let (x, y) = (it.next().unwrap(), it.next().unwrap());

            a.push(x);
            b.push(y);
        }

        a.sort();
        b.sort();

        for x in b.iter() {
            *m.entry(*x).or_insert(0) += 1;
        }

        for x in a.iter() {
            ans += *x * *m.entry(*x).or_default();
        }
    }

    println!("{ans}");
}
