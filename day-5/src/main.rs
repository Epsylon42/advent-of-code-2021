struct Line {
    start: (i16, i16),
    end: (i16, i16),
}

fn parse_line(s: &str) -> Line {
    let mut iter = s.trim().split(" -> ");
    let fst = iter.next().unwrap();
    let snd = iter.next().unwrap();
    assert_eq!(iter.next(), None);

    Line {
        start: parse_pair(fst),
        end: parse_pair(snd),
    }
}

fn parse_pair(s: &str) -> (i16, i16) {
    let mut iter = s.split(',');
    let fst = iter.next().unwrap();
    let snd = iter.next().unwrap();
    assert_eq!(iter.next(), None);

    (fst.parse().unwrap(), snd.parse().unwrap())
}

const MAP_SIZE: usize = 1024;

struct Map {
    data: Box<[[u16; MAP_SIZE]; MAP_SIZE]>,
    num_overlaps: usize,
}

impl Map {
    fn new() -> Self {
        Map {
            data: Box::new([[0; MAP_SIZE]; MAP_SIZE]),
            num_overlaps: 0,
        }
    }

    fn put(&mut self, (x, y): (i16, i16)) {
        self.data[y as usize][x as usize] += 1;
        if self.data[y as usize][x as usize] == 2 {
            self.num_overlaps += 1;
        }
    }

    fn render(&mut self, line: &Line) {
        let mut x = line.start.0;
        let sx = (line.end.0 - line.start.0).signum();
        let mut y = line.start.1;
        let sy = (line.end.1 - line.start.1).signum();

        while x != line.end.0 || y != line.end.1 {
            self.put((x, y));

            x += sx;
            y += sy;
        }
        self.put((x, y));
    }
}

fn task1(lines: Vec<Line>) -> usize {
    let mut map = Map::new();

    for line in &lines {
        if line.start.0 == line.end.0 || line.start.1 == line.end.1 {
            map.render(line);
        }
    }

    map.num_overlaps
}

fn task2(lines: Vec<Line>) -> usize {
    let mut map = Map::new();

    for line in &lines {
        map.render(line);
    }

    map.num_overlaps
}

fn main() {
    aoclib::AocTask::read_lines(parse_line)
    .task1(task1)
    .task2(task2)
    .run_display();
}
