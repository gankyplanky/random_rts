use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;

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
}
