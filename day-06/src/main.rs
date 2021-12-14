const MAX_TIMER: usize = 8;
const RESET_TIMER: usize = 6;

type Timers = [u64; MAX_TIMER + 1];

fn task(mut timers: Timers, steps: usize) -> u64 {
    for _ in 0..steps {
        let zero_timers = timers[0];

        timers.copy_within(1.., 0);

        timers[RESET_TIMER] += zero_timers;
        timers[MAX_TIMER] = zero_timers;
    }

    timers.into_iter().sum()
}

fn main() {
    aoclib::AocTask::read_full(|s| {
        let mut timers = Timers::default();
        for s in s.trim().split(',') {
            timers[s.parse::<usize>().unwrap()] += 1;
        }

        timers
    })
    .task1(|input| task(input, 80))
    .task2(|input| task(input, 256))
    .run_display();
}
