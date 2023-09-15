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
    pub selected_building: Option<usize>,
    pub selected_unit: Option<usize>,
    pub placing_building: bool,
}

impl<'p> Player<'p> {
    pub fn new(faction: Faction, texture_source: &'p Texture<'p>) -> Player<'p> {
        let mut new_p = Player {
            buildings: vec![],
            units: vec![],
            faction,
            selected_unit: None,
            selected_building: None,
            placing_building: false,
            top_right_ui: UiElement::new(Sprite::new(texture_source, Rect::new(0, 0, 64, 64), 
                Point::new(5, 5), 100, 100)),
            bottom_right_ui: vec![ // Bottom right UI containter
                UiElement::new(Sprite::new(texture_source, Rect::new(0, 0, 64, 64), 
                    Point::new(1920 - 280, 1080 - 280), 280, 280) ),
            ],
        };
        
        let mut i: i32 = 0;
        while i < 4 {
            let mut j: i32 = 0; // Bottom right button panels
            while j < 4 {
                new_p.bottom_right_ui.push( 
                    UiElement::new(Sprite::new(texture_source, Rect::new(64, 0, 64, 64), 
                        Point::new(1920 - 280 + 30 + j * 60, 1080 - 280 + 25 + i * 60),
                        50, 50) ),
                );
                j += 1;
            }
            i += 1;
        }

        return new_p;
    }
    
    fn get_building_buttons<'f>(&'f self) -> &'p [Option<Button>; 16] {
        let temp_buttons_panel_index = self.buildings[self.selected_building.unwrap()]
            .button_panel_index.to_owned();
        let buttons: &[Option<Button>; 16] = &self.buildings[self.selected_building.unwrap()]
            .buttons[temp_buttons_panel_index];
        return buttons; 
    }

    pub fn select_building<'f>(&'f mut self, index: usize) {
        self.selected_unit = None;
        self.selected_building = Some(index);
    }
    
    pub fn deselect<'f>(&'f mut self) {
        if self.selected_unit.is_some() {
            self.selected_unit = None;
        }

        if self.selected_building.is_some() {
            self.selected_building = None;
        }

        self.placing_building = false;
    }
    
    pub fn check_button<'f>(&'f mut self, point: Point, index: usize) -> bool {
        if self.selected_building.is_some() {
            let buttons = self.get_building_buttons();

            if buttons[index].is_some() {
                if buttons[index].unwrap().ui.collider.contains_point(point) {
                    let temp_player_clone = self.to_owned();
                    let temp_btn_fnc = buttons[index].unwrap().btn_function.to_owned();
                    let temp_building: &mut Building = &mut self.
                        buildings[self.selected_building.unwrap().to_owned()];
                    temp_building.execute_fn(temp_btn_fnc, temp_player_clone); 
                    return true;
                }  
            }
        }

        return false;
    } 

    pub fn render_ui<'f>(&'f self, canvas: &'f mut WindowCanvas) {
        let mut i: usize = 0;
        while i < self.bottom_right_ui.len() {
            self.bottom_right_ui[i].render(canvas);
            i += 1;
        }
        
        if self.selected_building.is_some() {
            let buttons = self.get_building_buttons();
            i = 0;
            while i < buttons.len() {
                if buttons[i].is_some() {
                    buttons[i].unwrap().render(canvas);
                }
                i += 1;
            }
        }
    }
}

