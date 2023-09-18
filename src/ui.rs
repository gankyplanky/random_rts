use sdl2::render::WindowCanvas;
use sdl2::rect::Rect;

use super::{Sprite, Collidable, Building};

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

    pub fn render(&self, canvas: &mut WindowCanvas) {
        self.sprite.render(canvas);
    }
}

#[derive(Clone, Copy)]
pub struct Button<'b> {
    pub ui: UiElement<'b>,
    pub btn_function: ButtonFunction, 
}

impl<'b> Button<'b> {
    pub fn new(ui_elem: UiElement<'b>, btn_function: ButtonFunction) -> Button<'b> {
        let new_button = Button {
            ui: ui_elem,
            btn_function
        };

        return new_button;
    }

    pub fn render(&self, canvas: &mut WindowCanvas) {
        self.ui.render(canvas);
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ButtonFunction {
    ShowTier1Buildings,
    ShowTier2Buildings,
    MakeWorker,
    Back,
    MakeBarracks,
    MakeCC,
    PlaceConstruction,
    PlaceBarracks,
    PlaceCommandCentre,
}
