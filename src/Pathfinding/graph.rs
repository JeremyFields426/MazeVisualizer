use std::collections::HashMap;

use super::Node;
use super::Node::Coords;

#[derive(Debug)]
pub struct Graph {
    nodes: HashMap<Coords, Node::Node>,

    size: i32,
}

impl Graph {
    pub fn new(size: i32) -> Graph {
        let mut graph = Graph { nodes: HashMap::new(), size };

        graph.create_nodes();
        graph.create_neighbors();

        return graph;
    }

    fn create_nodes(&mut self) {
        for x in 0..self.size {
            for y in 0..self.size {
                self.add_node(x, y);
            }
        }
    }

    fn create_neighbors(&mut self) {
        for node_coords in self.get_nodes() {
            let candidates: Vec<Coords> = vec![
                (node_coords.0 + 1, node_coords.1),
                (node_coords.0 - 1, node_coords.1),
                (node_coords.0, node_coords.1 - 1),
                (node_coords.0, node_coords.1 + 1)
            ];

            for neighbor_coords in candidates {
                if !self.is_valid_node_coordinate(neighbor_coords) { continue; }

                if node_coords == neighbor_coords { continue; }

                {
                    let opt_node = self.get_node_mut(node_coords);

                    if opt_node.is_none() { continue; }

                    let node = opt_node.unwrap();

                    node.add_neighbor(neighbor_coords);
                }

                let opt_neighbor = self.get_node_mut(neighbor_coords);

                if opt_neighbor.is_none() { return; }

                let neighbor = opt_neighbor.unwrap();

                neighbor.add_neighbor(node_coords);
            }
        }
    }

    pub fn add_node(&mut self, x: i32, y: i32) {
        let node = Node::Node::new(x, y);

        self.nodes.insert(node.get_coords(), node);
    }

    pub fn add_connection(&mut self, node_coords: Coords, neighbor_coords: Coords) {
        let opt_node = self.get_node_mut(node_coords);

        match opt_node {
            None => {},
            Some(node) => {
                node.add_connection(neighbor_coords);
            }
        }
    }

    pub fn remove_connection(&mut self, node_coords: Coords, neighbor_coords: Coords) {
        let opt_node = self.get_node_mut(node_coords);

        match opt_node {
            None => {},
            Some(node) => {
                node.remove_connection(neighbor_coords);
            }
        }
    }

    pub fn get_node(&self, coords: Coords) -> Option<&Node::Node> {
        return self.nodes.get(&coords);
    }

    pub fn get_node_mut(&mut self, coords: Coords) -> Option<&mut Node::Node> {
        return self.nodes.get_mut(&coords);
    }

    pub fn get_nodes(&self) -> Vec<Coords> {
        let mut nodes: Vec<Coords> = vec![];

        for coords in self.nodes.keys() {
            nodes.push(*coords);
        }

        return nodes;
    }

    pub fn get_neighbors(&self, node_coords: Coords) -> Vec<Coords> {
        let opt_node = self.get_node(node_coords);

        match opt_node {
            None => return vec![],
            Some(node) => return node.get_neighbors(),
        }
    }

    pub fn get_connections(&self, node_coords: Coords) -> Vec<Coords> {
        let opt_node = self.get_node(node_coords);

        match opt_node {
            None => return vec![],
            Some(node) => return node.get_connections(),
        }
    }

    pub fn is_connected(&self, node_coords: Coords, neighbor_coords: Coords) -> bool {
        let opt_node = self.get_node(node_coords);

        match opt_node {
            None => return true,
            Some(node) => return node.is_connected_to(neighbor_coords),
        }
    }

    pub fn is_valid_node_coordinate(&self, coords: Coords) -> bool {
        return 0 <= coords.0 && coords.0 < self.size && 0 <= coords.1 && coords.1 < self.size
    }

    pub fn get_size(&self) -> i32 {
        return self.size;
    }
}
