#![allow(clippy::unusual_byte_groupings, clippy::clone_on_copy)]

use std::collections::{HashMap, HashSet};

const fn segment_mask(num: u8) -> Segment {
    1 << num
}

type Digit = u8;
type Segment = u8;
type SegmentSet = u8;

const NUM_SEGMENTS: usize = 7;

type SegmentMapping = [SegmentSet; NUM_SEGMENTS];

const DIGITS: [Digit; 10] = [
    0b0_1110111,
    0b0_0100100,
    0b0_1011101,
    0b0_1101101,
    0b0_0101110,
    0b0_1101011,
    0b0_1111011,
    0b0_0100101,
    0b0_1111111,
    0b0_1101111,
];

fn construct_digit(code: &[u8]) -> Digit {
    code.iter()
        .map(|x| segment_mask(x - b'a'))
        .fold(0, |acc, x| acc | x)
}

fn digit_to_index(input_digit: Digit) -> usize {
    DIGITS
        .iter()
        .position(|&digit| input_digit == digit)
        .unwrap()
}

struct Task {
    patterns: Vec<Digit>,
    output: Vec<Digit>,
}

impl Task {
    fn decode_output(self) -> Vec<Digit> {
        let mut segment_mapping: SegmentMapping = [0b0_1111111; NUM_SEGMENTS];
        let mut possible_pattern_digits: Vec<HashSet<usize>> =
            vec![HashSet::from_iter(0..10); self.patterns.len()];

        for (pat, possible_digits) in self.patterns.iter().zip(possible_pattern_digits.iter_mut()) {
            possible_digits.retain(|&i| DIGITS[i].count_ones() == pat.count_ones());
        }

        self.reduce_segments(&mut segment_mapping, &possible_pattern_digits);

        'outer: while let Some(ambiguous_pattern) =
            possible_pattern_digits.iter().position(|set| set.len() > 1)
        {
            for &fixed_digit in &possible_pattern_digits[ambiguous_pattern] {
                let mut scry_segments = segment_mapping.clone();
                let mut scry_ppd = possible_pattern_digits.clone();
                for possible_digits in &mut scry_ppd {
                    possible_digits.remove(&fixed_digit);
                }
                scry_ppd[ambiguous_pattern].clear();
                scry_ppd[ambiguous_pattern].insert(fixed_digit);

                self.reduce_segments(&mut scry_segments, &scry_ppd);
                self.reduce_possible_digits(&scry_segments, &mut scry_ppd);
                if scry_ppd.iter().all(|set| !set.is_empty()) {
                    segment_mapping = scry_segments;
                    possible_pattern_digits = scry_ppd;
                    continue 'outer;
                }
            }
        }

        assert!(segment_mapping
            .iter()
            .all(|mapping| mapping.count_ones() == 1));

        self.output
            .into_iter()
            .map(|pat| {
                segment_mapping
                    .iter()
                    .enumerate()
                    .filter(|&(segment_idx, _)| contains_segment(pat, segment_idx))
                    .fold(0, |acc, (_, decoded_segment)| (acc | decoded_segment))
            })
            .collect()
    }

    fn reduce_segments(
        &self,
        segment_mapping: &mut SegmentMapping,
        possible_pattern_digits: &[HashSet<usize>],
    ) {
        for (pat, possible_digits) in self.patterns.iter().zip(possible_pattern_digits.iter()) {
            for (segment_idx, mapping) in segment_mapping.iter_mut().enumerate() {
                if contains_segment(*pat, segment_idx) {
                    *mapping &= possible_digits
                        .iter()
                        .map(|&i| DIGITS[i])
                        .fold(0, |acc, x| acc | x);
                }
            }
        }

        loop {
            let mut modified = false;
            for (possible_mappings, count) in count_duplicates(segment_mapping.as_slice()) {
                if possible_mappings.count_ones() as usize != count {
                    continue;
                }
                for mapping in segment_mapping.iter_mut() {
                    if *mapping != possible_mappings {
                        let refined_mapping = *mapping & !possible_mappings;
                        if *mapping != refined_mapping {
                            modified = true;
                        }
                        *mapping = refined_mapping;
                    }
                }
            }

            if !modified {
                break;
            }
        }
    }

    fn reduce_possible_digits(
        &self,
        segment_mapping: &SegmentMapping,
        possible_pattern_digits: &mut Vec<HashSet<usize>>,
    ) {
        for (pat, possible_digits) in self.patterns.iter().zip(possible_pattern_digits.iter_mut()) {
            let possible_displayed_pattern = segment_mapping
                .iter()
                .enumerate()
                .filter(|&(segment_idx, _)| contains_segment(*pat, segment_idx))
                .fold(0, |acc, (_, mapping)| (acc | mapping));

            possible_digits.retain(|&i| DIGITS[i] & !possible_displayed_pattern == 0);
        }
    }
}

fn contains_segment(digit: Digit, segment_idx: usize) -> bool {
    (digit >> segment_idx) & 1 != 0
}

fn count_duplicates(input: &[u8]) -> HashMap<u8, usize> {
    let mut count = HashMap::new();
    for value in input {
        count
            .entry(*value)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    count
}

fn task1(tasks: Vec<Task>) -> usize {
    tasks
        .into_iter()
        .flat_map(Task::decode_output)
        .filter(|&digit| [1, 4, 7, 8].into_iter().any(|i| DIGITS[i] == digit))
        .count()
}

fn task2(tasks: Vec<Task>) -> usize {
    tasks
        .into_iter()
        .map(Task::decode_output)
        .map(|digits| {
            digits
                .into_iter()
                .map(digit_to_index)
                .fold(0, |acc, x| acc * 10 + x)
        })
        .sum()
}

fn main() {
    aoclib::AocTask::read_lines(|line| {
        let (patterns, output) = aoclib::split_into_two(line, " | ");

        let patterns = patterns
            .split_ascii_whitespace()
            .map(|s| construct_digit(s.as_bytes()))
            .collect();

        let output = output
            .split_ascii_whitespace()
            .map(|s| construct_digit(s.as_bytes()))
            .collect();

        Task { patterns, output }
    })
    .task1(task1)
    .task2(task2)
    .run_display();
}
