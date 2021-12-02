#![feature(array_windows)]

fn task1(values: Vec<u16>) {
    let increased_count = values
        .array_windows::<2>()
        .filter(|[a, b]| b > a)
        .count();

    println!("{increased_count}");
}

fn task2(values: Vec<u16>) {
    let increased_count = values
        .windows(4)
        .filter(|win| {
            let a: u16 = win[0..3].iter().sum();
            let b: u16 = win[1..4].iter().sum();

            b > a
        })
        .count();

    println!("{increased_count}");
}

fn main() {
    aoclib::AocTask::read_lines(|line| line.parse::<u16>().unwrap())
        .task1(task1)
        .task2(task2)
        .run();
}
