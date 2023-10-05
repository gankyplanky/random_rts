use sdl2::render::WindowCanvas;
use sdl2::rect::Rect;

use crate::{Sprite, Collidable, sprite::TextureManager};
use crate::general::Renderable;

use std::cmp::max;

#[derive(Clone, Copy)]
pub struct UiElement {
    pub sprite: Sprite,
    pub collider_type: Collidable,
    pub collider: Rect,
    pub props: Option<UIProperties>,
}

impl UiElement {
    pub fn new<'f>(mut sprite: Sprite, props: Option<UIProperties>, 
            texture_rect_order: i32) -> UiElement {
        
        let collider = sprite.loc_rect.to_owned();
        sprite.texture_rect.x = texture_rect_order * 64;

        UiElement {
            collider_type: Collidable::UI,
            sprite,
            collider,
            props,
        }
    }
}

impl Renderable for UiElement {
    fn render<'f>(&'f self, tx_mgr: &'f TextureManager, canvas: &'f mut WindowCanvas) {
        self.sprite.render(tx_mgr, canvas);
    }

    fn get_loc_rect<'f>(&'f self) -> Rect {
        self.sprite.get_loc_rect()
    }

    fn set_loc_rect<'f>(&'f mut self, new: Rect) {
        self.collider = new;
        self.sprite.set_loc_rect(new);
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
}

impl Renderable for Button {
    fn render<'f>(&'f self, tx_mgr: &'f TextureManager, canvas: &'f mut WindowCanvas) {
        self.ui.render(tx_mgr, canvas);
    }

    fn get_loc_rect<'f>(&'f self) -> Rect {
        self.ui.get_loc_rect()
    }

    fn set_loc_rect<'f>(&'f mut self, new: Rect) {
        self.ui.set_loc_rect(new);
    }
}

pub struct UIManager {
    viewport: Rect,
}

impl UIManager {
    pub fn new<'f>(viewport: Rect) -> UIManager {
        UIManager {
            viewport 
        }
    }

    pub fn set_viewport<'f>(&'f mut self, new: Rect) {
        self.viewport = new;
    }

    fn get_max_z<'f>(&'f self, elems: &'f mut Vec<UiElement>) -> usize {
        let mut max_z: usize = 0;
        elems.iter()
            .filter(|elem| 
                self.viewport.contains_rect(elem.get_loc_rect()) &&
                elem.props.is_some())
            .for_each(|elem| max_z = max(max_z, elem.props.unwrap().z_layer));
        max_z
    }

    fn get_x<'f>(&'f self, props: UIProperties, elem_rect: Rect) -> i32 {
        match props.x_align {
            XAlignment::Left => { 0 + props.margin + props.l_padding },
            XAlignment::Right => {
                self.viewport.w - elem_rect.w - props.margin - props.r_padding
            },
            XAlignment::Centre => {
                ((self.viewport.w >> 1) - (elem_rect.w >> 1)) + 
                    props.l_padding - props.r_padding + (props.margin >> 1) 
            },
            XAlignment::None => { elem_rect.x }
        }
    }
    
    fn get_y<'f>(&'f self, props: UIProperties, elem_rect: Rect) -> i32 {
        match props.y_align {
            YAlignment::Top => { 0 + props.margin + props.t_padding },
            YAlignment::Bottom => { 
                self.viewport.h - elem_rect.h - props.margin - props.b_padding 
            },
            YAlignment::Centre => {
                ((self.viewport.h >> 1) - (elem_rect.h >> 1)) + 
                    props.t_padding - props.b_padding + (props.margin >> 1) 
            },
            YAlignment::None => { elem_rect.y }
        }
    }

    pub fn organize_simple<'f>(&'f mut self, context: Rect, elems: &'f mut Vec<UiElement>) { 
        self.set_viewport(context);
        
        let max_z = self.get_max_z(elems) + 1;

        for cur_z in 0..max_z {
            elems.iter_mut()
                .filter(|elem| 
                    self.viewport.contains_rect(elem.get_loc_rect()) && 
                    elem.props.is_some())
                .filter(|elem| cur_z == elem.props.unwrap().z_layer)
                .for_each(|elem| {
                    let props = elem.props.unwrap().to_owned();
                    let temp_rect = Rect::new(
                        self.get_x(props, elem.get_loc_rect()), 
                        self.get_y(props, elem.get_loc_rect()),
                        elem.get_loc_rect().width(), 
                        elem.get_loc_rect().height());
                    elem.set_loc_rect(temp_rect);
                });
        }
    }
}

#[derive(Clone, Copy)]
pub struct UIProperties {
    pub x_align: XAlignment,
    pub y_align: YAlignment,
    pub margin: i32,
    pub t_padding: i32,
    pub b_padding: i32,
    pub l_padding: i32,
    pub r_padding: i32,
    pub z_layer: usize,
}

impl UIProperties {
    pub fn new<'f>(align: (XAlignment, YAlignment), margin: i32, 
            padding: (i32, i32, i32, i32), z_layer: usize) -> UIProperties {
        UIProperties { 
            x_align: align.0,
            y_align: align.1,
            margin,
            t_padding: padding.0,
            b_padding: padding.1,
            l_padding: padding.2,
            r_padding: padding.3,
            z_layer
        }
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub enum XAlignment {
    Left,
    Right,
    Centre,
    None
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub enum YAlignment {
    Top,
    Bottom,
    Centre,
    None
}

#[allow(dead_code)]
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
