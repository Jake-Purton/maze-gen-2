use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::sync::mpsc::Sender;
use std::thread;
use std::time::Duration;
use rand::thread_rng;
use rand::seq::SliceRandom;

#[derive(Clone)]
pub struct Graph<VId, E = (), V = ()> {
    pub vertices: HashMap<VId, V>,
    pub adjacency: HashMap<VId, Vec<(VId, E)>>,
}

impl <VId, E, V> Graph<VId, E, V> 
where 
    VId: Eq + Hash,
    V: Hash,
{

    pub fn new() -> Graph<VId, E, V> {
        Graph {
            vertices: HashMap::new(),
            adjacency: HashMap::new()
        }
    }

    pub fn push_vertex (self: &mut Graph<VId, E, V>, vid: VId, vertex: V) {
        self.vertices.insert(vid, vertex);
    }

    // edge in one direction
    pub fn push_edge (self: &mut Graph<VId, E, V>, from: VId, to: VId, edge: E) {
        let adjacent_to_from = self.adjacency.entry(from).or_default();
        adjacent_to_from.push((to, edge));
    }

    pub fn get_mut_edges(self: &mut Graph<VId, E, V>, vid: VId) -> Option<&mut Vec<(VId, E)>> {

        self.adjacency.get_mut(&vid)

    }

    pub fn get_edges(self: &Graph<VId, E, V>, vid: VId) -> Option<&Vec<(VId, E)>> {

        self.adjacency.get(&vid)

    }
}

pub fn random_depth_first (maze: &mut Graph<u32, EdgeNode, VertexNode>, initial: u32) {

    let mut visited: HashSet<u32> = HashSet::new();
    let mut stack: Vec<u32> = Vec::new();
    let mut edge_pairs: HashSet<(u32, u32)> = HashSet::new();

    visited.insert(initial);
    stack.push(initial);

    while let Some(current) = stack.pop() {
        if let Some(edges) = maze.get_mut_edges(current) {

            edges.shuffle(&mut thread_rng());
            
            // make this random
            for (vid, edge) in edges.iter_mut() {

                if visited.contains(&vid) {

                    continue;

                } else {
                    
                    stack.push(current);
                    *edge = EdgeNode::Some;

                    edge_pairs.insert((current, *vid));

                    visited.insert(vid.clone());
                    stack.push(vid.clone());

                    break;
                }     
            }

        }
    }

    for pair in edge_pairs {
        let edges = maze.adjacency.get_mut(&pair.1).unwrap();

        for edge in edges {
            if edge.0 == pair.0 {
                edge.1 = EdgeNode::Some;
            }
        }
    }

}

pub fn depth_find_path (start: u32, end: u32, maze: Graph<u32, EdgeNode, VertexNode>, tx: Sender<(u32, bool)>,) {

    let mut stack = Vec::new();
    stack.push(start);

    let mut path = Vec::new();
    path.push(start);

    let mut visited = HashSet::new();

    while let Some(cell) = stack.pop() {

        visited.insert(cell);
        let _ = tx.send((cell, false));

        if cell == end {
            break;
        }

        if let Some(edges) = maze.get_edges(cell) {

            let mut a = false;

            for edge in edges {

                if matches!(edge.1, EdgeNode::Some) && !visited.contains(&edge.0) {

                    a = true;
                    path.push(edge.0);
                    stack.push(edge.0);

                }

                
            }

            if !a {
                while let Some(b) = path.pop() {

                    if let Some(c) = stack.pop() {
                        stack.push(c);

                        if b == c {
                            path.push(b);
                            break;
                        }
                    }                    
                }
            }
        }

        thread::sleep(Duration::from_millis(1))

    }

    thread::sleep(Duration::from_millis(100));

    println!("end found");
    for i in path {
        thread::sleep(Duration::from_millis(10));
        let _ = tx.send((i, true));
    }


}

#[derive(Hash, Debug, Clone)]
pub struct VertexNode;

#[derive(Hash, Debug, Clone)]
pub enum EdgeNode {
    Some,
    None,
}

pub fn new_maze (square_size: u32) -> Graph<u32, EdgeNode, VertexNode> {

    let mut maze: Graph<u32, EdgeNode, VertexNode> = Graph::new();

    for y in 0..square_size {
        for x in 0..square_size {
            let id = x + (y * square_size);

            maze.push_vertex(id, VertexNode);

            if let Some(sub) = x.checked_sub(1) {
                let neighbor_id = sub + (y * square_size);
                maze.push_edge(id, neighbor_id, EdgeNode::None)
            }

            if let Some(add) = x.checked_add(1) {
                if add < square_size {
                    let neighbor_id = add + (y * square_size);
                    maze.push_edge(id, neighbor_id, EdgeNode::None)
                }
            }

            if let Some(sub) = y.checked_sub(1) {
                let neighbor_id = x + (sub * square_size);
                maze.push_edge(id, neighbor_id, EdgeNode::None)
            }

            if let Some(add) = y.checked_add(1) {
                if add < square_size {
                    let neighbor_id = x + (add * square_size);
                    maze.push_edge(id, neighbor_id, EdgeNode::None)
                }
            }
        }
    }

    maze
}