extern crate sdl2;
extern crate stopwatch;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::image::LoadTexture;
use sdl2::rect::{Point, Rect};
use sdl2::render::{WindowCanvas, Texture};

use stopwatch::Stopwatch;

use std::time::Duration;

struct Sprite<'s> {
    texture_source: &'s Texture<'s>,
    location: Point,
    texture_loaction: Rect,
    width: u32,
    height: u32,
}

impl<'s> Sprite<'s> {
    fn new(texture_source: &'s Texture, texture_loaction: Rect, 
            initial_location: Point, width: u32, height: u32) -> Sprite<'s> {
        let new_sprite = Sprite {
            texture_source,
            texture_loaction,
            location: initial_location,
            width,
            height,
        };

        return new_sprite;
    }

    fn render(&mut self, canvas: &mut WindowCanvas) {
        canvas.copy(self.texture_source, self.texture_loaction,
            Rect::new(self.location.x, self.location.y, self.width, self.height))
            .expect("Failed to render texture");
    }
}

struct Camera {
    move_up: bool,
    move_down: bool,
    move_left: bool,
    move_right: bool,
    can_move: bool,
    viewport: Rect,
    up_keycode: Keycode,
    down_keycode: Keycode,
    left_keycode: Keycode,
    right_keycode: Keycode,
    speed: u32,
    width_bound: i32,
    height_bound: i32,
}

impl Camera {
    fn new(viewport: Rect, up: Keycode, down: Keycode,
            left: Keycode, right: Keycode, speed: u32,
            width_bound: i32, height_bound: i32) -> Camera {
        let new_cam = Camera {
            move_up: false,
            move_down: false,
            move_left: false,
            move_right: false,
            can_move: true,
            viewport,
            up_keycode: up,
            down_keycode: down,
            left_keycode: left,
            right_keycode: right,
            speed,
            width_bound,
            height_bound
        };

        return new_cam;
    }
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    
    //Window setup
    let window_width = 1920;
    let window_height = 1080;

    let window = video_subsystem.window("Random RTS", window_width, window_height)
        .position_centered().fullscreen()
        .build()
        .unwrap();
    
    let mut canvas = window.into_canvas().build().unwrap();
    let mut player_cam = Camera::new(canvas.viewport(), Keycode::Up,
        Keycode::Down, Keycode::Left, Keycode::Right, 4, -10000, -10000);
    
    //Texture setup
    let texture_loader = canvas.texture_creator(); 
    let none_texture = texture_loader.load_texture("assets/none_sprite.png");
    
    //Load Sprites
    let mut sprite = Sprite::new(none_texture.as_ref().unwrap(), Rect::new(0, 0, 64, 64), 
        Point::new(0, 0), 50, 50);
  
    let mut event_pump = sdl_context.event_pump().unwrap();
    'main: loop {
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown {keycode: Some(Keycode::Escape), .. } => {
                    break 'main
                },
                //Keybind handling segment
                Event::KeyDown { keycode, .. } => { // Key pressed
                    let down_key = keycode.unwrap();
                    if down_key == player_cam.up_keycode {
                        player_cam.move_up = true;
                    }
                    if down_key == player_cam.down_keycode {
                        player_cam.move_down = true;
                    }
                    if down_key == player_cam.left_keycode {
                        player_cam.move_left = true;
                    }
                    if down_key == player_cam.right_keycode {
                        player_cam.move_right = true;
                    }
                },
                Event::KeyUp { keycode, .. } => { // Key released
                    let up_key = keycode.unwrap();
                    if up_key == player_cam.up_keycode {
                        player_cam.move_up = false;
                    }
                    if up_key == player_cam.down_keycode {
                        player_cam.move_down = false;
                    }
                    if up_key == player_cam.left_keycode {
                        player_cam.move_left = false;
                    }
                    if up_key == player_cam.right_keycode {
                        player_cam.move_right = false;
                    }
                },
                _ => {}
            }
        }

        //Logic Processing segment
        //Camera Movement
        if player_cam.can_move {
            let mut x_move: i32 = 0;
            let mut y_move: i32 = 0;
            
            if player_cam.move_up {
                y_move += player_cam.speed as i32;
            }
            if player_cam.move_down {
                y_move -= player_cam.speed as i32;
            }
            if player_cam.move_left {
                x_move += player_cam.speed as i32;
            }
            if player_cam.move_right {
                x_move -= player_cam.speed as i32;
            }

            player_cam.viewport.offset(x_move, y_move);
            canvas.set_viewport(player_cam.viewport);
        }
        
        //Rendering segment
        sprite.render(&mut canvas);

        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 144));
    }
}
