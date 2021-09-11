pub mod engine;
pub mod entity_manager;
pub mod component_manager;
pub mod components;

use engine::Engine;

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 800;

fn main() {
    Engine::new(WINDOW_WIDTH, WINDOW_HEIGHT)
        .expect("failed to create engine")
        .run();
}
