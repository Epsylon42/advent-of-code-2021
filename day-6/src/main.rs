const MAX_TIMER: usize = 8;
const RESET_TIMER: usize = 6;

type Timers = [u64; MAX_TIMER + 1];

fn task(in_timers: Vec<u8>, steps: usize) -> u64 {
    let mut timers = Timers::default();

    for timer in in_timers {
        timers[timer as usize] += 1;
    }

    run_simulation(&mut timers, steps);

    timers.into_iter().sum()
}

fn run_simulation(timers: &mut Timers, steps: usize) {
    for _ in 0..steps {
        let zero_timers = timers[0];

        timers.copy_within(1.., 0);

        timers[RESET_TIMER] += zero_timers;
        timers[MAX_TIMER] = zero_timers;
    }
}

fn main() {
    aoclib::AocTask::read_full(|s| {
        s.trim()
            .split(',')
            .map(|s| s.parse::<u8>().unwrap())
            .collect::<Vec<u8>>()
    })
    .task1(|input| task(input, 80))
    .task2(|input| task(input, 256))
    .run_display();
}
