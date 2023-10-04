use std::time::Duration;
use std::cmp::{max, min};

use stopwatch::Stopwatch;
use sdl2::rect::{Point, Rect};
use sdl2::render::WindowCanvas;
use strum::IntoEnumIterator;
use strum_macros::{EnumIter, Display};

use crate::general::{Selectable, Renderable};
use crate::sprite::{TextureType, TextureManager};
use crate::world::{World, Cell};

use super::sprite::Sprite;
use super::general::{self, Collidable, Faction};
use super::ui::{Button, UiElement, ButtonFunction};
use super::Player;

#[derive(Clone)]
pub struct Building {
    pub sprite: Sprite,
    pub team: i32,
    pub building_type: BuildingType,
    pub faction: Faction,
    pub collider_type: Collidable,
    pub collider: Rect,
    pub buttons: Vec<[Option<Button>; 16]>,
    pub button_panel_index: usize,
    pub button_panel_limit: usize,
    pub constructing: Option<Construction>,
    pub status: BuildingStatus,
    pub place_construction_flag: bool,
}

impl Building {
    pub fn new<'f>(location: Point, building_type: BuildingType, faction: Faction,
            team: i32, bottom_right_ui: Vec<UiElement>,
            atlas: &'f TextureManager) -> Building {
        
        let mut new_building = Building {
            team,
            building_type,
            faction,
            buttons: vec![],
            button_panel_index: 0,
            button_panel_limit: 1,
            place_construction_flag: false,
            status: BuildingStatus::NotBuilt,
            collider_type: Collidable::GroundCollidable,
            collider: Rect::new(location.x, location.y,
                building_type.get_w(), building_type.get_h()),
            constructing: None,
            sprite: {
                Sprite::new(
                    Rect::new(location.x, location.y,
                        building_type.get_w(), building_type.get_h()),
                    TextureType::Building { faction, b_type: building_type },
                    atlas)
            } 
        };
        
        Building::init_buttons(&mut new_building, bottom_right_ui, atlas);

        return new_building;
    }

    fn init_buttons<'f>(building: &'f mut Building, bottom_right_ui: Vec<UiElement>,
            atlas: &'f TextureManager) {

        match building.faction {
            Faction::PlaceholderFaction1 => {
                match building.building_type {
                    BuildingType::CommandCentre => {
                        building.button_panel_limit = 3;
                        building.buttons.push([None; 16]);
                        building.buttons.push([None; 16]);
                        building.buttons.push([None; 16]);
                        
                        building.buttons[0][0] = general::gen_button(atlas,
                            bottom_right_ui.to_owned(), ButtonFunction::ShowTier1Buildings, 0);
                        building.buttons[0][1] = general::gen_button(atlas,
                            bottom_right_ui.to_owned(), ButtonFunction::ShowTier2Buildings, 1);
                        building.buttons[0][15] = general::gen_button(atlas,
                            bottom_right_ui.to_owned(), ButtonFunction::PlaceConstruction, 15);
                       
                        building.buttons[1][0] = general::gen_button(atlas,
                            bottom_right_ui.to_owned(), ButtonFunction::MakeCC, 0);
                        building.buttons[1][1] = general::gen_button(atlas,
                            bottom_right_ui.to_owned(), ButtonFunction::MakeBarracks, 1);
                        building.buttons[1][15] = general::gen_button(atlas,
                            bottom_right_ui.to_owned(), ButtonFunction::Back, 15);

                        building.buttons[2][15] = general::gen_button(atlas,
                            bottom_right_ui.to_owned(), ButtonFunction::Back, 15);
                    },
                    BuildingType::Barracks => {
                        building.button_panel_limit = 1;
                        building.buttons.push([None; 16]);

                    }
                }
            }
        } 
    }
    
    pub fn _get_texture_type<'f>(&'f self) -> TextureType {
        TextureType::Building { faction: self.faction, b_type: self.building_type }
    }

    pub fn width_in_cells<'f>(&'f self) -> i32 {
        self.collider.w / 25
    }

    pub fn height_in_cells<'f>(&'f self) -> i32 {
        self.collider.h / 25
    }
    
    pub fn x_in_cells<'f>(&'f self) -> i32 {
        self.collider.x / 25
    }

    pub fn y_in_cells<'f>(&'f self) -> i32 {
        self.collider.y / 25
    }

    pub fn execute_fn<'f>(&'f mut self, function: ButtonFunction, owner: Player) {
        match function {
            ButtonFunction::ShowTier1Buildings => {
                self.set_button_panel(1); 
            },
            ButtonFunction::ShowTier2Buildings => {
                self.set_button_panel(2);
            },
            ButtonFunction::Back => {
                self.set_button_panel(0);
            },
            ButtonFunction::MakeWorker => {
                unimplemented!();
            },
            ButtonFunction::MakeBarracks => {
                self.start_construction(BuildingType::Barracks,
                    owner.bottom_right_ui.to_owned());
            },
            ButtonFunction::MakeCC => {
                self.start_construction(BuildingType::CommandCentre,
                    owner.bottom_right_ui.to_owned());
            },
            ButtonFunction::PlaceConstruction => {
                self.place_construction_flag = true; 
            },
            _ => {}
        } 
    }
    
    pub fn construction_done<'f>(&'f mut self) -> bool {
        let temp_con: &'f mut Construction = self.constructing.as_mut().unwrap();
        let check = temp_con.check_timer();
        check
    }
    
    pub fn get_constructed<'f>(&'f self, atlas: &'f TextureManager) -> Option<Building> {
        if self.constructing.is_some() {
            let temp_con: &'f Construction = self.constructing.as_ref().unwrap();
            if temp_con.finished {
                return Some(temp_con.build_building(atlas));
            }
        }

        None
    }

    fn set_button_panel<'f>(&'f mut self, index: usize) {
        self.button_panel_index = index;
    }
   
    pub fn move_building<'f>(&'f mut self, mouse_point: Point, cam_viewport: Rect, 
            grid: &'f mut Vec<Vec<Cell>>) {
        let mut world_pos: Point = Point::new(
            cam_viewport.x + mouse_point.x - self.collider.w + 25,
            cam_viewport.y + mouse_point.y - self.collider.h + 25 
        );

        world_pos.x = min(world_pos.x,
            (grid.len() - 2) as i32 * 25 - self.width_in_cells() * 25);
        world_pos.y = min(world_pos.y,
            grid[0].len() as i32 * 25 - self.height_in_cells() * 25);

        world_pos.x = max(world_pos.x, 0);
        world_pos.y = max(world_pos.y, 0);

        world_pos.x = world_pos.x - (world_pos.x % 25);
        world_pos.y = world_pos.y - (world_pos.y % 25);

        self.sprite.set_location(world_pos);
        self.collider.x = world_pos.x;
        self.collider.y = world_pos.y;
    }
    
    pub fn highlight_cells<'f>(&'f self, game_map: &'f mut World) {
        let cell_x = self.x_in_cells();
        let cell_y = self.y_in_cells();
        let w_cells = self.width_in_cells();
        let h_cells = self.height_in_cells();

        let mut i: i32 = 0;
        while i < w_cells {
            let mut j: i32 = 0;
            while j < h_cells {
                let ii = (cell_x + i) as usize;
                let jj = (cell_y + j) as usize;
                game_map.grid[ii][jj].highlight();
                j += 1;
            }
            i += 1;
        }
    }

    pub fn dehighlight_cells<'f>(&'f self, game_map: &'f mut World) {
        let cell_x = self.x_in_cells();
        let cell_y = self.y_in_cells();
        let w_cells = self.width_in_cells();
        let h_cells = self.height_in_cells();

        let mut i: i32 = 0;
        while i < w_cells {
            let mut j: i32 = 0;
            while j < h_cells {
                let ii = (cell_x + i) as usize;
                let jj = (cell_y + j) as usize;
                game_map.grid[ii][jj].dehighlight();
                j += 1;
            }
            i += 1;
        }
    }

    fn start_construction<'f>(&'f mut self, building_type: BuildingType,
        bottom_right_ui: Vec<UiElement>) {
        
        self.constructing = Some(Construction::new(building_type, self.faction,
            self.team, building_type.get_build_time(), bottom_right_ui));
    }
}

impl Renderable for Building {
    fn render<'f>(&'f self, tx_mgr: &'f TextureManager, canvas: &'f mut WindowCanvas) {
        match self.status {
            BuildingStatus::Built => {
                self.sprite.render(tx_mgr, canvas);
            },
            BuildingStatus::Placing => {
                let mut temp_rect = self.sprite.texture_rect;
                temp_rect.x += 128;
                self.sprite.render_with_custom(tx_mgr, canvas, None, Some(temp_rect));
            },
            BuildingStatus::NotBuilt => {}
        }
    }

    fn get_loc_rect<'f>(&'f self) -> Rect {
        self.sprite.get_loc_rect()
    }
}

impl Selectable for Building {
    fn get_selection<'f>(&'f self, index: usize) -> general::Selection {
        general::Selection::Building(index)
    }

    fn get_buttons<'f>(&'f self) -> &[Option<Button>; 16] {
        &self.buttons[self.button_panel_index]
    }
}

#[derive(Clone)]
pub struct Construction {
    pub building_type: BuildingType,
    pub faction: Faction,
    pub team: i32,
    pub bottom_right_ui: Vec<UiElement>,
    pub timer: Stopwatch,
    pub timer_end:  Duration,
    pub finished: bool,
}

impl Construction {
    pub fn new<'f>(building_type: BuildingType, faction: Faction, team: i32,
            timer_end: Duration, bottom_right_ui: Vec<UiElement>) -> Construction {
        
        let mut new_construction = Construction {
            building_type,
            faction,
            team,
            bottom_right_ui,
            timer: Stopwatch::new(),
            timer_end,
            finished: false,
        };
        
        new_construction.timer.start();

        return new_construction;
    }

    pub fn check_timer<'f>(&'f mut self) -> bool {
        if self.timer.elapsed().as_millis() >= self.timer_end.as_millis() {
            self.timer.stop();
            self.finished = true;
            return true;
        }
        return false;
    }

    pub fn build_building<'f>(&'f self, atlas: &'f TextureManager) -> Building {
        Building::new(Point::new(0, 0), self.building_type, self.faction, self.team,
            self.bottom_right_ui.to_owned(), atlas)
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum BuildingStatus {
    Built,
    Placing,
    NotBuilt,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, EnumIter, Display)]
pub enum BuildingType {
    CommandCentre,
    Barracks,
}

#[allow(unreachable_patterns)]
impl BuildingType {
    pub fn get_all_variants() -> Vec<BuildingType> {
        BuildingType::iter().collect()
    }

    pub fn get_w<'f>(&'f self) -> u32 {
        match self {
            BuildingType::CommandCentre => { 150 },
            BuildingType::Barracks => { 100 },
            _ => { unimplemented!() }
        }
    }

    pub fn get_h<'f>(&'f self) -> u32 {
        match self {
            BuildingType::CommandCentre => { 150 },
            BuildingType::Barracks => { 100 },
            _ => { unimplemented!() }
        }
    }

    pub fn get_build_time<'f>(&'f self) -> Duration {
        match self {
            BuildingType::CommandCentre => { Duration::from_millis(general::CCBUILD_TIME) },
            BuildingType::Barracks => { Duration::from_millis(general::BARRACKS_BUILD_TIME) },
            _ => { unimplemented!() }
        }
    }
}
