use sdl2::render::WindowCanvas;
use sdl2::rect::Rect;

use crate::{Sprite, Collidable, sprite::TextureManager};

#[derive(Clone, Copy)]
pub struct UiElement {
    pub sprite: Sprite,
    pub collider_type: Collidable,
    pub collider: Rect,
}

impl UiElement {
    pub fn new<'f>(mut sprite: Sprite, texture_rect_order: i32) -> UiElement {
        
        let collider = sprite.loc_rect.to_owned();
        sprite.texture_rect.x = texture_rect_order * 64;

        UiElement {
            collider_type: Collidable::UI,
            sprite,
            collider 
        }
    }

    pub fn render<'f>(&'f self, canvas: &'f mut WindowCanvas, atlas: &'f TextureManager) {
        self.sprite.render(atlas, canvas);
    }
}

#[derive(Clone, Copy)]
pub struct Button {
    pub ui: UiElement,
    pub btn_function: ButtonFunction, 
}

impl Button {
    pub fn new<'f>(ui_elem: UiElement, btn_function: ButtonFunction) -> Button {
        Button {
            ui: ui_elem,
            btn_function
        }
    }

    pub fn render<'f>(&'f self, canvas: &'f mut WindowCanvas, atlas: &'f TextureManager) {
        self.ui.render(canvas, atlas);
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
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

#[allow(unreachable_patterns)]
impl ButtonFunction {
    pub fn get_texture_index<'f>(&'f self) -> i32 {
        match self {
            ButtonFunction::ShowTier1Buildings => { 0 },
            ButtonFunction::ShowTier2Buildings => { 1 },
            ButtonFunction::MakeWorker => { 5 },
            ButtonFunction::Back => { 2 },
            ButtonFunction::MakeBarracks => { 4 },
            ButtonFunction::MakeCC => { 3 },
            ButtonFunction::PlaceBarracks => { 4 },
            ButtonFunction::PlaceCommandCentre => { 3 },
            ButtonFunction::PlaceConstruction => { 6 },
            _ => { unimplemented!() }
        }
    }
}
