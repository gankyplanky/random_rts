use sdl2::render::WindowCanvas;
use sdl2::rect::{Point, Rect};

use crate::building::{BuildingType, BuildingStatus};
use crate::general::{Selectable, Renderable};
use crate::sprite::{TextureManager, TextureType};
use crate::ui::{ButtonFunction, UIProperties, XAlignment, YAlignment, UIManager};
use crate::world::World;
use crate::{Sprite, Building, Faction, Unit, ui::{UiElement, Button}};
use crate::general::{self, Selection};

#[derive(Clone)]
pub struct Player {
    pub buildings: Vec<Building>,
    pub units: Vec<Unit>,
    pub faction: Faction,
    pub bottom_right_ui: Vec<UiElement>,
    pub selected: Selection,
    pub placing_building: bool,
    pub construction_buttons: [Option<Button>; 16], 
}

impl Player {
    // New Player
    pub fn new<'f>(faction: Faction, viewport: Rect, tx_mgr: &'f TextureManager, 
            ui_mgr: &'f mut UIManager) -> Player {
        let mut new_p = Player {
            buildings: vec![],
            units: vec![],
            faction,
            selected: Selection::None,
            construction_buttons: [None; 16],
            placing_building: false,
            bottom_right_ui: vec![UiElement::new(Sprite::new(
                Rect::new(0, 0, 280, 280),
                TextureType::UI { type_index: 1 }, 
                tx_mgr), 
                Some(UIProperties::new(
                    (XAlignment::Right, YAlignment::Bottom), 
                    0,
                    (0, 0, 0, 0),
                    0)),
                0)], // Bottom right UI container
        };
        
        ui_mgr.organize_simple(viewport, &mut new_p.bottom_right_ui);
        
        { // Bottom right button panels
            let mut i: i32 = 0;
            while i < 4 {
                let mut j: i32 = 0; 
                while j < 4 {
                    new_p.bottom_right_ui.push( 
                        UiElement::new(Sprite::new(
                            Rect::new(
                                new_p.bottom_right_ui[0].collider.x + j * 60 + 30,
                                new_p.bottom_right_ui[0].collider.y + i * 60 + 30,
                                50, 50), 
                            TextureType::UI { type_index: 1 }, 
                            tx_mgr ),
                            Some(UIProperties::new(
                                (XAlignment::Centre, YAlignment::Centre), 
                                0,
                                (0, 0, 0, 0),
                                1)),
                        1));
                    j += 1;
                }
                i += 1;
            }
        }

        new_p.construction_buttons[0] = general::gen_button(tx_mgr,
            new_p.bottom_right_ui.to_owned(), ButtonFunction::PlaceCommandCentre, 0);
        new_p.construction_buttons[1] = general::gen_button(tx_mgr,
            new_p.bottom_right_ui.to_owned(), ButtonFunction::PlaceBarracks, 1);
        new_p.construction_buttons[15] = general::gen_button(tx_mgr,
            new_p.bottom_right_ui.to_owned(), ButtonFunction::Back, 15);

        return new_p;
    }
    
    //Building Interactions
    
    pub fn check_completed_constructions<'f>(&'f mut self, tx_mgr: &'f TextureManager) {
        let mut completed_cons: Vec<Building> = vec![];

        self.buildings.iter_mut().filter(|building| building.construction_done())
            .for_each(|building| completed_cons.push(building.get_constructed(tx_mgr)));
        
        completed_cons.iter().for_each(|completed_con|
            self.buildings.push(completed_con.to_owned()));
    }

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
        for building in self.buildings.iter() {
            if building.place_construction_flag {
                return true;
            }
        }
        false
    }
    
    //General
    fn get_buttons<'f>(&'f self) -> Option<&[Option<Button>; 16]> {
        let mut buttons: Option<&[Option<Button>; 16]> = None;

        if self.selected.is_building() {
            buttons = Some(self.buildings[self.selected.index()].get_buttons());
        } else if self.selected.is_unit() {
            buttons = Some(self.units[self.selected.index()].get_buttons());
        } 

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

    pub fn render_owned<'f>(&'f self, tx_mgr: &'f TextureManager, canvas: &'f mut WindowCanvas) {
        self.buildings.iter().for_each(|b| {
            b.render(tx_mgr, canvas);
        });

        self.units.iter().for_each(|u| {
            u.render(tx_mgr, canvas);
        });
    }

    pub fn render_ui<'f>(&'f self, tx_mgr: &'f TextureManager, canvas: &'f mut WindowCanvas) {
        self.bottom_right_ui.iter().for_each(|ui| {
            ui.render(tx_mgr, canvas);
        });

        if self.check_place_construction_flag() {
            self.construction_buttons.iter()
                .filter(|btn| btn.is_some())
                .for_each(|btn| btn.unwrap().render(tx_mgr, canvas));
        
        } else if self.selected.is_some() {
            let buttons = self.get_buttons().unwrap();
            buttons.iter()
                .filter(|btn| btn.is_some())
                .for_each(|btn| btn.unwrap().render(tx_mgr, canvas));
        }
    }
}

