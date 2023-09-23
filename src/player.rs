use sdl2::render::{WindowCanvas, Texture};
use sdl2::rect::{Point, Rect};

use crate::building::{BuildingType, BuildingStatus};
use crate::ui::ButtonFunction;
use crate::world::World;

use super::{Sprite, Building, Faction, Unit, ui::{UiElement, Button}};
use super::general;

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
    pub construction_buttons: [Option<Button<'p>>; 16], 
}

impl<'p> Player<'p> {
    // New Player
    pub fn new(faction: Faction, texture_source: &'p Texture<'p>,
            button_texture: &'p Texture<'p>) -> Player<'p> {
        let mut new_p = Player {
            buildings: vec![],
            units: vec![],
            faction,
            selected_unit: None,
            selected_building: None,
            construction_buttons: [None; 16],
            placing_building: false,
            top_right_ui: UiElement::new(Sprite::new(texture_source, Rect::new(0, 0, 64, 64), 
                Point::new(5, 5), 100, 100)),
            bottom_right_ui: vec![ // Bottom right UI containter
                UiElement::new(Sprite::new(texture_source, Rect::new(0, 0, 64, 64), 
                    Point::new(1920 - 280, 1080 - 280), 280, 280) ),
            ],
        };
        
        {
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
        }

        new_p.construction_buttons[0] = Some(Button::new(UiElement::new(
            Sprite::new(button_texture, Rect::new(
                    general::COMMAND_CENTRE_INDEX * 64, 0, 64, 64), 
                Point::new(new_p.bottom_right_ui[1].sprite.location.x, 
                    new_p.bottom_right_ui[1].sprite.location.y),
                50, 50)
            ),
            crate::ui::ButtonFunction::PlaceCommandCentre
        ));
        new_p.construction_buttons[1] = Some(Button::new(UiElement::new(
            Sprite::new(button_texture, Rect::new(
                    general::BARRACKS_INDEX * 64, 0, 64, 64), 
                Point::new(new_p.bottom_right_ui[2].sprite.location.x, 
                    new_p.bottom_right_ui[2].sprite.location.y),
                50, 50)
            ),
            crate::ui::ButtonFunction::PlaceBarracks
        ));
        new_p.construction_buttons[15] = Some(Button::new(UiElement::new(
            Sprite::new(button_texture, Rect::new(
                    general::BACK_INDEX * 64, 0, 64, 64), 
                Point::new(new_p.bottom_right_ui[16].sprite.location.x, 
                    new_p.bottom_right_ui[16].sprite.location.y),
                50, 50)
            ),
            crate::ui::ButtonFunction::Back
        ));
        
        return new_p;
    }
    
    //Building Interactions
    fn get_building_buttons<'f>(&'f self) -> &'p [Option<Button>; 16] {
        let temp_buttons_panel_index = self.buildings[self.selected_building.unwrap()]
            .button_panel_index.to_owned();
        let buttons: &[Option<Button>; 16] = &self.buildings[self.selected_building.unwrap()]
            .buttons[temp_buttons_panel_index];
        return buttons; 
    }
    
    pub fn place_building<'f>(&'f mut self, game_map: &'f mut World) {
        let index = self.selected_building.unwrap().to_owned();
        let cell_x = self.buildings[index].x_in_cells();
        let cell_y = self.buildings[index].y_in_cells();
        let w_cells = self.buildings[index].width_in_cells();
        let h_cells = self.buildings[index].height_in_cells();
         
        let mut i: i32 = 0;
        while i < w_cells {
            let mut j: i32 = 0;
            while j < h_cells {
                let ii = (cell_x + i) as usize;
                let jj = (cell_y + j) as usize;
                if game_map.grid[ii][jj].occupied {
                    return;
                }
                j += 1;
            }
            i += 1;
        }
        
        i = 0;
        while i < w_cells {
            let mut j: i32 = 0;
            while j < h_cells {
                let ii = (cell_x + i) as usize;
                let jj = (cell_y + j) as usize;
                game_map.grid[ii][jj].occupy();

                j += 1;
            }

            i += 1;
        }
          
        self.buildings[index].status = BuildingStatus::Built;
        self.deselect();

    }

    pub fn start_placing_building<'f>(&'f mut self, action_type: ButtonFunction) {
        let building_type: BuildingType;
        match action_type {
            ButtonFunction::PlaceBarracks => {
                building_type = BuildingType::Barracks;
            },
            ButtonFunction::PlaceCommandCentre => {
                building_type = BuildingType::CommandCentre;
            },
            ButtonFunction::Back => {
                self.deselect();
                return;
            },
            _ => { 
                println!("------Error when selecting building type for placement!!");
                self.deselect();
                return;
            }
        }

        {
            let mut i: usize = 0;
            while i < self.buildings.len() {
                if self.buildings[i].status == BuildingStatus::NotBuilt {
                    if self.buildings[i].building_type == building_type {
                        self.deselect();
                        self.selected_building = Some(i);
                        self.buildings[i].status = BuildingStatus::Placing;
                        self.placing_building = true;
                        break;
                    }
                }       

                i += 1;
            }
        }
    }
    
    pub fn check_place_construction_flag<'f>(&'f self) -> bool {
        let mut i: usize = 0;
        while i < self.buildings.len() {
            if self.buildings[i].place_construction_flag {
                return true;
            }

            i += 1;
        }
        return false;
    }
    
    //General
    pub fn select_building<'f>(&'f mut self, index: usize) {
        self.selected_unit = None;
        self.selected_building = Some(index);
    }
    
    pub fn dehighlight<'f>(&'f mut self, game_map: &'f mut World) {
        if self.selected_building.is_some() {
            let index = self.selected_building.unwrap().to_owned();
            self.buildings[index].dehighlight_cells(game_map);
        }
    }

    pub fn deselect<'f>(&'f mut self) {
        if self.selected_unit.is_some() {
            self.selected_unit = None;
        }

        if self.selected_building.is_some() {
            let index = self.selected_building.unwrap().to_owned();
            self.buildings[index].button_panel_index = 0;
            self.buildings[index].place_construction_flag = false;
            self.selected_building = None;
        }

        self.placing_building = false;
    }
    
    pub fn check_button<'f>(&'f mut self, point: Point, index: usize) -> bool {
        if self.check_place_construction_flag() {
            if self.construction_buttons[index].is_some() {
                if self.construction_buttons[index].unwrap().ui.collider.contains_point(point) {
                    let temp_btn_fn = self.construction_buttons[index]
                        .unwrap().to_owned().btn_function;
                    self.start_placing_building(temp_btn_fn);
                }
            }
        } else if self.selected_building.is_some() {
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
        
        if self.check_place_construction_flag() {
            i = 0;
            while i < self.construction_buttons.len() {
                if self.construction_buttons[i].is_some() {
                    self.construction_buttons[i].unwrap().render(canvas);
                }
                i += 1;
            }
        } else if self.selected_building.is_some() {
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

