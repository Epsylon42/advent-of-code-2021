use std::io::{stdin, BufRead, BufReader};

enum Movement {
    Horizontal(u16),
    Vertical(i16),
}

fn parse_movement(line: &str) -> Movement {
    let mut split = line.split_ascii_whitespace();
    let word = split.next().unwrap();
    let value = split.next().unwrap();
    assert_eq!(split.next(), None);

    match word {
        "forward" => Movement::Horizontal(value.parse().unwrap()),
        "down" => Movement::Vertical(value.parse().unwrap()),
        "up" => Movement::Vertical(-value.parse::<i16>().unwrap()),
        _ => panic!("unexpected command"),
    }
}

fn main() {
    let reader = BufReader::new(stdin());

    let mut pos = (0, 0);
    let mut aim = 0;

    for movement in reader
        .lines()
        .map(Result::unwrap)
        .map(|line| parse_movement(&line))
    {
        match movement {
            Movement::Horizontal(a) => {
                pos.0 += a as i32;
                pos.1 += a as i32 * aim;
            }
            Movement::Vertical(a) => aim += a as i32,
        }
    }

    println!("{}", pos.0 * pos.1);
}
