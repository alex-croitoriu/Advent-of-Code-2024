use std::fs::File;
use std::io::{BufRead, BufReader};

const PKG_NAME: &str = env!("CARGO_PKG_NAME");

pub fn solve() {
    let mut ans = 0;

    if let Ok(file) = File::open(format!("inputs/{PKG_NAME}.txt")) {
        let lines: Vec<String> = BufReader::new(file).lines().map(|l| l.unwrap()).collect();
        
        let mut occupied = vec![-1; 1000000];
        let (mut pos, mut index) = (0, 0);

        for line in lines {
            for (i, ch) in line.chars().enumerate() {
                let length = (ch.to_digit(10).unwrap()) as usize;
                if i % 2 == 0 {
                    for j in pos..pos + length {
                        occupied[j] = index;
                    }
                    index += 1;
                }
                pos += length;
            }

            let mut first_available = 0;

            for i in (0..pos).rev() {
                if occupied[i] != -1 && i > first_available {
                    while occupied[first_available] != -1 && i > first_available {
                        first_available += 1;
                    }

                    occupied[first_available] = occupied[i];
                    occupied[i] = -1;
                    first_available += 1;
                }
            }

            for i in 0..pos {
                if occupied[i] != -1 {
                    ans += i as i128 * occupied[i];
                }
            }
        }
    }

    println!("{ans}");
}
