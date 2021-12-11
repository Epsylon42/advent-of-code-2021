struct EnergyMap {
    map: Vec<u8>,
    num_flashes: usize,
    width: usize,
}

#[derive(PartialEq, Eq)]
enum StepResult {
    AllFlashed,
    NotAllFlashed,
}

impl EnergyMap {
    fn new(width: usize, height: usize) -> Self {
        EnergyMap {
            map: vec![0; width * height],
            num_flashes: 0,
            width,
        }
    }

    fn flatten_coord(&self, (x, y): (usize, usize)) -> usize {
        y * self.width + x
    }

    fn unflatten_coord(&self, coord: usize) -> (usize, usize) {
        (coord % self.width, coord / self.width)
    }

    fn neighbors_of(&self, (x, y): (usize, usize)) -> impl Iterator<Item = (usize, usize)> {
        let width = self.width as i32;
        let height = (self.map.len() / self.width) as i32;
        let original_coord = (x as i32, y as i32);

        (-1..=1)
            .flat_map(|ox| (-1..=1).map(move |oy| (ox, oy)))
            .map(move |(ox, oy)| (x as i32 + ox, y as i32 + oy))
            .filter(move |&(x, y)| {
                (x, y) != original_coord && (0..width).contains(&x) && (0..height).contains(&y)
            })
            .map(|(x, y)| (x as usize, y as usize))
    }

    fn step(&mut self) -> StepResult {
        for value in &mut self.map {
            *value += 1;
        }

        for flat_coord in 0..self.map.len() {
            if self.map[flat_coord] > 9 {
                self.flash(flat_coord)
            }
        }

        let mut result = StepResult::AllFlashed;
        for value in &mut self.map {
            if *value != 0 {
                result = StepResult::NotAllFlashed
            }
        }

        result
    }

    fn flash(&mut self, flat_coord: usize) {
        if self.map[flat_coord] == 0 {
            return;
        }

        self.map[flat_coord] = 0;
        self.num_flashes += 1;
        for neighbor_coord in self.neighbors_of(self.unflatten_coord(flat_coord)) {
            let flat_neighbor_coord = self.flatten_coord(neighbor_coord);
            if self.map[flat_neighbor_coord] != 0 {
                self.map[flat_neighbor_coord] += 1;
            }
            if self.map[flat_neighbor_coord] > 9 {
                self.flash(flat_neighbor_coord);
            }
        }
    }
}

fn task1(mut map: EnergyMap) -> usize {
    for _ in 0..100 {
        map.step();
    }

    map.num_flashes
}

fn task2(mut map: EnergyMap) -> usize {
    (1..)
        .find(|_| map.step() == StepResult::AllFlashed)
        .unwrap()
}

fn main() {
    aoclib::AocTask::read_full(|input| {
        let width = input.find('\n').unwrap();
        let height = input.trim().split('\n').count();

        let mut map = EnergyMap::new(width, height);

        for (i, c) in input
            .as_bytes()
            .into_iter()
            .filter(|c| (b'0'..=b'9').contains(c))
            .enumerate()
        {
            map.map[i] = c - b'0';
        }

        map
    })
    .task1(task1)
    .task2(task2)
    .run_display();
}
