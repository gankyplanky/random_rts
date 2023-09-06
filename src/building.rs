use sdl2::rect::{Point, Rect};
use sdl2::render::{WindowCanvas, Texture};

use super::sprite::Sprite;
use super::general::Collidable;
use super::general::Faction;

pub struct Building<'b> {
    pub sprite: Sprite<'b>,
    pub team: i32,
    pub building_type: BuildingType,
    pub faction: Faction,
    pub collider_type: Collidable,
    pub collider: Rect,
    pub top_clamp: Point,
    pub bottom_clamp: Point,
}

impl<'b> Building<'b> {
    pub fn new(location: Point, building_type: BuildingType, faction: Faction,
            team: i32, bottom_clamp: Point, texture_source: &'b Texture<'b>) -> Building<'b> {
        let temp_building_type = building_type.clone();
        let new_building = Building {
            team,
            building_type,
            faction,
            top_clamp: location,
            bottom_clamp,
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

        return new_building;
    }

    pub fn render(&mut self, canvas: &mut WindowCanvas) {
        self.sprite.render(canvas);
    } 
}

#[derive(Clone, Copy)]
pub enum BuildingType {
    CommandCentre,
    Barracks,
}

