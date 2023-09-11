use sdl2::rect::{Point, Rect};
use sdl2::render::{WindowCanvas, Texture};

use super::sprite::Sprite;
use super::general::Collidable;
use super::general::Faction;
use super::ui::{Button, UiElement};
use super::Player;

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
}

impl<'b> Building<'b> {
    pub fn new<'a>(location: Point, building_type: BuildingType, faction: Faction,
            team: i32, texture_source: &'b Texture<'b>,
            button_texture: &'b Texture<'b>, owner: &'a Player<'a>) -> Building<'b> {
        let temp_building_type = building_type.clone();
        let mut new_building = Building {
            team,
            building_type,
            faction,
            buttons: vec![],
            button_panel_index: 0,
            button_panel_limit: 1,
            collider_type: Collidable::GroundCollidable,
            collider: Rect::new(location.x, location.y, 0, 0),
            sprite: Sprite { 
                texture_source,
                location,
                texture_loaction: {
                    match temp_building_type {
                        BuildingType::CommandCentre => {
                            Rect::new(0, 0, 128, 128)  
                        },
                        BuildingType::Barracks => {
                            Rect::new(0, 0, 64, 64)
                        },
                    }
                },
                width: { 
                    match temp_building_type {
                        BuildingType::CommandCentre => {
                            100  
                        },
                        BuildingType::Barracks => {
                            50
                        },
                    }
                },
                height: { 
                    match temp_building_type {
                        BuildingType::CommandCentre => {
                            100  
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
        Building::init_buttons(&mut new_building, button_texture, owner);

        return new_building;
    }
    
    fn init_buttons<'a>(building: &mut Building<'b>, button_texture: &'b Texture<'b>, owner: &'a Player<'a>) {
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
                                Point::new(owner.bottom_right_ui[1].sprite.location.x, 
                                    owner.bottom_right_ui[1].sprite.location.y),
                                50, 50)
                            ),
                            ||{
                                println!("clicked 1");
                            }));
                        building.buttons[0][1] = Some(Button::new(UiElement::new(
                            Sprite::new(button_texture, Rect::new(64, 0, 64, 64), 
                                Point::new(owner.bottom_right_ui[2].sprite.location.x, 
                                    owner.bottom_right_ui[2].sprite.location.y),
                                50, 50)
                            ),
                            ||{
                                println!("clicked 2");
                            }));
                    },
                    BuildingType::Barracks => {

                    }
                }
            }
        } 
    }

    pub fn render(&self, canvas: &mut WindowCanvas) {
        self.sprite.render(canvas);
    } 
}

#[derive(Clone, Copy)]
pub enum BuildingType {
    CommandCentre,
    Barracks,
}

