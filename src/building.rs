use std::time::Duration;

use sdl2::rect::{Point, Rect};
use sdl2::render::{WindowCanvas, Texture};

use super::sprite::Sprite;
use super::general::Collidable;
use super::general::Faction;
use super::ui::{Button, UiElement, ButtonFunction};
use super::Player;
use stopwatch::Stopwatch;

const CCBUILD_TIME: u64 = 5000;
const BARRACKS_BUILD_TIME: u64 = 5000;

#[derive(Clone)]
pub struct Building<'b> {
    pub sprite: Sprite<'b>,
    pub team: i32,
    pub building_type: BuildingType,
    pub faction: Faction,
    pub collider_type: Collidable,
    pub collider: Rect,
    pub buttons: Vec<[Option<Button<'b>>; 16]>,
    pub button_panel_index: usize,
    pub button_panel_limit: usize,
    pub constructing: Option<Construction<'b>>,
    pub status: BuildingStatus,
    pub place_construction_flag: bool,
}

impl<'b> Building<'b> {
    pub fn new<'a>(location: Point, building_type: BuildingType, faction: Faction,
            team: i32, texture_source: &'b Texture<'b>,
            button_texture: &'b Texture<'b>,
             bottom_right_ui: Vec<UiElement<'a>>) -> Building<'b> {
        let temp_building_type = building_type.clone();
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
            collider: Rect::new(location.x, location.y, 0, 0),
            constructing: None,
            sprite: Sprite { 
                texture_source,
                location,
                texture_location: {
                    match temp_building_type {
                        BuildingType::CommandCentre => {
                            Rect::new(0, 0, 128, 128)  
                        },
                        BuildingType::Barracks => {
                            Rect::new(128, 0, 128, 128)
                        },
                    }
                },
                width: { 
                    match temp_building_type {
                        BuildingType::CommandCentre => {
                            150  
                        },
                        BuildingType::Barracks => {
                            50
                        },
                    }
                },
                height: { 
                    match temp_building_type {
                        BuildingType::CommandCentre => {
                            150  
                        },
                        BuildingType::Barracks => {
                            50
                        },
                    }
                },
                rect: Rect::new(0, 0, 0, 0)
            }
        };
        
        new_building.sprite.rect = Rect::new(location.x, location.y, 
            new_building.sprite.width, new_building.sprite.height);
        Building::init_buttons(&mut new_building, button_texture, bottom_right_ui);
        new_building.collider = new_building.sprite.rect.to_owned();

        return new_building;
    }

    fn init_buttons<'a>(building: &mut Building<'b>, button_texture: &'b Texture<'b>,
            bottom_right_ui: Vec<UiElement<'a>>) {
        match building.faction {
            Faction::PlaceholderFaction1 => {
                match building.building_type {
                    BuildingType::CommandCentre => {
                        building.button_panel_limit = 3;
                        building.buttons.push([None; 16]);
                        building.buttons.push([None; 16]);
                        building.buttons.push([None; 16]);

                        building.buttons[0][0] = Some(Button::new(UiElement::new(
                            Sprite::new(button_texture, Rect::new(0, 0, 64, 64), 
                                Point::new(bottom_right_ui[1].sprite.location.x, 
                                    bottom_right_ui[1].sprite.location.y),
                                50, 50)
                            ),
                            crate::ui::ButtonFunction::ShowTier1Buildings
                        ));
                        building.buttons[0][1] = Some(Button::new(UiElement::new(
                            Sprite::new(button_texture, Rect::new(64, 0, 64, 64), 
                                Point::new(bottom_right_ui[2].sprite.location.x, 
                                    bottom_right_ui[2].sprite.location.y),
                                50, 50)
                            ),
                            crate::ui::ButtonFunction::ShowTier2Buildings
                        ));
                        building.buttons[0][15] = Some(Button::new(UiElement::new(
                            Sprite::new(button_texture, Rect::new(5 * 64, 0, 64, 64), 
                                Point::new(bottom_right_ui[16].sprite.location.x, 
                                    bottom_right_ui[16].sprite.location.y),
                                50, 50)
                            ),
                            crate::ui::ButtonFunction::PlaceConstruction
                        ));
                        building.buttons[1][4] = Some(Button::new(UiElement::new(
                            Sprite::new(button_texture, Rect::new(2 * 64, 0, 64, 64), 
                                Point::new(bottom_right_ui[5].sprite.location.x, 
                                    bottom_right_ui[5].sprite.location.y),
                                50, 50)
                            ),
                            crate::ui::ButtonFunction::MakeCC
                        ));
                        building.buttons[1][5] = Some(Button::new(UiElement::new(
                            Sprite::new(button_texture, Rect::new(4 * 64, 0, 64, 64), 
                                Point::new(bottom_right_ui[6].sprite.location.x, 
                                    bottom_right_ui[6].sprite.location.y),
                                50, 50)
                            ),
                            crate::ui::ButtonFunction::Back
                        ));
                        building.buttons[2][8] = Some(Button::new(UiElement::new(
                            Sprite::new(button_texture, Rect::new(3 * 64, 0, 64, 64), 
                                Point::new(bottom_right_ui[9].sprite.location.x, 
                                    bottom_right_ui[9].sprite.location.y),
                                50, 50)
                            ),
                            crate::ui::ButtonFunction::MakeWorker
                        ));
                        building.buttons[2][9] = Some(Button::new(UiElement::new(
                            Sprite::new(button_texture, Rect::new(4 * 64, 0, 64, 64), 
                                Point::new(bottom_right_ui[10].sprite.location.x, 
                                    bottom_right_ui[10].sprite.location.y),
                                50, 50)
                            ),
                            crate::ui::ButtonFunction::Back
                        ));
                    },
                    BuildingType::Barracks => {

                    }
                }
            }
        } 
    }
    
    pub fn execute_fn<'f>(&'f mut self, function: ButtonFunction, owner: Player<'b>) {
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
    
    pub fn get_constructed<'f>(&'f self) -> Option<Building<'b>> {
        if self.constructing.is_some() {
            let temp_con: &'f Construction = self.constructing.as_ref().unwrap();
            if temp_con.finished {
                return Some(temp_con.build_building());
            }
        }

        None
    }

    fn set_button_panel<'f>(&'f mut self, index: usize) {
        self.button_panel_index = index;
    }
   
    pub fn move_building<'f>(&'f mut self, mouse_point: Point, cam_viewport: Rect) {
        let world_pos: Point = Point::new(
            cam_viewport.x + mouse_point.x - self.collider.w + 25,
            cam_viewport.y + mouse_point.y - self.collider.h + 25 
        );

        self.sprite.location = world_pos;
        self.collider.x = world_pos.x;
        self.collider.y = world_pos.y;
        self.sprite.rect.x = world_pos.x;
        self.sprite.rect.y = world_pos.y;
    }

    fn start_construction<'f>(&'f mut self, building_type: BuildingType,
            bottom_right_ui: Vec<UiElement<'b>>) {

        self.constructing = Some(Construction::new(Point::new(0, 0), building_type,
        self.faction.to_owned(), self.team.to_owned(),
        self.sprite.texture_source, 
        self.buttons[0][0].unwrap().ui.sprite.texture_source,
        match building_type {
            BuildingType::CommandCentre => {
                Duration::from_millis(CCBUILD_TIME)            
            },
            BuildingType::Barracks => {
                Duration::from_millis(BARRACKS_BUILD_TIME)
            }
        },
        bottom_right_ui));
    }

    pub fn render<'f>(&'f mut self, canvas: &'f mut WindowCanvas) {
        match self.status {
            BuildingStatus::Built => {
                self.sprite.render(canvas);
            },
            BuildingStatus::Placing => {
                self.sprite.texture_location.x += 128;
                self.sprite.render(canvas);
                self.sprite.texture_location.x -= 128;
            },
            BuildingStatus::NotBuilt => {}
        }
    } 
}

#[derive(Clone)]
pub struct Construction<'c> {
    pub building_type: BuildingType,
    pub location: Point,
    pub faction: Faction,
    pub team: i32,
    pub texture_source: &'c Texture<'c>,
    pub button_texture: &'c Texture<'c>,
    pub bottom_right_ui: Vec<UiElement<'c>>,
    pub timer: Stopwatch,
    pub timer_end:  Duration,
    pub finished: bool,
}

impl<'c> Construction<'c> {
    pub fn new<'f>(location: Point, building_type: BuildingType, faction: Faction,
            team: i32, texture_source: &'c Texture<'c>,
            button_texture: &'c Texture<'c>, timer_end: Duration,
            bottom_right_ui: Vec<UiElement<'c>>) -> Construction<'c> {
        
        let mut new_construction = Construction {
            building_type,
            location,
            faction,
            team,
            texture_source,
            button_texture,
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

    pub fn build_building<'f>(&'f self) -> Building<'c> {
        Building::new(self.location, self.building_type, self.faction, self.team,
            self.texture_source, self.button_texture, self.bottom_right_ui.to_owned())
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum BuildingStatus {
    Built,
    Placing,
    NotBuilt,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum BuildingType {
    CommandCentre,
    Barracks,
}

