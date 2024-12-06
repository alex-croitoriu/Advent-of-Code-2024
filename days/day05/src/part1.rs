use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

const PKG_NAME: &str = env!("CARGO_PKG_NAME");

pub fn solve() {
    let mut ans = 0;

    if let Ok(file) = File::open(format!("inputs/{PKG_NAME}.txt")) {
        let lines: Vec<String> = BufReader::new(file).lines().map(|l| l.unwrap()).collect();
        let mut it = lines.iter();

        let mut rules: Vec<(i32, i32)> = Vec::new();

        while let Some(line) = it.next() {            
            if line.is_empty() {
                break;
            }

            let mut it = line.split('|').map(|s| s.parse::<i32>().unwrap());
            let (x, y) = (it.next().unwrap(), it.next().unwrap());
            rules.push((x, y)); 
        }

        while let Some(line) = it.next() {
            let mut valid = true;
            let update = line.split(',').map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>();
            let mut index: HashMap<i32, i32> = HashMap::new();

            for (i, number) in update.iter().enumerate() {
                index.insert(*number, i as i32);
            }

            for (x, y) in rules.iter() {
                if let (Some(ix), Some(iy)) = (index.get(x), index.get(y)) {
                    if ix > iy {
                        valid = false;
                        break;
                    }
                }
            }

            if valid {
                ans += update[update.len() / 2];
            }
        }
    }

    println!("{ans}");
}
