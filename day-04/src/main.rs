use std::collections::HashMap;

const VICTORY_CONDITIONS: [u32; 10] = [
    0b_11111_00000_00000_00000_00000,
    0b_00000_11111_00000_00000_00000,
    0b_00000_00000_11111_00000_00000,
    0b_00000_00000_00000_11111_00000,
    0b_00000_00000_00000_00000_11111,
    0b_10000_10000_10000_10000_10000,
    0b_01000_01000_01000_01000_01000,
    0b_00100_00100_00100_00100_00100,
    0b_00010_00010_00010_00010_00010,
    0b_00001_00001_00001_00001_00001,
];

struct Board {
    numbers: [[u8; 5]; 5],
    marked_mask: u32,
}

impl Board {
    fn mark(&mut self, (row_n, col_n): (u8, u8)) {
        self.marked_mask |= (1 << col_n) << (5 * row_n);
    }

    fn check_marked(&self, (row_n, col_n): (u8, u8)) -> bool {
        self.marked_mask & ((1 << col_n) << (5 * row_n)) != 0
    }

    fn check_victory(&self) -> bool {
        for cond in VICTORY_CONDITIONS {
            if ((self.marked_mask & cond) | !cond) == u32::MAX {
                return true;
            }
        }

        false
    }

    fn calculate_score(&self, last_number: u8) -> u32 {
        let mut score = 0;
        for (row_n, row) in self.numbers.iter().enumerate() {
            for (col_n, &num) in row.iter().enumerate() {
                if !self.check_marked((row_n as u8, col_n as u8)) {
                    score += num as u32;
                }
            }
        }

        score * last_number as u32
    }
}

struct Input {
    sequence: Vec<u8>,
    boards: Vec<Board>,
}

struct Maps {
    outer: Vec<Vec<usize>>,
    inner: Vec<HashMap<u8, (u8, u8)>>,
}

impl Maps {
    fn generate(input: &Input) -> Self {
        let mut maps = Maps {
            outer: vec![Vec::new(); 256],
            inner: Vec::new(),
        };

        for (board_n, board) in input.boards.iter().enumerate() {
            let mut inner_map = HashMap::new();

            for (row_n, row) in board.numbers.iter().enumerate() {
                for (col_n, num) in row.iter().enumerate() {
                    inner_map.insert(*num, (row_n as u8, col_n as u8));
                    maps.outer[*num as usize].push(board_n);
                }
            }

            maps.inner.push(inner_map);
        }

        maps
    }
}

fn task1(mut input: Input) -> u32 {
    let maps = Maps::generate(&input);

    for number in input.sequence {
        for &board_n in &maps.outer[number as usize] {
            let board = &mut input.boards[board_n];

            let coord = maps.inner[board_n][&number];
            board.mark(coord);
            if board.check_victory() {
                return board.calculate_score(number);
            }
        }
    }

    unreachable!()
}

fn task2(mut input: Input) -> u32 {
    let maps = Maps::generate(&input);
    let mut won_boards_n = 0;
    let mut won_boards: Vec<bool> = (0..input.boards.len()).map(|_| false).collect();

    for number in input.sequence {
        for &board_n in &maps.outer[number as usize] {
            if won_boards[board_n] {
                continue;
            }
            let board = &mut input.boards[board_n];

            let coord = maps.inner[board_n][&number];
            board.mark(coord);
            if board.check_victory() {
                won_boards_n += 1;
                won_boards[board_n] = true;

                if won_boards_n == input.boards.len() {
                    return input.boards[board_n].calculate_score(number);
                }
            }
        }
    }

    unreachable!()
}

fn main() {
    aoclib::AocTask::read_full(|s| {
        let first_line_end = s.find('\n').unwrap();
        let sequence = s[..first_line_end]
            .split(',')
            .map(str::parse)
            .map(Result::unwrap)
            .collect();

        let boards = s[first_line_end..]
            .trim()
            .split("\n\n")
            .map(|board_s| {
                let mut numbers = [[0; 5]; 5];
                for (row_n, line) in board_s.split('\n').enumerate() {
                    for (col_n, num) in line.split_ascii_whitespace().enumerate() {
                        numbers[row_n][col_n] = num.parse().unwrap();
                    }
                }

                Board {
                    numbers,
                    marked_mask: 0,
                }
            })
            .collect();

        Input { sequence, boards }
    })
    .task1(task1)
    .task2(task2)
    .run_display();
}
