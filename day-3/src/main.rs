#[derive(Clone)]
struct BitField {
    bits: Vec<bool>,
}

impl BitField {
    fn from_conditional(size: usize, cond: impl FnMut(usize) -> bool) -> Self {
        BitField {
            bits: (0..size).map(cond).collect(),
        }
    }

    fn to_number(&self) -> u16 {
        let mut value = 0;
        for bit in &self.bits {
            value *= 2;
            value += *bit as u16;
        }

        value
    }
}

fn task1(values: Vec<BitField>) -> u32 {
    let len = values[0].bits.len();
    let mut one_count = vec![0; len];

    for field in &values {
        for (b, count) in field.bits.iter().zip(&mut one_count) {
            *count += *b as usize;
        }
    }

    let gamma = BitField::from_conditional(len, |i| one_count[i] > values.len() / 2).to_number();
    let eta = BitField::from_conditional(len, |i| one_count[i] < values.len() / 2).to_number();

    gamma as u32 * eta as u32
}

fn task2(values: Vec<BitField>) -> u32 {
    task2_gas(values.clone(), true) as u32 * task2_gas(values, false) as u32
}

fn task2_gas(mut values: Vec<BitField>, bit_criteria: bool) -> u16 {
    let len = values[0].bits.len();
    for i in 0..len {
        if values.len() <= 1 {
            break;
        }

        let one_count = values
            .iter()
            .filter(|value| value.bits[i])
            .count();
        let zero_count = values.len() - one_count;

        #[rustfmt::skip]
        let expected = if bit_criteria {
            if one_count >= zero_count { true } else { false }
        } else {
            if zero_count <= one_count { false } else { true }
        };

        values.retain(|value| value.bits[i] == expected);
    }

    values.pop().unwrap().to_number()
}

fn main() {
    aoclib::AocTask::read_lines(|line| {
        let bits = line
            .chars()
            .map(|c| match c {
                '0' => false,
                '1' => true,
                _ => panic!("Unexpected character"),
            })
            .collect();

        BitField { bits }
    })
    .task1(task1)
    .task2(task2)
    .run_display();
}
