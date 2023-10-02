use sdl2::render::WindowCanvas;

use crate::sprite::TextureManager;
use crate::general::{Selection, Selectable};
use crate::ui::Button;

#[derive(Clone, Copy)]
pub struct Unit {

}

#[allow(dead_code, unused_variables)]
impl Unit {
    pub fn render<'f>(&'f self, canvas: &'f mut WindowCanvas, atlas: &'f TextureManager) {
        unimplemented!()
    }
}

#[allow(dead_code, unused_variables)]
impl Selectable for Unit {
    fn get_selection<'f>(&'f self, index: usize) -> Selection {
        unimplemented!()
    }

    fn get_buttons<'f>(&'f self) -> &[Option<Button>; 16] {
        unimplemented!()
    }
}
