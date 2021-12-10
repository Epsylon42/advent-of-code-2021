use std::collections::HashSet;

struct HeightMap {
    map: Vec<u8>,
    width: usize,
}

impl HeightMap {
    fn new(width: usize, height: usize) -> Self {
        HeightMap {
            map: vec![0; width * height],
            width,
        }
    }

    fn flatten_coord(&self, (x, y): (usize, usize)) -> usize {
        y * self.width + x
    }

    fn unflatten_coord(&self, coord: usize) -> (usize, usize) {
        (coord % self.width, coord / self.width)
    }

    fn index(&self, coord: (usize, usize)) -> &u8 {
        &self.map[self.flatten_coord(coord)]
    }

    fn neighbors_of(&self, (x, y): (usize, usize)) -> impl Iterator<Item = (usize, usize)> {
        let width = self.width as i32;
        let height = (self.map.len() / self.width) as i32;

        [(-1, 0), (1, 0), (0, -1), (0, 1)]
            .into_iter()
            .map(move |(ox, oy)| (x as i32 + ox, y as i32 + oy))
            .filter(move |(x, y)| (0..width).contains(x) && (0..height).contains(y))
            .map(|(x, y)| (x as usize, y as usize))
    }
}

fn lowest_points<'a>(heightmap: &'a HeightMap) -> impl Iterator<Item = (usize, u8)> + 'a {
    heightmap
        .map
        .iter()
        .copied()
        .enumerate()
        .filter(|&(flat_coord, height)| {
            heightmap
                .neighbors_of(heightmap.unflatten_coord(flat_coord))
                .map(|coord| *heightmap.index(coord))
                .all(|neighbor_height| height < neighbor_height)
        })
}

fn task1(heightmap: HeightMap) -> u32 {
    lowest_points(&heightmap)
        .map(|(_, height)| height as u32 + 1)
        .sum()
}

fn basin_size(heightmap: &HeightMap, origin: (usize, usize)) -> u32 {
    let mut visited = HashSet::<(usize, usize)>::new();
    let mut to_visit = vec![origin];
    while let Some(coord) = to_visit.pop() {
        let flow_sources = heightmap
            .neighbors_of(coord)
            .filter(|&neighbor_coord| heightmap.index(neighbor_coord) >= heightmap.index(coord))
            .filter(|&neighbor_coord| *heightmap.index(neighbor_coord) != 9)
            .filter(|neighbor_coord| !visited.contains(neighbor_coord));

        to_visit.extend(flow_sources);
        visited.insert(coord);
    }

    visited.len() as u32
}

fn task2(heightmap: HeightMap) -> u32 {
    let mut basin_sizes = lowest_points(&heightmap)
        .map(|(flat_coord, _)| basin_size(&heightmap, heightmap.unflatten_coord(flat_coord)))
        .collect::<Vec<_>>();

    basin_sizes.sort();
    basin_sizes.into_iter().rev().take(3).product()
}

fn main() {
    aoclib::AocTask::read_full(|input| {
        let width = input.find('\n').unwrap();
        let height = input.trim().split('\n').count();

        let mut map = HeightMap::new(width, height);

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
