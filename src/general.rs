use crate::ui::{ButtonFunction, Button, UiElement};
use crate::sprite::{Sprite, TextureManager, TextureType};
use sdl2::rect::Rect;
use strum_macros::Display;

pub const CCBUILD_TIME: u64 = 1000;
pub const BARRACKS_BUILD_TIME: u64 = 1000;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Collidable {
    GroundCollidable,
    GroundUncollidable,
    AirCollidable,
    AirUncollidable,
    AbsoluteCollidable,
    World,
    UI,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Display)]
pub enum Faction {
    PlaceholderFaction1,
}

pub fn gen_button<'f>(atlas: &'f TextureManager, bottom_right_ui: Vec<UiElement>,
        btn_function: ButtonFunction, order: usize) -> Option<Button> {
    
    Some(Button::new(UiElement::new(Sprite::new(
        Rect::new(
            bottom_right_ui[order + 1].collider.x,
            bottom_right_ui[order + 1].collider.y,
            50, 50),
        TextureType::UI { type_index: 0 }, atlas),
        btn_function.get_texture_index()), btn_function))
}

pub fn string_contains_any<'f>(str: String, options: Vec<String>) -> bool {
    for option in options {
        if str.contains(option.as_str()) {
            return true;
        }
    }

    return false;
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

