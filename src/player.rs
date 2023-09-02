use sdl2::render::{WindowCanvas, Texture};
use sdl2::rect::{Point, Rect};

use super::{Sprite, Building, Faction, Unit, ui::UiElement};

pub struct Player<'p> {
    pub buildings: Vec<Building<'p>>,
    pub units: Vec<Unit>,
    pub faction: Faction,
    pub top_right_ui: UiElement<'p>,
    pub bottom_left_ui: Vec<UiElement<'p>>
}

impl<'p> Player<'p> {
    pub fn new(faction: Faction, texture_source: &'p Texture<'p>) -> Player<'p> {
        let mut new_p = Player {
            buildings: vec![],
            units: vec![],
            faction,
            top_right_ui: UiElement::new(Sprite::new(texture_source, Rect::new(0, 0, 64, 64), 
                Point::new(5, 5), 100, 100)),
            bottom_left_ui: vec![
                UiElement::new(Sprite::new(texture_source, Rect::new(0, 0, 64, 64), 
                    Point::new(1920 - 280, 1080 - 280), 280, 280) ),
            ],
        };
        
        let mut i: i32 = 0;
        while i < 4 {
            let mut j: i32 = 0;
            while j < 4 {
                new_p.bottom_left_ui.push(
                    UiElement::new(Sprite::new(texture_source, Rect::new(64, 0, 64, 64), 
                        Point::new(1920 - 280 + 30 + i * 60, 1080 - 280 + 25 + j * 60),
                        50, 50) ),
                );
                j += 1;
            }
            i += 1;
        }
        new_p.bottom_left_ui.push(
            UiElement::new(Sprite::new(texture_source, Rect::new(128, 0, 64, 64), 
                Point::new(1920 - 280 + 30, 1080 - 280 + 25),
                50, 50) )
        );

        return new_p;
    }

    pub fn render_ui(&mut self, canvas: &mut WindowCanvas) {
        let mut i: usize = 0;
        while i < self.bottom_left_ui.len() {
            self.bottom_left_ui[i].render(canvas);
            i += 1;
        }
    }
}

