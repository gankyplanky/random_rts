use sdl2::render::{WindowCanvas, Texture};
use sdl2::rect::{Point, Rect};

use crate::building::{BuildingType, BuildingStatus};
use crate::general::Selectable;
use crate::ui::ButtonFunction;
use crate::world::World;

use super::{Sprite, Building, Faction, Unit, ui::{UiElement, Button}};
use super::general::{self, Selection};

#[derive(Clone)]
pub struct Player<'p> {
    pub buildings: Vec<Building<'p>>,
    pub units: Vec<Unit>,
    pub faction: Faction,
    pub top_right_ui: UiElement<'p>,
    pub bottom_right_ui: Vec<UiElement<'p>>,
    pub selected: Selection,
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
            selected: Selection::None,
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

        new_p.construction_buttons[0] = general::gen_button(button_texture,
            general::COMMAND_CENTRE_INDEX, 0, new_p.bottom_right_ui.to_owned(),
            ButtonFunction::PlaceCommandCentre);
        
        new_p.construction_buttons[1] = general::gen_button(button_texture,
            general::BARRACKS_INDEX, 1, new_p.bottom_right_ui.to_owned(),
            ButtonFunction::PlaceBarracks);
        
        new_p.construction_buttons[15] = general::gen_button(button_texture,
            general::BACK_INDEX, 15, new_p.bottom_right_ui.to_owned(),
            ButtonFunction::Back);
        
        return new_p;
    }
    
    //Building Interactions
    
    pub fn place_building<'f>(&'f mut self, game_map: &'f mut World) {
        let index = self.selected.index();
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
                        self.selected = Selection::Building(i);
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
    fn get_buttons<'f>(&'f self) -> Option<&'p [Option<Button>; 16]> {
        let mut buttons: Option<&[Option<Button>; 16]> = None;

        if self.selected.is_building() {
            buttons = Some(self.buildings[self.selected.index()].get_buttons());
        } /*else if self.selected.is_unit() {
            buttons = self.units[self.selected.index()].get_buttons();
        } */

        return buttons; 
    }
    
    fn get_selectables<'f>(&'f self) -> Vec<Selection> {
        let mut selectables: Vec<Selection> = vec![];
        let mut i: usize = 0;
        while i < self.buildings.len() {
            selectables.push(self.buildings[i].get_selection(i));
            i += 1;
        }
        /*i = 0;
        while i < self.units.len() {
            selectables.push(self.units[i].get_selection(i));
            i += 1;
        }*/
        selectables
    }
    
    fn check_selecting_click<'f>(&'f self, selection: Selection, click: Point) -> bool {
        match selection {
            Selection::Building(index) => {
                self.buildings[index].collider.contains_point(click)
            },
            /*Selection::Unit(index) => {
                self.units[index].collider.contains_point(click)
            },*/
            _ => { false }
        }
    }
    
    pub fn try_selecting<'f>(&'f mut self, click: Point) -> bool {
        for selectable in self.get_selectables() {
            if self.check_selecting_click(selectable, click) {
                self.selected = selectable;
                return true;
            }
        }

        false
    }

    pub fn dehighlight<'f>(&'f mut self, game_map: &'f mut World) {
        if self.selected.is_building() {
            let index = self.selected.index();
            self.buildings[index].dehighlight_cells(game_map);
        }
    }

    pub fn deselect<'f>(&'f mut self) {
        if self.selected.is_unit() {
            self.selected = Selection::None;
        }

        if self.selected.is_building() {
            let index = self.selected.index();
            self.buildings[index].button_panel_index = 0;
            self.buildings[index].place_construction_flag = false;
            self.selected = Selection::None;
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
        } else if self.selected.is_building() {
            let buttons = self.get_buttons().unwrap();

            if buttons[index].is_some() {
                if buttons[index].unwrap().ui.collider.contains_point(point) {
                    let temp_player_clone = self.to_owned();
                    let temp_btn_fnc = buttons[index].unwrap().btn_function.to_owned();
                    let temp_building: &mut Building = &mut self.
                        buildings[self.selected.index()];
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
        } else if self.selected.is_some() {
            let buttons = self.get_buttons().unwrap();
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

