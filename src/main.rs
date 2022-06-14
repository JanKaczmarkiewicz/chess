mod core;
mod render;

use crate::core::state::State;
use render::Renderer;

fn main() -> Result<(), String> {
    let mut renderer = Renderer::new()?;
    let state = State::new();

    renderer.update(state)?;

    renderer.wait_for_input(())?;

    Ok(())
}
