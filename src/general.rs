use crate::ui::{ButtonFunction, Button, UiElement};
use crate::sprite::Sprite;
use sdl2::rect::{Rect, Point};
use sdl2::render::Texture;

pub const CCBUILD_TIME: u64 = 1000;
pub const BARRACKS_BUILD_TIME: u64 = 1000;
pub const SHOW_TIER1_BUILDINGS_INDEX: i32 = 0;
pub const SHOW_TIER2_BUILDINGS_INDEX: i32 = 1;
pub const PLACE_CONSTRUCTIN_INDEX: i32 = 6;
pub const BARRACKS_INDEX: i32 = 4;
pub const COMMAND_CENTRE_INDEX: i32 = 3;
pub const WORKER_INDEX: i32 = 5;
pub const BACK_INDEX: i32 = 2;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Collidable {
    GroundCollidable,
    GroundUncollidable,
    AirCollidable,
    AirUncollidable,
    AbsoluteCollidable,
    World,
    UI,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Faction {
    PlaceholderFaction1,
}

pub fn gen_button<'f>(button_texture: &'f Texture<'f>, texture_index: i32, order: usize, 
        bottom_right_ui: Vec<UiElement<'f>>, button_function: ButtonFunction) -> Option<Button<'f>> {
    Some(Button::new(UiElement::new(
        Sprite::new(button_texture, Rect::new(
                texture_index * 64, 0, 64, 64), 
                Point::new(bottom_right_ui[order + 1].sprite.location.x, 
                    bottom_right_ui[order + 1].sprite.location.y),
            50, 50)
        ),
        button_function
    ))
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Selection {
    Building(usize),
    Unit(usize),
    None,
}

impl Selection {
    pub fn is_some<'f>(&'f self) -> bool {
        if self.to_owned() == Selection::None {
            false
        } else {
            true
        }
    }
    
    pub fn is_building<'f>(&'f self) -> bool {
        match self {
            Selection::Building( .. ) => {
                true
            },
            _ => { false }
        }
    }

    pub fn is_unit<'f>(&'f self) -> bool {
        match self {
            Selection::Unit( .. ) => {
                true
            },
            _ => { false }
        }
    }
    
    pub fn is_type<'f>(&'f self, selection_type: Selection) -> bool {
        self.to_owned() == selection_type
    }

    pub fn index<'f>(&'f self) -> usize {
        match self {
            Selection::Unit(index) => {
                index.to_owned()
            },
            Selection::Building(index) => {
                index.to_owned()
            }
            Self::None => { 
                panic!("Selection can't return index of None");
            }
        }
    }
}

pub trait Selectable {
    fn get_selection<'f>(&'f self, index: usize) -> Selection;
    fn get_buttons<'f>(&'f self) -> &[Option<Button>; 16];
}

