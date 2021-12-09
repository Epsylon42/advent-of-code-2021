#![feature(int_abs_diff)]

fn task1(mut submarines: Vec<u16>) -> u32 {
    submarines.sort();

    let num_positions = *submarines.last().unwrap() as usize + 1;

    let mut subs_to_the_left = 0;
    let mut subs_to_the_right = submarines.len();

    let mut fuel_requirements = submarines.iter().copied().map(u32::from).sum::<u32>();
    let mut min_fuel_requirements = fuel_requirements;

    let mut subs_iter = submarines.iter().peekable();
    for pos in 0..num_positions {
        let mut subs_at_position = 0;
        while subs_iter.peek() == Some(&&(pos as u16)) {
            subs_at_position += 1;
            subs_iter.next();
        }

        subs_to_the_left += subs_at_position;
        subs_to_the_right -= subs_at_position;

        fuel_requirements += subs_to_the_left as u32;
        fuel_requirements -= subs_to_the_right as u32;
        min_fuel_requirements = (min_fuel_requirements).min(fuel_requirements);
    }

    min_fuel_requirements
}

fn task2(submarines: Vec<u16>) -> u32 {
    let num_positions = *submarines.iter().max().unwrap() as usize + 1;

    (0..num_positions)
        .map(|pos| {
            submarines
                .iter()
                .flat_map(|&sub_pos| 1..=(sub_pos as u32).abs_diff(pos as u32))
                .sum()
        })
        .min()
        .unwrap()
}

fn main() {
    aoclib::AocTask::read_full(|s| {
        s.trim()
            .split(',')
            .map(str::parse)
            .map(Result::unwrap)
            .collect::<Vec<u16>>()
    })
    .task1(task1)
    .task2(task2)
    .run_display();
}
