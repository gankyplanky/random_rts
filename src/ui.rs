use sdl2::render::WindowCanvas;
use sdl2::rect::Rect;

use super::{Sprite, Collidable};

pub struct UiElement<'u> {
    pub sprite: Sprite<'u>,
    pub collider_type: Collidable,
    pub collider: Rect,
}

impl<'p> UiElement<'p> {
    pub fn new(sprite: Sprite<'p>) -> UiElement<'p> {
        let temp = sprite.clone();
        let collider = Rect::new(temp.location.x, temp.location.y, temp.width, temp.height);
        let new_ui = UiElement {
            collider_type: Collidable::UI,
            sprite,
            collider 
        };

        return new_ui;
    }

    pub fn render(&mut self, canvas: &mut WindowCanvas) {
        self.sprite.render(canvas);
    }
}

