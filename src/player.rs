use sdl2::render::{WindowCanvas, Texture};
use sdl2::rect::{Point, Rect};

use super::{Sprite, Building, Faction, Unit, ui::{UiElement, Button}};

#[derive(Clone)]
pub struct Player<'p> {
    pub buildings: Vec<Building<'p>>,
    pub units: Vec<Unit>,
    pub faction: Faction,
    pub top_right_ui: UiElement<'p>,
    pub bottom_right_ui: Vec<UiElement<'p>>,
    pub bottom_right_buttons: [Option<Button<'p>>; 16] 
}

impl<'p> Player<'p> {
    pub fn new(faction: Faction, texture_source: &'p Texture<'p>) -> Player<'p> {
        let mut new_p = Player {
            buildings: vec![],
            units: vec![],
            faction,
            top_right_ui: UiElement::new(Sprite::new(texture_source, Rect::new(0, 0, 64, 64), 
                Point::new(5, 5), 100, 100)),
            bottom_right_ui: vec![ // Bottom right UI containter
                UiElement::new(Sprite::new(texture_source, Rect::new(0, 0, 64, 64), 
                    Point::new(1920 - 280, 1080 - 280), 280, 280) ),
            ],
            bottom_right_buttons: [None; 16],
        };
        
        let mut i: i32 = 0;
        while i < 4 {
            let mut j: i32 = 0; // Bottom right button panels
            while j < 4 {
                new_p.bottom_right_ui.push( 
                    UiElement::new(Sprite::new(texture_source, Rect::new(64, 0, 64, 64), 
                        Point::new(1920 - 280 + 30 + i * 60, 1080 - 280 + 25 + j * 60),
                        50, 50) ),
                );
                j += 1;
            }
            i += 1;
        }

        return new_p;
    }

    pub fn render_ui(&self, canvas: &mut WindowCanvas) {
        let mut i: usize = 0;
        while i < self.bottom_right_ui.len() {
            self.bottom_right_ui[i].render(canvas);
            i += 1;
        }

        i = 0;
        while i < self.bottom_right_buttons.len() {
            if self.bottom_right_buttons[i].is_some() {
                self.bottom_right_buttons[i].unwrap().render(canvas);
            }
            i += 1;
        }
    }
}

