const fn is_opening_paren(paren: char) -> bool {
    matches!(paren, '(' | '[' | '{' | '<')
}

const fn corrupted_score(paren: char) -> u32 {
    match paren {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!("unexpected character")
    }
}

const fn incomplete_score(paren: char) -> u32 {
    match paren {
        '(' => 1,
        '[' => 2,
        '{' => 3,
        '<' => 4,
        _ => panic!("unexpected character")
    }
}

const fn pair_of(paren: char) -> char {
    match paren {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => panic!("unexpected character")
    }
}

fn task1(lines: Vec<String>) -> u32 {
    let mut score = 0;
    'lines: for line in lines {
        let mut paren_stack = Vec::new();
        for paren in line.chars() {
            if is_opening_paren(paren) {
                paren_stack.push(paren);
            } else if let Some(opening) = paren_stack.pop() {
                if paren != pair_of(opening) {
                    score += corrupted_score(paren);
                    continue 'lines;
                }
            } else {
                panic!("unexpected character")
            }
        }
    }

    score
}

fn calculate_incomplete_score(paren_stack: Vec<char>) -> u64 {
    paren_stack
        .into_iter()
        .rev()
        .map(incomplete_score)
        .fold(0, |acc, score| acc * 5 + score as u64)
}

fn task2(lines: Vec<String>) -> u64 {
    let mut scores = Vec::new();

    'lines: for line in lines {
        let mut paren_stack = Vec::new();
        for paren in line.chars() {
            if is_opening_paren(paren) {
                paren_stack.push(paren);
            } else if let Some(opening) = paren_stack.pop() {
                if paren != pair_of(opening) {
                    paren_stack.push(opening);
                    continue 'lines;
                }
            } else {
                panic!("unexpected character")
            }
        }

        if !paren_stack.is_empty() {
            scores.push(calculate_incomplete_score(paren_stack));
        }
    }

    scores.sort();
    scores[scores.len() / 2]
}

fn main() {
    aoclib::AocTask::read_lines(str::to_owned)
        .task1(task1)
        .task2(task2)
        .run_display();
}
