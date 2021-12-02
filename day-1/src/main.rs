fn task1(values: Vec<u16>) {
    let mut increased_count = 0;
    let mut prev_height = None;
    for height in values {
        match prev_height {
            Some(prev_height) if prev_height < height => increased_count += 1,
            _ => {}
        }

        prev_height = Some(height);
    }

    println!("{increased_count}");
}

fn task2(values: Vec<u16>) {
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

fn main() {
    aoclib::AocTask::read_lines(|line| line.parse::<u16>().unwrap())
        .task1(task1)
        .task2(task2)
        .run();
}
