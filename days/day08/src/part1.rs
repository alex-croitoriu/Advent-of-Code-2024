use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::{HashMap, HashSet};

const PKG_NAME: &str = env!("CARGO_PKG_NAME");

pub fn inside(x: isize, y: isize, n: usize, m: usize) -> bool {
    x >= 0 && y >= 0 && x < (n as isize) && y < (m as isize)
}

pub fn solve() {
    let mut ans = 0;

    if let Ok(file) = File::open(format!("inputs/{PKG_NAME}.txt")) {
        let lines: Vec<String> = BufReader::new(file).lines().map(|l| l.unwrap()).collect();
        
        let mut antennas: HashMap<char, Vec::<(usize, usize)>>= HashMap::new();
        let mut antinodes: HashSet<(isize, isize)> = HashSet::new();

        let (n, m) = (lines.len(), lines[0].len());

        for (i, line) in lines.iter().enumerate() {
            for (j, ch) in line.chars().enumerate() {
                if ch != '.' {
                    antennas.entry(ch).and_modify(|v| v.push((i, j))).or_insert(vec![(i, j)]);
                }
            }
        }

        for (_ch, v) in antennas.iter() {
            for i in 0..v.len() - 1 {
                for j in i + 1..v.len() {
                    let dx = v[i].0 - v[j].0;
                    let dy = v[i].1 - v[j].1;

                    let forwards = ((v[j].0 - dx) as isize, (v[j].1 - dy) as isize);
                    let backwards = ((v[i].0 + dx) as isize, (v[i].1 + dy) as isize);

                    if inside(forwards.0, forwards.1, n, m) {
                        antinodes.insert(forwards);
                    }
                    if inside(backwards.0, backwards.1, n, m) {
                        antinodes.insert(backwards);
                    }
                }
            }
        }
        ans = antinodes.len();
    }

    println!("{ans}");
}
