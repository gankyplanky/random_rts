use sdl2::render::WindowCanvas;

use crate::sprite::TextureManager;
use crate::general::{Selection, Selectable, Renderable};
use crate::ui::Button;

#[derive(Clone, Copy)]
pub struct Unit {

}

#[allow(dead_code, unused_variables)]
impl Unit {

}

#[allow(dead_code, unused_variables)]
impl Renderable for Unit {
    fn render<'f>(&'f self, tx_mgr: &'f TextureManager, canvas: &'f mut WindowCanvas) {
        unimplemented!()
    }

    fn get_loc_rect<'f>(&'f self) -> sdl2::rect::Rect {
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
