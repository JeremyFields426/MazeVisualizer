extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use core::time;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, UpdateArgs};
use piston::window::WindowSettings;

use rand::Rng;

use crate::Pathfinding::Node::Coords;
use crate::Pathfinding::{Graph};
use crate::Maze;

pub struct Application {
    window: Window,
    graphics: GlGraphics,
    events: Events,

    path_timer: f64,
}

impl Application {
    pub fn new(title: String, width: u32, height: u32) -> Application {
        let opengl = OpenGL::V3_2;

        let window: Window = WindowSettings::new(title, [width, height])
            .graphics_api(opengl)
            .exit_on_esc(true)
            .build()
            .unwrap();

        let graphics = GlGraphics::new(opengl);

        let events = Events::new(EventSettings::new());

        let app = Application { graphics, window, events, path_timer: 0.0 };

        return app;
    }

    pub fn get_next_event(&mut self) -> Option<piston::Event> {
        return self.events.next(&mut self.window);
    }

    pub fn render(&mut self, maze: &Maze::Maze, args: &RenderArgs) {
        let (width, height) = (args.window_size[0], args.window_size[1]);

        self.graphics.draw(args.viewport(), |context, gl| {
            Application::clear_screen(gl);

            if !maze.is_finished() {
                Application::render_maze_generation(&maze.graph, maze.get_current_coords(), width, height, &context, gl);
            }

            if !maze.astar.is_finished() {
                Application::render_openset(maze, width, height, &context, gl);
            }
            
            Application::render_path(maze, width, height, &context, gl);
            Application::render_ends(maze, width, height, &context, gl);

            Application::render_border(width, height, &context, gl);
            Application::render_connections(&maze.graph, width, height, &context, gl);
        });
    }

    fn clear_screen(gl: &mut GlGraphics) {
        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

        graphics::clear(WHITE, gl);
    }

    fn render_openset(maze: &Maze::Maze, width: f64, height: f64, context: &graphics::Context, gl: &mut GlGraphics) {
        const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];

        for current_coords in maze.astar.get_open_set() {
            Application::draw_coords(&maze.graph, BLUE, current_coords, width, height, context, gl);
        }
    }

    fn render_ends(maze: &Maze::Maze, width: f64, height: f64, context: &graphics::Context, gl: &mut GlGraphics) {
        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

        Application::draw_coords(&maze.graph, GREEN, maze.astar.get_start_coords(), width, height, context, gl);
        Application::draw_coords(&maze.graph, BLACK, maze.astar.get_current_coords(), width, height, context, gl);
        Application::draw_coords(&maze.graph, RED, maze.astar.get_goal_coords(), width, height, context, gl);
    }

    fn render_path(maze: &Maze::Maze, width: f64, height: f64, context: &graphics::Context, gl: &mut GlGraphics) {
        const ORANGE: [f32; 4] = [1.0, 0.64, 0.0, 1.0];

        for current_coords in maze.astar.get_path() {
            Application::draw_coords(&maze.graph, ORANGE, current_coords, width, height, context, gl);
        }
    }

    fn render_border(width: f64, height: f64, context: &graphics::Context, gl: &mut GlGraphics) {
        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        
        let transform = context.transform;

        graphics::line(BLACK, 1.0, [1.0, 1.0, width - 1.0, 1.0], transform, gl);

        graphics::line(BLACK, 1.0, [1.0, 1.0, 1.0, height - 1.0], transform, gl);

        graphics::line(BLACK, 1.0, [width - 1.0, height - 1.0, width - 1.0, 1.0], transform, gl);

        graphics::line(BLACK, 1.0, [width - 1.0, height - 1.0, 1.0, height - 1.0], transform, gl);
    }

    fn render_maze_generation(graph: &Graph::Graph, current_coords: Coords, width: f64, height: f64, context: &graphics::Context, gl: &mut GlGraphics) {
        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

        Application::draw_coords(graph, BLACK, current_coords, width, height, context, gl);
    }

    fn render_connections(graph: &Graph::Graph, width: f64, height: f64, context: &graphics::Context, gl: &mut GlGraphics) {
        for node_coords in graph.get_nodes() {
            for neighbor_coords in graph.get_neighbors(node_coords) {
                if graph.is_connected(node_coords, neighbor_coords) { continue; }

                Application::render_connection(graph, node_coords, neighbor_coords, width, height, context, gl);
            }
        }
    }

    fn render_connection(
        graph: &Graph::Graph, node_coords: Coords, neighbor_coords: Coords, width: f64, height: f64, context: &graphics::Context, gl: &mut GlGraphics
    ) {
        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

        let node_coords_f = (f64::from(node_coords.0), f64::from(node_coords.1));

        let graph_size = f64::from(graph.get_size());

        let tile_width = width / graph_size;
        let tile_height = height / graph_size;

        let top_left = ((node_coords_f.0 / graph_size) * width, (node_coords_f.1 / graph_size) * height);
        let top_right = (top_left.0 + tile_width, top_left.1);
        let bottom_left = (top_left.0, top_left.1 + tile_height);
        let bottom_right = (top_left.0 + tile_width, top_left.1 + tile_height);

        let transform = context.transform;

        if node_coords.0 == neighbor_coords.0 {
            if node_coords.1 < neighbor_coords.1 { // Connection from bottom.
                graphics::line(BLACK, 1.0, [bottom_right.0, bottom_right.1, bottom_left.0, bottom_left.1], transform, gl);
            } else { // Connection from top.
                graphics::line(BLACK, 1.0, [top_left.0, top_left.1, top_right.0, top_right.1], transform, gl);
            }
        } else {
            if node_coords.0 < neighbor_coords.0 { // Connection from right.
                graphics::line(BLACK, 1.0, [bottom_right.0, bottom_right.1, top_right.0, top_right.1], transform, gl);
            } else { // Connection from left.
                graphics::line(BLACK, 1.0, [top_left.0, top_left.1, bottom_left.0, bottom_left.1], transform, gl);
            }
        }
    }

    fn draw_coords(graph: &Graph::Graph, color: [f32; 4], current_coords: Coords, width: f64, height: f64, context: &graphics::Context, gl: &mut GlGraphics) {
        let current_coords_f = (f64::from(current_coords.0), f64::from(current_coords.1));

        let graph_size = f64::from(graph.get_size());

        let tile_width = width / graph_size;
        let tile_height = height / graph_size;

        let top_left = ((current_coords_f.0 / graph_size) * width, (current_coords_f.1 / graph_size) * height);

        graphics::rectangle(color, [top_left.0, top_left.1, tile_width, tile_height], context.transform, gl);
    }

    pub fn update(&mut self, maze: &mut Maze::Maze, args: &UpdateArgs) {
        if !maze.is_finished() {
            for _ in 0..100 {
                maze.generate_maze();
            }

            return;
        }

        if !maze.astar.is_finished() {
            for _ in 0..3 {
                maze.astar.generate_path(&maze.graph);
            }
        } else if self.path_timer > 1.0 {
            let size = maze.graph.get_size();

            let mut rng = rand::thread_rng();

            let start_coords = (rng.gen_range(0..size), rng.gen_range(0..size));
            let goal_coords = (rng.gen_range(0..size), rng.gen_range(0..size));

            maze.astar.initialize(start_coords, goal_coords);

            self.path_timer = 0.0;
        } else {
            self.path_timer += args.dt;
        }
    }
}