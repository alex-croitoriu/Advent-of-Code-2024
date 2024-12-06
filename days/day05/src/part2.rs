use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;
use std::cmp::Ordering;

const PKG_NAME: &str = env!("CARGO_PKG_NAME");

pub fn solve() {
    let mut ans = 0;

    if let Ok(file) = File::open(format!("inputs/{PKG_NAME}.txt")) {
        let lines: Vec<String> = BufReader::new(file).lines().map(|l| l.unwrap()).collect();
        let mut it = lines.iter();

        let mut rules: HashMap<i32, Vec<i32>> = HashMap::new();

        while let Some(line) = it.next() {            
            if line.is_empty() {
                break;
            }

            let mut it = line.split('|').map(|s| s.parse::<i32>().unwrap());
            let (x, y) = (it.next().unwrap(), it.next().unwrap());
            (*rules.entry(x).or_insert(Vec::new())).push(y);
        }

        while let Some(line) = it.next() {
            let mut update = line.split(',').map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>();
            let mut sorted = true;

            update.sort_by(|x, y| {
                if let Some(v) = rules.get(x) {
                    for z in v {
                        if z == y {
                            sorted = false;
                            return Ordering::Less;
                        }
                    }
                }
                Ordering::Equal
            });

            if !sorted {
                ans += update[update.len() / 2];
            }
        }
    }

    println!("{ans}");
}
