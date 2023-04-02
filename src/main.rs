mod Maze;
mod Rendering;
mod Pathfinding;

use piston::input::{RenderEvent, UpdateEvent};

fn main() {
    let size = 50;

    let mut maze = Maze::Maze::new(size);

    let mut app = Rendering::app::Application::new("Maze Runner".to_string(), 750, 750);

    while let Some(e) = app.get_next_event() {
        if let Some(args) = e.render_args() {
            app.render(&maze, &args);
        }

        if let Some(args) = e.update_args() {
            app.update(&mut maze, &args);
        }
    }
}
