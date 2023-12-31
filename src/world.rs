use sdl2::render::WindowCanvas;
use sdl2::rect::{Rect, Point};

use crate::sprite::{TextureType, TextureManager};
use crate::sprite::Sprite;
use crate::general::{Collidable, Renderable};

//Represents current world or map, also used as camera boundary
pub struct World {
    pub world_sprites: Vec<Vec<Sprite>>,
    pub world_encode: Vec<Vec<i32>>,
    pub grid: Vec<Vec<Cell>>,
}

impl World {
    pub fn new<'f>(world_encode: Vec<Vec<i32>>, atlas: &'f TextureManager) -> World {
        let mut new_world = World{
            world_encode,
            grid: {
                let mut new_grid: Vec<Vec<Cell>> = vec![vec![]];
                new_grid.pop();
                new_grid
            },
            world_sprites: {
                let mut new_sprites: Vec<Vec<Sprite>> = vec![vec![]];
                new_sprites.pop();
                new_sprites
            },
        };
        new_world.load_sprites(atlas);
    
        {
            let mut i: usize = 0;
            while i < (new_world.world_sprites.len() * 2) {
                let mut j: usize = 0;
                let mut grid_row: Vec<Cell> = vec![];
                while j < (new_world.world_sprites[i/2].len() * 2) {
                    grid_row.push(Cell::new(Point::new(i as i32 * 25, j as i32 * 25),
                        TextureType::World { tile_index: 0 },
                        atlas));
                    j += 1;
                }
                
                new_world.grid.push(grid_row);
                i += 1;
            }
        }

        return new_world;
    }

    pub fn load_sprites<'f>(&'f mut self, atlas: &'f TextureManager) {
        let mut i: usize = 0;
        while i < self.world_encode.len() {
            let mut j: usize = 0;
            let mut temp_sprites: Vec<Sprite> = vec![];
            while j < self.world_encode[i].len() {
                temp_sprites.push(Sprite::new(
                    Rect::new(i as i32 * 50, j as i32 * 50, 50, 50),
                    TextureType::World { tile_index: self.world_encode[i][j] as usize },
                    atlas));
                j += 1;
            }
            
            self.world_sprites.push(temp_sprites);
            i += 1;
        }
    }

    pub fn render<'f>(&'f self, canvas: &'f mut WindowCanvas, mut viewport: Rect,
            show_grid: bool, tx_mgr: &'f TextureManager) {
        
        viewport.set_width(viewport.width() + 100);
        viewport.set_height(viewport.height() + 100);
        viewport.set_x(viewport.x - 100);
        viewport.set_y(viewport.y - 100);
        
        self.world_sprites.iter().flatten()
            .filter(|tile| viewport.contains_rect(tile.loc_rect))
            .for_each(|tile| tile.render(tx_mgr, canvas));

        if show_grid {
            self.grid.iter().flatten()
                .filter(|cell| viewport.contains_rect(cell.get_loc_rect()))
                .for_each(|cell| cell.render(tx_mgr, canvas));
        }
    }
}

#[derive(Clone, Copy)]
pub struct Cell {
    pub sprite: Sprite,
    pub occupied: bool,
    pub highlighted: bool,
}

impl Cell {
    pub fn new<'f>(location: Point, t_type: TextureType, atlas: &'f TextureManager) -> Cell {
        Cell {
            sprite: Sprite::new(Rect::new(location.x, location.y, 25, 25), t_type, atlas),
            occupied: false,
            highlighted: false,
        }
    }

    pub fn occupy<'f>(&'f mut self) {
        self.occupied = true;
        self.sprite.texture_rect.x = 64;
    }

    pub fn _deoccupy<'f>(&'f mut self) {
        self.occupied = false;
        if self.highlighted {
            self.sprite.texture_rect.x = 128;
        } else {
            self.sprite.texture_rect.x = 0;
        }

    }
    
    pub fn highlight<'f>(&'f mut self) {
        self.highlighted = true;
        if !self.occupied {
            self.sprite.texture_rect.x = 128;
        }
    }

    pub fn dehighlight<'f>(&'f mut self) {
        self.highlighted = false;
        if !self.occupied {
            self.sprite.texture_rect.x = 0;
        }
    }
}

impl Renderable for Cell {
    fn render<'f>(&'f self, tx_mgr: &'f TextureManager, canvas: &'f mut WindowCanvas) {
        self.sprite.render(tx_mgr, canvas);
    }

    fn get_loc_rect<'f>(&'f self) -> Rect {
        self.sprite.get_loc_rect()
    }
}

#[allow(dead_code)]
pub struct WorldObject {
    sprite: Sprite,
    collider: Rect,
    collider_type: Collidable,
}

#[allow(dead_code)]
impl WorldObject {
    pub fn new<'f>(t_type: TextureType, collider: Rect, collider_type: Collidable, 
            atlas: &'f TextureManager) -> WorldObject {
        WorldObject {
            sprite: Sprite::new(collider, t_type, atlas), 
            collider,
            collider_type,
        }
    }
}

impl Renderable for WorldObject {
    fn render<'f>(&'f self, tx_mgr: &'f TextureManager, canvas: &'f mut WindowCanvas) {
        self.sprite.render(tx_mgr, canvas);
    }

    fn get_loc_rect<'f>(&'f self) -> Rect {
        self.collider
    }
}
