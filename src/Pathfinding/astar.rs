use std::collections::HashMap;
use priority_queue::PriorityQueue;

use super::Node;
use super::Graph;
use super::Node::Coords;

pub struct Astar {
    start_coords: Coords,
    goal_coords: Coords,
    current_coords: Coords,

    open_set: PriorityQueue<Coords, i32>,
    came_from: HashMap<Coords, Coords>,
    g_score: HashMap<Coords, i32>,
    h_score: HashMap<Coords, i32>,

    path: Vec<Coords>,
    is_finished: bool
}

impl Astar {
    pub fn new() -> Astar {
        return Astar { 
            start_coords: (0, 0), 
            goal_coords: (0, 0), 
            current_coords: (0, 0),
            open_set: PriorityQueue::new(),
            came_from: HashMap::new(),
            g_score: HashMap::new(),
            h_score: HashMap::new(),
            path: vec![],
            is_finished: true
        };
    }

    pub fn initialize(&mut self, start_coords: Coords, goal_coords: Coords) {
        self.start_coords = start_coords;
        self.goal_coords = goal_coords;
        self.current_coords = start_coords;
        self.open_set = PriorityQueue::new();
        self.came_from = HashMap::new();
        self.g_score = HashMap::new();
        self.h_score = HashMap::new();
        self.path = vec![];
        self.is_finished = false;

        self.g_score.insert(start_coords, 0);
        self.add_to_open_set(start_coords);
    }

    pub fn generate_path(&mut self, graph: &Graph::Graph) {
        if self.is_finished { return; }

        let current_coords_opt = self.get_next();

        if current_coords_opt.is_none() {
            self.is_finished = true;

            return;
        }

        self.current_coords = current_coords_opt.unwrap();

        if self.current_coords == self.goal_coords {
            self.reconstruct_path();

            self.is_finished = true;

            return;
        }

        for connection_coords in graph.get_connections(self.current_coords) {
            let tentative_g_score = self.g(self.current_coords) + 
                Node::Node::get_distance_between(self.current_coords, connection_coords);

            if tentative_g_score < self.g(connection_coords) {
                self.came_from.insert(connection_coords, self.current_coords);

                self.g_score.insert(connection_coords, tentative_g_score);

                self.add_to_open_set(connection_coords);
            }
        }
    }

    fn reconstruct_path(&mut self) {
        self.path.push(self.current_coords);

        loop {
            let opt_current_coords = self.came_from.get(&self.current_coords);

            match opt_current_coords {
                None => break,
                Some(current_coords) => {
                    self.current_coords = *current_coords;

                    self.path.push(self.current_coords);
                }
            }
        }
    }

    fn add_to_open_set(&mut self, node_coords: Coords) {
        if let Some(_) = self.open_set.get(&node_coords) { return; }

        let f = self.f(node_coords);

        self.open_set.push(node_coords, f);
    }

    fn get_next(&mut self) -> Option<Coords> {
        let opt_node_coords = self.open_set.pop();

        match opt_node_coords {
            None => return None::<Coords>,
            Some(node_coords) => return Some(node_coords.0)
        }
    }

    fn g(&mut self, node_coords: Coords) -> i32 {
        if let Some(g) = self.g_score.get(&node_coords) {
            return *g;
        }

        let g = 100_000;

        self.g_score.insert(node_coords, g);

        return g;
    }

    fn h(&mut self, node_coords: Coords) -> i32 {
        if let Some(h) = self.h_score.get(&node_coords) {
            return *h;
        }

        let h = Node::Node::get_distance_between(node_coords, self.goal_coords);

        self.h_score.insert(node_coords, h);

        return h;
    }

    fn f(&mut self, node_coords: Coords) -> i32 {
        return 100_000_000 - self.g(node_coords) + self.h(node_coords)
    }

    pub fn get_start_coords(&self) -> Coords {
        return self.start_coords;
    }

    pub fn get_current_coords(&self) -> Coords {
        return self.current_coords;
    }

    pub fn get_goal_coords(&self) -> Coords {
        return self.goal_coords;
    }

    pub fn get_open_set(&self) -> Vec<Coords> {
        let mut nodes: Vec<Coords> = vec![];

        for node in &self.open_set {
            nodes.push(*node.0);
        }

        return nodes;
    }

    pub fn get_path(&self) -> Vec<Coords> {
        return self.path.clone();
    }

    pub fn is_finished(&self) -> bool {
        return self.is_finished;
    }
}