use sdl2::rect::{Point, Rect};
use sdl2::render::{Texture, WindowCanvas};

//Single sprite to render, used in higher abstractions for rendering
#[derive(Clone, Copy)]
pub struct Sprite<'s> {
    pub texture_source: &'s Texture<'s>,
    pub location: Point,
    pub texture_location: Rect,
    pub width: u32,
    pub height: u32,
    pub rect: Rect
}

impl<'s> Sprite<'s> {
    pub fn new(texture_source: &'s Texture, texture_location: Rect, 
            initial_location: Point, width: u32, height: u32) -> Sprite<'s> {
        let new_sprite = Sprite {
            texture_source,
            texture_location,
            location: initial_location,
            width,
            height,
            rect: Rect::new(initial_location.x, initial_location.y, width, height)
        };

        return new_sprite;
    }

    pub fn render(&self, canvas: &mut WindowCanvas) {
        canvas.copy(self.texture_source, self.texture_location,
            Rect::new(self.location.x, self.location.y, self.width, self.height))
            .expect("Failed to render texture");
    }
}
