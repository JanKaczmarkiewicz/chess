mod core;
mod render;

use crate::core::state::State;

use render::Renderer;

fn main() -> Result<(), String> {
    let renderer = Renderer::new()?;
    let state = State::new();

    loop {}

    renderer.update(state)?;

    Ok(())
}
