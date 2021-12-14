#![feature(array_windows)]

use std::collections::{BTreeMap, HashMap};

struct Data {
    sequence: Vec<u8>,
    substitutions: HashMap<[u8; 2], u8>,
}

fn calculate_solution(element_count: HashMap<u8, usize>) -> usize {
    let most_common = *element_count.iter().map(|(_, count)| count).max().unwrap();

    let least_common = *element_count.iter().map(|(_, count)| count).min().unwrap();

    most_common - least_common
}

fn simulate1(mut data: Data, iterations: usize) -> usize {
    for iter in 0..iterations {
        let mut next_sequence = Vec::with_capacity(data.sequence.len() * 3 / 2);
        next_sequence.push(data.sequence[0]);

        for pair in data.sequence.array_windows::<2>() {
            if let Some(&sub) = data.substitutions.get(pair) {
                next_sequence.push(sub);
            }
            next_sequence.push(pair[1]);
        }

        eprintln!("{iter}: {}", next_sequence.len());
        data.sequence = next_sequence;
    }

    let mut element_count = HashMap::new();
    for element in data.sequence {
        *element_count.entry(element).or_insert(0) += 1;
    }

    calculate_solution(element_count)
}

fn task1(data: Data) -> usize {
    simulate2(data, 10)
}

fn simulate2(data: Data, iterations: usize) -> usize {
    let mut pairs_map = HashMap::new();
    for &pair in data.sequence.array_windows::<2>() {
        *pairs_map.entry(pair).or_insert(0) += 1;
    }

    let mut next_pairs = HashMap::new();
    for _ in 0..iterations {
        next_pairs.clear();

        for (from, &to) in &data.substitutions {
            if let Some(count) = pairs_map.get(from) {
                *next_pairs.entry([from[0], to]).or_insert(0) += count;
                *next_pairs.entry([to, from[1]]).or_insert(0) += count;
            }
        }

        std::mem::swap(&mut pairs_map, &mut next_pairs);
    }

    let mut element_count0 = BTreeMap::new();
    let mut element_count1 = BTreeMap::new();
    for (pair, count) in pairs_map {
        *element_count0.entry(pair[0]).or_insert(0) += count;
        *element_count1.entry(pair[1]).or_insert(0) += count;
    }

    let element_count: HashMap<_, _> = element_count0
        .into_iter()
        .zip(element_count1)
        .map(|((key0, value0), (key1, value1))| {
            assert_eq!(key0, key1);
            (key0, value0.min(value1))
        })
        .collect();

    let (most_common_value, mut most_common_count) = element_count
        .iter()
        .max_by_key(|(_, count)| *count)
        .map(|(a, b)| (a, *b))
        .unwrap();

    let (least_common_value, mut least_common_count) = element_count
        .iter()
        .min_by_key(|(_, count)| *count)
        .map(|(a, b)| (a, *b))
        .unwrap();

    // This may not be entirely correct. Off-by-one errors are a fuck
    if most_common_value == data.sequence.first().unwrap()
        || most_common_value == data.sequence.last().unwrap()
    {
        most_common_count += 1;
    }
    if least_common_value == data.sequence.first().unwrap()
        || least_common_value == data.sequence.last().unwrap()
    {
        least_common_count += 1;
    }

    most_common_count - least_common_count
}

fn task2(data: Data) -> usize {
    simulate2(data, 40)
}

fn main() {
    aoclib::AocTask::read_full(|input| {
        let (fst, snd) = aoclib::split_into_two(input, "\n\n");

        Data {
            sequence: fst.into(),
            substitutions: snd
                .split('\n')
                .map(|line| {
                    let (fst, snd) = aoclib::split_into_two(line, " -> ");
                    assert_eq!(fst.len(), 2);
                    assert_eq!(snd.len(), 1);
                    (fst.as_bytes().try_into().unwrap(), snd.as_bytes()[0])
                })
                .collect(),
        }
    })
    .task1(task1)
    .task2(task2)
    .run_display();
}
