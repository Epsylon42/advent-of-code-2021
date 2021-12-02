use std::io::{stdin, BufRead, BufReader};

fn main() {
    let reader = BufReader::new(stdin());

    let mut increased_count = 0;
    let mut prev_height = None;
    for line in reader.lines() {
        let height: u16 = line.unwrap().parse().unwrap();
        match prev_height {
            Some(prev_height) if prev_height < height => increased_count += 1,
            _ => {}
        }

        prev_height = Some(height);
    }

    println!("{increased_count}");
}
