mod core;
mod render;

use crate::core::state::State;
use render::Renderer;

fn main() -> Result<(), String> {
    let mut renderer = Renderer::new()?;
    let mut state = State::new();

    renderer.update(&state)?;

    for input in renderer.input_iter()? {
        state.handle_action(input);
        renderer.update(&state)?;
    }

    Ok(())
}
