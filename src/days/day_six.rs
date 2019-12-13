// thank you wes @repnop for helping me with the graph code <3
#[derive(Debug)]
struct Node {
    name: String,
    children: Vec<usize>,
    parent: Option<usize>,
}

impl Node {
    /// creates new Node
    fn new(name: String, children: Vec<usize>, parent: Option<usize>) -> Self {
        Self { name, children, parent }
    }
}

#[derive(Debug)]
struct Graph {
    nodes: Vec<Node>,
}

impl std::ops::Index<usize> for Graph {
    type Output = Node;

    fn index(&self, idx: usize) -> &Node {
        &self.nodes[idx]
    }
}

impl std::ops::IndexMut<usize> for Graph {
    fn index_mut(&mut self, idx: usize) -> &mut Node {
        &mut self.nodes[idx]
    }
}

impl Graph {
    /// creates new Graph
    fn new() -> Self {
        Self { nodes: Vec::new() }
    }

    /// find a node in the graph by it's name and return if exists
    fn find_node(&self, name: &str) -> Option<usize> {
        let idx = self.nodes.iter().position(|n| n.name == name)?;
        Some(idx)
    }

    /// create a new node and return it's index
    fn create_node(&mut self, node: Node) -> usize {
        self.nodes.push(node);
        self.nodes.len() - 1
    }

    /// add a child node to the graph
    fn add_child(&mut self, parent: usize, child: Node) {
        let idx = self.create_node(child);
        self[parent].children.push(idx);
    }

    /// return the root node in the graph
    #[allow(dead_code)]
    fn root(&self) -> Option<usize> {
        self.nodes.iter().position(|n| n.parent.is_none())
    }

    // RECURSION!
    fn get_parents(&self, node: usize) -> usize {
        let node = &self[node];
        match node.parent {
            None => 0,
            Some(parent_idx) => self.get_parents(parent_idx) + 1,
        }
    }

    /// construct a graph from the input data
    fn from_input(input: &str) -> Self {
        let mut graph = Graph::new();

        for line in input.lines() {
            let pos = line.chars().position(|c| c == ')').unwrap();
            let (orbiter, orbitee) = line.split_at(pos);
            let orbitee = &orbitee[1..];

            if let Some(parent) = graph.find_node(orbiter) {
                if let Some(child) = graph.find_node(orbitee) {
                    graph[parent].children.push(child);
                } else {
                    let node = Node::new(orbitee.into(), Vec::new(), Some(parent));
                    graph.add_child(parent, node);
                }
            } else {
                let node = Node::new(orbiter.into(), Vec::new(), None);
                let parent = graph.create_node(node);

                if let Some(child) = graph.find_node(orbitee) {
                    graph[child].parent = Some(parent);
                } else {
                    let node = Node::new(orbitee.into(), Vec::new(), Some(parent));
                    graph.add_child(parent, node);
                }
            }
        }

        graph
    }
}

pub fn part_one() {
    let timer = std::time::Instant::now();

    let graph = Graph::from_input(&std::fs::read_to_string("./inputs/day_six").unwrap());
    let mut count = 0;
    for node_idx in 0..graph.nodes.len() {
        count += graph.get_parents(node_idx);
    }
    println!("count: {}", count);

    println!("execution time: {}us", timer.elapsed().as_micros());
}

pub fn part_two() {
    let timer = std::time::Instant::now();

    //code here

    println!("execution time: {}us", timer.elapsed().as_micros());
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &'static str = "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L";
    #[test]
    fn day_six_part_one() {
        let graph = Graph::from_input(TEST_INPUT);

        let mut count = 0;
        for node_idx in 0..graph.nodes.len() {
            count += graph.get_parents(node_idx);
        }

        assert_eq!(count, 42);
    }

}
