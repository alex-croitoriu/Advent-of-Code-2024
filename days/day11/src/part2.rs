use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

const PKG_NAME: &str = env!("CARGO_PKG_NAME");

pub fn blink(number: i128, iteration: i32, memo: &mut HashMap<(i128, i32), i128>) -> i128 {
    let mut ans: i128 = 0;

    if iteration == 75 {
        memo.insert((number, iteration), 1);
        return 1;
    }

    if let Some(partial_ans) = memo.get(&(number, iteration)) {
        return *partial_ans;
    }

    let str = number.to_string();

    if number == 0 {
        let partial_ans = blink(1, iteration + 1, memo);

        memo.insert((number, iteration), partial_ans);
        ans += partial_ans;
    }
    else if str.len() % 2 == 0 {
        let mut partial_ans: i128 = 0;
        partial_ans += blink(str[..str.len() / 2].parse::<i128>().unwrap(), iteration + 1, memo);
        partial_ans += blink(str[str.len() / 2..].parse::<i128>().unwrap(), iteration + 1, memo);

        memo.insert((number, iteration), partial_ans);
        ans += partial_ans;
    }
    else {
        let partial_ans = blink(number * 2024, iteration + 1, memo);

        memo.insert((number, iteration), partial_ans);
        ans += partial_ans;
    }

    ans
}

pub fn solve() {
    let mut ans = 0;

    if let Ok(file) = File::open(format!("inputs/{PKG_NAME}.txt")) {
        let lines: Vec<String> = BufReader::new(file).lines().map(|l| l.unwrap()).collect();
        
        for line in lines {
            let numbers: Vec<i128> = line.split_whitespace().map(|s| s.parse::<i128>().unwrap()).collect();

            for number in numbers.iter() {
                ans += blink(*number, 0, &mut HashMap::new());
            }
        }
    }

    println!("{ans}");
}
