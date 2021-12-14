use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum CaveType {
    Small,
    Large,
}

type NodeIndex = petgraph::graph::NodeIndex<u32>;

struct CavesDesc {
    start: NodeIndex,
    end: NodeIndex,
    graph: petgraph::Graph<(String, CaveType), u32, petgraph::Undirected, u32>,
}

impl CavesDesc {
    fn count_paths_task1(&self, node: NodeIndex, stack: &mut Vec<NodeIndex>) -> usize {
        if node == self.end {
            return 1;
        }

        stack.push(node);

        let count = self
            .graph
            .neighbors(node)
            .filter_map(|neighbor| {
                let valid_path =
                    self.graph[neighbor].1 == CaveType::Large || !stack.contains(&neighbor);
                if valid_path {
                    Some(self.count_paths_task1(neighbor, stack))
                } else {
                    None
                }
            })
            .sum();

        stack.pop().unwrap();

        count
    }

    fn count_paths_task2(&self, node: NodeIndex, stack: &mut Vec<NodeIndex>, had_duplicate: bool) -> usize {
        if node == self.end {
            return 1;
        }

        stack.push(node);

        let count = self
            .graph
            .neighbors(node)
            .filter(|&neighbor| neighbor != self.start)
            .filter_map(|neighbor| {
                let mut had_duplicate = had_duplicate;
                let valid_path = if self.graph[neighbor].1 == CaveType::Large {
                    true
                } else if had_duplicate {
                    self.graph[neighbor].1 == CaveType::Large || !stack.contains(&neighbor)
                } else {
                    let count = stack.iter().filter(|&&x| x == neighbor).count();
                    if count > 0 {
                        had_duplicate = true;
                    }
                    self.graph[neighbor].1 == CaveType::Large || count < 2
                };

                if valid_path {
                    Some(self.count_paths_task2(neighbor, stack, had_duplicate))
                } else {
                    None
                }
            })
            .sum();

        stack.pop().unwrap();

        count
    }
}

fn task1(caves: CavesDesc) -> usize {
    caves.count_paths_task1(caves.start, &mut vec![])
}

fn task2(caves: CavesDesc) -> usize {
    caves.count_paths_task2(caves.start, &mut vec![], false)
}


fn main() {
    aoclib::AocTask::read_full(|input| {
        let mut nodes: HashMap<String, NodeIndex> = input
            .trim()
            .split('\n')
            .flat_map(|s| s.split('-'))
            .map(|s| (s.to_owned(), 0.into()))
            .collect();

        let mut graph = petgraph::Graph::new_undirected();
        for (name, value) in &mut nodes {
            let cave_type = if name.chars().all(|c| c.is_ascii_lowercase()) {
                CaveType::Small
            } else {
                CaveType::Large
            };
            *value = graph.add_node((name.to_owned(), cave_type));
        }

        for connection in input.trim().split('\n') {
            let (a, b) = aoclib::split_into_two(connection, "-");
            graph.add_edge(nodes[a], nodes[b], 0);
        }

        CavesDesc {
            graph,
            start: nodes["start"],
            end: nodes["end"],
        }
    })
    .task1(task1)
    .task2(task2)
    .run_display();
}
