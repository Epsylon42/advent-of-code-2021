#![feature(array_chunks, portable_simd, slice_as_chunks)]
use std::simd::u8x32 as u8vec;

use bitvec::prelude::*;
use rayon::prelude::*;

const MAX_TIMER: u8 = 8;
const RESET_TIMER: u8 = 6;

fn count_zeros(mask: std::simd::mask8x32) -> u64 {
    mask.select(u8vec::splat(1), u8vec::splat(0)).horizontal_sum() as u64
}

fn perform_iteration(timers: &mut BitVec<Msb0, u8>) {
    let decrement_zeros = if timers.len() != timers.as_mut_raw_slice().len() * 8 {
        let len = timers.as_mut_raw_slice().len();
        timers.as_mut_raw_slice()[len - 1] &= 0b11110000;
        true
    } else {
        false
    };

    let (chunks, rest) = timers.as_mut_raw_slice().as_chunks_mut::<32>();

    let num_zeros = std::sync::atomic::AtomicU64::new(0);
    chunks.par_iter_mut()
        .for_each(|chunk| {
            let mut vec = u8vec::from_array(*chunk);

            for offset in [0, 4] {
                let mask = (vec & u8vec::splat(0b1111 << offset)).lanes_eq(u8vec::splat(0));
                num_zeros.fetch_add(count_zeros(mask), std::sync::atomic::Ordering::Relaxed);

                vec = mask.select(vec | u8vec::splat(RESET_TIMER << offset), vec - u8vec::splat(1 << offset));
            }

            *chunk = vec.into();
        });

    let mut num_zeros = num_zeros.into_inner();
    for x in rest {
        for offset in [0, 4] {
            if *x & (0b1111 << offset) == 0 {
                num_zeros += 1;
                *x |= RESET_TIMER << offset;
            } else {
                *x -= 1 << offset;
            }
        }
    }

    if decrement_zeros {
        num_zeros -= 1;
    }
    timers.extend(std::iter::repeat(MAX_TIMER | (MAX_TIMER << 4)).take(num_zeros as usize / 2));
    if num_zeros % 2 != 0 {
        timers.resize(timers.len() + 4, true);
        let range = timers.len() - 4 ..;
        timers[range].store(MAX_TIMER);
    }
}

fn task_simd(timers_in: Vec<u8>, steps: usize) -> usize {
    let mut timers = BitVec::<Msb0, u8>::new();
    for timer in timers_in {
        timers.resize(timers.len() + 4, false);

        let range = timers.len() - 4 ..;
        timers[range].store(timer);
    }

    let thread = std::thread::spawn(move || {
        let timers = timers;

        timers.as_raw_slice()
            .into_par_iter()
            .map(|timers| {
                let mut timers = BitVec::<Msb0, u8>::from_slice(&[*timers]).unwrap();

                for step in 0..steps {
                    eprintln!("{step}");
                    perform_iteration(&mut timers);
                }
                timers.len() / 4
            })
            .sum::<usize>()
    });

    thread.join().unwrap()
}

fn read_input() -> Vec<u8> {
    let s = std::fs::read_to_string("data").unwrap();

    let mut timers = Vec::<u8>::new();
    for s in s.trim().split(',') {
        timers.push(s.parse().unwrap());
    }

    timers
}

fn main() {
    println!("{}", task_simd(read_input(), 256));
}

//fn task_simd1(mut timers: Vec<u8>, steps: usize) -> u64 {
    //let progress = indicatif::ProgressBar::new(steps as u64);
    //for _ in 0..steps {
        //let (chunks, rest) = timers.as_chunks_mut::<32>();

        //let mut num_zeros = 0;
        //for chunk in chunks {
            //let mut vec = u8vec::from_array(*chunk);
            //let mask = vec.lanes_eq(u8vec::splat(0));
            //num_zeros += mask.select(u8vec::splat(1), u8vec::splat(0)).horizontal_sum() as u64;
            //vec = mask.select(u8vec::splat(RESET_TIMER), vec - u8vec::splat(1));
            //*chunk = vec.into();
        //}
        //for x in rest {
            //if *x == 0 {
                //num_zeros += 1;
                //*x = RESET_TIMER;
            //} else {
                //*x -= 1;
            //}
        //}

        //timers.extend((0..num_zeros).map(|_| MAX_TIMER));
        //progress.inc(1);
    //}
    //progress.finish();

    //timers.len() as u64
//}
