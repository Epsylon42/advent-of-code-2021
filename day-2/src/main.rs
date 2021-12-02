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

fn task1(movement: Vec<Movement>) -> i32 {
    let mut pos = (0, 0);

    for movement in movement {
        match movement {
            Movement::Horizontal(a) => pos.0 += a as i32,
            Movement::Vertical(a) => pos.1 += a as i32,
        }
    }

    pos.0 * pos.1
}

fn task2(movement: Vec<Movement>) -> i32 {
    let mut pos = (0, 0);
    let mut aim = 0;

    for movement in movement {
        match movement {
            Movement::Horizontal(a) => {
                pos.0 += a as i32;
                pos.1 += a as i32 * aim;
            }
            Movement::Vertical(a) => aim += a as i32,
        }
    }

    pos.0 * pos.1
}

fn main() {
    aoclib::AocTask::read_lines(parse_movement)
        .task1(task1)
        .task2(task2)
        .run_display();
}
