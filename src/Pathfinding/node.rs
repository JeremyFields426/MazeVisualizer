use std::{collections::HashSet};


pub type Coords = (i32, i32);

#[derive(Debug, PartialEq)]
pub struct Node {
    pub x: i32,
    pub y: i32,
    
    neighbors: HashSet<Coords>,
    connections: HashSet<Coords>,
}

impl Node {
    pub fn new(x: i32, y: i32) -> Node {
        return Node { x, y, neighbors: HashSet::new(), connections: HashSet::new() };
    }
    
    pub fn is_connected_to(&self, neighbor_coords: Coords) -> bool {
        return self.connections.contains(&neighbor_coords);
    }

    pub fn add_neighbor(&mut self, neighbor_coords: Coords) {
        self.neighbors.replace(neighbor_coords);
    }

    pub fn get_neighbors(&self) -> Vec<Coords> {
        let mut neighbors: Vec<Coords> = vec![];

        for neighbor_coords in self.neighbors.iter() {
            neighbors.push(*neighbor_coords);
        }

        return neighbors;
    }

    pub fn add_connection(&mut self, neighbor_coords: Coords) {
        self.connections.replace(neighbor_coords);
    }

    pub fn remove_connection(&mut self, neighbor_coords: Coords) {
        self.connections.remove(&neighbor_coords);
    }

    pub fn get_connections(&self) -> Vec<Coords> {
        let mut connections: Vec<Coords> = vec![];

        for connection_coords in self.connections.iter() {
            connections.push(*connection_coords);
        }

        return connections;
    }

    pub fn get_distance_between(node_coords: Coords, other_coords: Coords) -> i32 {
        return i32::abs(node_coords.0 - other_coords.0) + i32::abs(node_coords.1 - other_coords.1)
    }

    pub fn get_coords(&self) -> Coords {
        return (self.x, self.y)
    }
}
