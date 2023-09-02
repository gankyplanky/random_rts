use sdl2::render::{Texture, WindowCanvas};
use sdl2::rect::{Rect, Point};
use rand::prelude::Distribution;
use rand::distributions::Uniform;

use super::sprite::Sprite;
use super::general::Collidable;

//Represents current world or map, also used as camera boundary
pub struct World<'w> {
    pub world_textures: Vec<&'w Texture<'w>>,
    pub world_sprites: Vec<Vec<Sprite<'w>>>,
    pub world_encode: Vec<Vec<i32>>
}

impl<'w> World<'w> {
    pub fn new(world_textures: Vec<&'w Texture>, world_encode: Vec<Vec<i32>>) -> World<'w> {
        let mut new_world = World{
            world_textures,
            world_encode,
            world_sprites: {
                let new_sprites: Vec<Vec<Sprite>> = vec![vec![]];
                new_sprites
            }
        };
        new_world.load_sprites();
        return new_world;
    }

    pub fn load_sprites(&mut self) {
        self.world_sprites.pop();
        let mut i: usize = 0;
        let mut rng = rand::thread_rng();
        let die = Uniform::from(0..3);
        while i < self.world_encode.len() {
            let mut j: usize = 0;
            let mut temp_sprites: Vec<Sprite> = vec![];
            while j < self.world_encode[i].len() {
                let number = die.sample(&mut rng);
                temp_sprites.push(Sprite::new(
                    self.world_textures[self.world_encode[i][j] as usize],
                    Rect::new(number as i32 * 64, 0, 64, 64),
                    Point::new(i as i32 * 50, j as i32 * 50), 50, 50));
                j += 1;
            }
            
            self.world_sprites.push(temp_sprites);
            i += 1;
        }
    }

    pub fn render(&mut self, canvas: &mut WindowCanvas) {
        let mut i: usize = 0;
        while i < self.world_sprites.len() {
            let mut j: usize = 0;
            while j < self.world_sprites[i].len() {
                self.world_sprites[i][j].render(canvas);
                j += 1;
            }
            i += 1;
        }
        
    }
}

pub struct WorldObject<'o> {
    sprite: Sprite<'o>,
    collider: Rect,
    collider_type: Collidable,
    top_clamp: Point,
    bottom_clamp: Point,
}

impl<'o> WorldObject<'o> {
    pub fn new(texture_source: &'o Texture, collider: Rect, collider_type: Collidable,
            initial_location: Point, bottom_clamp: Point) -> WorldObject<'o> {
        let new_object = WorldObject {
            sprite: {
                Sprite { 
                    texture_source, 
                    location: initial_location, 
                    texture_loaction: Rect::new(0, 0, 64, 64),
                    width: 50, 
                    height: 50, 
                }
            },
            collider,
            collider_type,
            top_clamp: initial_location,
            bottom_clamp
        };

        return new_object;
    }

    pub fn render(&mut self, canvas: &mut WindowCanvas) {
        self.sprite.render(canvas);
    }
}

