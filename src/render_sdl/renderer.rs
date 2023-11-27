use sdl2::render::Canvas;

use crate::render::{Renderable, Renderer};

pub(crate) struct SdlRenderer;

impl Renderer<Canvas<sdl2::video::Window>> for SdlRenderer {
    fn render(
        &self,
        target: &mut Canvas<sdl2::video::Window>,
        renderables: &[&dyn Renderable<Canvas<sdl2::video::Window>>],
    ) -> Result<(), String> {
        for r in renderables {
            r.render(target)?
        }
        target.present();
        Ok(())
    }
}
