use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use std::cmp::{max, min};

use crate::world::World;

//Represents bindings for player camera
pub struct Camera {
	pub move_up: bool,
	pub move_down: bool,
	pub move_left: bool,
	pub move_right: bool,
	pub can_move: bool,
	pub up_keycode: Keycode,
	pub down_keycode: Keycode,
	pub left_keycode: Keycode,
	pub right_keycode: Keycode,
	pub viewport: Rect,
	pub speed: u32,
}

impl Camera {
    pub fn new(viewport: Rect, up: Keycode, down: Keycode,
            left: Keycode, right: Keycode, speed: u32) -> Camera {
        let new_cam = Camera {
            move_up: false,
            move_down: false,
            move_left: false,
            move_right: false,
            can_move: true,
            up_keycode: up,
            down_keycode: down,
            left_keycode: left,
            right_keycode: right,
            viewport,
            speed,
        };

        return new_cam;
    }
    
    pub fn check_up_key(&mut self, up_key: Keycode) {
        if up_key == self.up_keycode {
            self.move_up = false;
        }
        if up_key == self.down_keycode {
            self.move_down = false;
        }
        if up_key == self.left_keycode {
            self.move_left = false;
        }
        if up_key == self.right_keycode {
            self.move_right = false;
        }
    }

    pub fn check_down_key(&mut self, down_key: Keycode) {
        if down_key == self.up_keycode {
            self.move_up = true;
        }
        if down_key == self.down_keycode {
            self.move_down = true;
        }
        if down_key == self.left_keycode {
            self.move_left = true;
        }
        if down_key == self.right_keycode {
            self.move_right = true;
        }
    }

    pub fn mouse_panning(&mut self, x: i32, y: i32) {
        if y >= 0 && y <= 5 {
            self.move_up = true;
        } else {
            self.move_up = false;
        }
        
        if y >= self.viewport.height() as i32 - 5 && y <= self.viewport.height() as i32 {
            self.move_down = true;
        } else {
            self.move_down = false;
        }

        if x >= 0 && x <= 5 {
            self.move_left = true;
        } else {
            self.move_left = false;
        }

        if x >= self.viewport.width() as i32 - 5 && x <= self.viewport.width() as i32 {
            self.move_right = true;
        } else {
            self.move_right = false;
        }
    }

    pub fn move_cam(&mut self, game_map: &World) {
        if self.can_move {
            let mut x_move: i32 = 0;
            let mut y_move: i32 = 0;
            
            if self.move_up {
                y_move -= self.speed as i32;
            }
            if self.move_down {
                y_move += self.speed as i32;
            }
            if self.move_left {
                x_move -= self.speed as i32;
            }
            if self.move_right {
                x_move += self.speed as i32;
            }
            
            self.viewport.set_x(self.viewport.x + x_move);
            self.viewport.set_y(self.viewport.y + y_move);

            self.viewport.set_x(max(self.viewport.x, 40));
            self.viewport.set_y(max(self.viewport.y, 40));

            self.viewport.set_x(min(self.viewport.x,
                (game_map.world_sprites.len() as i32 - 1) * 
                    game_map.world_sprites[0][0].width as i32 - 
                        self.viewport.width() as i32 + 55));
            
            self.viewport.set_y(min(self.viewport.y, 
                game_map.world_sprites[0].len() as i32 * 
                    game_map.world_sprites[0][0].height as i32 - 
                        self.viewport.height() as i32 + 55));
        }

    }
}
