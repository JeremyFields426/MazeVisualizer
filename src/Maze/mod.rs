use std::{collections::HashSet};

use rand::Rng;

use crate::Pathfinding::{astar, Graph, Node::{Coords}};

pub struct Maze {
    pub graph: Graph::Graph,
    pub astar: astar::Astar,

    visited: HashSet<Coords>,
    stack: Vec<Coords>,
    current_coords: Coords,
    is_finished: bool,
}

impl Maze {
    pub fn new(size: i32) -> Maze {
        return Maze { 
            graph: Graph::Graph::new(size), 
            astar: astar::Astar::new(),

            visited: HashSet::new(),
            stack: vec![],
            current_coords: (0, 0),
            is_finished: false
        };
    }

    pub fn generate_maze(&mut self) {
        if self.is_finished { return; }

        self.visited.replace(self.current_coords);

        let mut opt_neighbor_coords = self.get_random_unvisited_neighbor(self.current_coords);

        if let Some(neighbor_coords) = opt_neighbor_coords {
            self.graph.add_connection(self.current_coords, neighbor_coords);
            self.graph.add_connection(neighbor_coords, self.current_coords);

            self.stack.push(self.current_coords);

            self.current_coords = neighbor_coords;

            return
        }

        opt_neighbor_coords = self.stack.pop();

        if let Some(neighbor_coords) = opt_neighbor_coords {
            self.current_coords = neighbor_coords;

            return
        }

        self.is_finished = true;
    }

    fn get_random_unvisited_neighbor(&self, node_coords: Coords) -> Option<Coords> {
        let mut neighbors = self.graph.get_neighbors(node_coords);

        let mut rng = rand::thread_rng();

        while !neighbors.is_empty() {
            let index = rng.gen_range(0..neighbors.len());

            let neighbor_coords = neighbors[index];

            if !self.visited.contains(&neighbor_coords) {
                return Some(neighbor_coords);
            }

            neighbors.remove(index);
        }

        return None::<Coords>;
    }

    pub fn get_current_coords(&self) -> Coords {
        return self.current_coords;
    }

    pub fn is_finished(&self) -> bool {
        return self.is_finished;
    }
}
