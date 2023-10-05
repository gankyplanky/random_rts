use crate::ui::{ButtonFunction, Button, UiElement, UIProperties, XAlignment, YAlignment};
use crate::sprite::{Sprite, TextureManager, TextureType};

use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use strum_macros::Display;

use std::cmp::max;

pub const CCBUILD_TIME: u64 = 1000;
pub const BARRACKS_BUILD_TIME: u64 = 1000;

#[allow(dead_code)]
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
        Some(UIProperties::new(
            (XAlignment::None, YAlignment::None), 
            0, 
            (0, 0, 0, 0), 
            get_max_z(&bottom_right_ui) + 1)),
        btn_function.get_texture_index()),
        btn_function))
}

fn get_max_z<'f>(elems: &'f Vec<UiElement>) -> usize {
    let mut max_z: usize = 0;
    elems.iter()
        .filter(|elem| 
            elem.props.is_some())
        .for_each(|elem| max_z = max(max_z, elem.props.unwrap().z_layer));
    max_z
}

#[allow(dead_code)]
pub fn string_contains_any<'f>(str: String, options: Vec<String>) -> bool {
    for option in options {
        if str.contains(option.as_str()) {
            return true;
        }
    }

    return false;
}

#[allow(dead_code)]
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
    
    #[allow(dead_code)]
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

pub trait Renderable {
    fn render<'f>(&'f self, tx_mgr: &'f TextureManager, canvas: &'f mut WindowCanvas);
    
    #[allow(unused_variables)]
    fn render_with_custom<'f>(&'f self, tx_mgr: &'f TextureManager,
        canvas: &'f mut WindowCanvas, loc: Option<Rect>, t_loc: Option<Rect>) {

        self.render(tx_mgr, canvas);
    }

    fn get_loc_rect<'f>(&'f self) -> Rect {
        unimplemented!()
    }
    
    #[allow(unused_variables)]
    fn set_loc_rect<'f>(&'f mut self, new: Rect) {
        unimplemented!()
    }
}

pub trait Selectable {
    fn get_selection<'f>(&'f self, index: usize) -> Selection;
    fn get_buttons<'f>(&'f self) -> &[Option<Button>; 16];
}

