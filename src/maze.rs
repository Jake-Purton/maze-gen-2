use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use rand::thread_rng;
use rand::seq::SliceRandom;

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

    pub fn get_vertex(self: &Graph<VId, E, V>, vid: &VId) -> Option<&V> {
        self.vertices.get(vid)
    }

}

pub fn random_depth_first (maze: &mut Graph<u32, EdgeNode, VertexNode>, initial: u32) {

    let mut visited: HashSet<u32> = HashSet::new();
    let mut stack: Vec<u32> = Vec::new();

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

                    visited.insert(vid.clone());
                    stack.push(vid.clone());

                    break;
                }     
            }

        }
    }
}

#[derive(Hash, Debug)]
pub enum VertexNode {
    Start,
    None,
    End,
}

#[derive(Hash, Debug)]
pub enum EdgeNode {
    Some,
    None,
}

pub fn new_maze (square_size: u32) -> Graph<u32, EdgeNode, VertexNode> {

    let mut maze: Graph<u32, EdgeNode, VertexNode> = Graph::new();

    for y in 0..square_size {
        for x in 0..square_size {
            let id = x + (y * square_size);

            maze.push_vertex(id, VertexNode::None);

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