#![feature(array_windows)]

fn task1(values: Vec<u16>) -> usize {
    values.array_windows::<2>().filter(|[a, b]| b > a).count()
}

fn task2(values: Vec<u16>) -> usize {
    values
        .windows(4)
        .filter(|win| {
            let a: u16 = win[0..3].iter().sum();
            let b: u16 = win[1..4].iter().sum();

            b > a
        })
        .count()
}

fn main() {
    aoclib::AocTask::read_lines(|line| line.parse::<u16>().unwrap())
        .task1(task1)
        .task2(task2)
        .run_display();
}
