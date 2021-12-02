use std::io::{stdin, BufRead, BufReader};

fn main() {
    let reader = BufReader::new(stdin());

    let values: Vec<u16> = reader
        .lines()
        .map(Result::unwrap)
        .map(|line| line.parse())
        .map(Result::unwrap)
        .collect();

    let increased_count: usize = values
        .windows(4)
        .map(|win| {
            let a: u16 = win[0..3].iter().sum();
            let b: u16 = win[1..4].iter().sum();

            if b > a { 1 } else { 0 }
        })
        .sum();

    println!("{increased_count}");
}
