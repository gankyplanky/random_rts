use sdl2::render::WindowCanvas;
use sdl2::rect::Rect;

use super::{Sprite, Collidable};

#[derive(Clone, Copy)]
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

#[derive(Clone, Copy)]
pub struct Button<'b> {
    pub ui: UiElement<'b>,
    pub click_func: fn()
}

impl<'b> Button<'b> {
    pub fn new(ui_elem: UiElement<'b>, click_func: fn()) -> Button<'b> {
        let new_button = Button {
            ui: ui_elem,
            click_func
        };

        return new_button;
    }

    pub fn render(&mut self, canvas: &mut WindowCanvas) {
        self.ui.render(canvas);
    }

    pub fn click(&mut self) {
        (self.click_func)();
    }
}
