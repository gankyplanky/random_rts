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
    pub world_encode: Vec<Vec<i32>>,
    pub grid_texture: &'w Texture<'w>,
    pub grid: Vec<Vec<Cell<'w>>>,
}

impl<'w> World<'w> {
    pub fn new(world_textures: Vec<&'w Texture>, world_encode: Vec<Vec<i32>>,
            grid_texture: &'w Texture<'w>) -> World<'w> {
        let mut new_world = World{
            world_textures,
            grid_texture,
            world_encode,
            grid: {
                let new_grid: Vec<Vec<Cell>> = vec![vec![]];
                new_grid
            },
            world_sprites: {
                let new_sprites: Vec<Vec<Sprite>> = vec![vec![]];
                new_sprites
            }
        };
        new_world.load_sprites();
    
        {
            new_world.grid.pop();
            let mut i: usize = 0;
            while i < (new_world.world_sprites.len() * 2) {
                let mut j: usize = 0;
                let mut grid_row: Vec<Cell> = vec![];
                while j < (new_world.world_sprites[i/2].len() * 2) {
                    grid_row.push(Cell::new(
                        Point::new(i as i32 * 25, j as i32 * 25),
                        grid_texture));
                    j += 1;
                }
                
                new_world.grid.push(grid_row);
                i += 1;
            }
        }

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
        
        {
            let mut i: usize = 0;
            while i < self.world_sprites.len() {
                let mut j: usize = 0;
                while j < self.world_sprites[i].len() {
                    self.world_sprites[i][j].rect.offset(100, 100);
                    j += 1;
                }
                i += 1;
            }
        }
    }

    pub fn render(&self, canvas: &mut WindowCanvas, mut viewport: Rect, show_grid: bool) {
        let mut i: usize = 0;
        viewport.set_width(viewport.width() + 100);
        viewport.set_height(viewport.height() + 100);

        while i < self.world_sprites.len() {
            let mut j: usize = 0;
            while j < self.world_sprites[i].len() {
                if viewport.contains_rect(self.world_sprites[i][j].rect) {
                    self.world_sprites[i][j].render(canvas);
                }
                j += 1;
            }
            i += 1;
        }

        if show_grid {
            viewport.set_x(viewport.x - 100);
            viewport.set_y(viewport.y - 100);

            i = 0;
            while i < self.grid.len() {
                let mut j: usize = 0;
                while j < self.grid[i].len() {
                    if viewport.contains_rect(self.grid[i][j].sprite.rect) {
                        self.grid[i][j].render(canvas);
                    }
                    j += 1;
                }
                i += 1;
            }
        }
    }
}

#[derive(Clone, Copy)]
pub struct Cell<'c> {
    pub sprite: Sprite<'c>,
    pub occupied: bool,
    pub highlighted: bool,
}

impl<'c> Cell<'c> {
    pub fn new<'f>(location: Point, texture: &'c Texture<'c>) -> Cell<'c> {
        Cell {
            sprite: Sprite::new(texture, Rect::new(0, 0, 64, 64), location, 25, 25),
            occupied: false,
            highlighted: false,
        }
    }

    pub fn occupy<'f>(&'f mut self) {
        self.occupied = true;
        self.sprite.texture_location.x = 64;
    }

    pub fn deoccupy<'f>(&'f mut self) {
        self.occupied = false;
        if self.highlighted {
            self.sprite.texture_location.x = 128;
        } else {
            self.sprite.texture_location.x = 0;
        }

    }
    
    pub fn highlight<'f>(&'f mut self) {
        self.highlighted = true;
        if !self.occupied {
            self.sprite.texture_location.x = 128;
        }
    }

    pub fn dehighlight<'f>(&'f mut self) {
        self.highlighted = false;
        if !self.occupied {
            self.sprite.texture_location.x = 0;
        }
    }

    pub fn render<'f>(&'f self, canvas: &'f mut WindowCanvas) {
        self.sprite.render(canvas);
    }
}

pub struct WorldObject<'o> {
    sprite: Sprite<'o>,
    collider: Rect,
    collider_type: Collidable,
}

impl<'o> WorldObject<'o> {
    pub fn new(texture_source: &'o Texture, collider: Rect, collider_type: Collidable,
            initial_location: Point) -> WorldObject<'o> {
        let new_object = WorldObject {
            sprite: {
                Sprite { 
                    texture_source, 
                    location: initial_location, 
                    texture_location: Rect::new(0, 0, 64, 64),
                    width: 50, 
                    height: 50,
                    rect: Rect::new(initial_location.x, initial_location.y, 50, 50)
                }
            },
            collider,
            collider_type,
        };

        return new_object;
    }

    pub fn render(&self, canvas: &mut WindowCanvas) {
        self.sprite.render(canvas);
    }
}

