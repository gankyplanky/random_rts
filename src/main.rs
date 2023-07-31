extern crate sdl2;
extern crate stopwatch;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::image::{self, InitFlag, LoadTexture};
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{WindowCanvas, Texture};

use stopwatch::Stopwatch;

use std::time::Duration;
use std::cmp::{max, min};

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
    up_keycode: Keycode,
    down_keycode: Keycode,
    left_keycode: Keycode,
    right_keycode: Keycode,
    speed: u32,
}

impl Camera {
    fn new(up: Keycode, down: Keycode,
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
            speed,
        };

        return new_cam;
    }
}

struct World<'w> {
    world_texture: &'w Texture<'w>,
    world_sprites: Vec<Vec<Sprite<'w>>>,
    world_encode: Vec<Vec<i32>>
}

impl<'w> World<'w> {
    fn new(world_texture: &'w Texture, world_encode: Vec<Vec<i32>>) -> World<'w> {
        let mut new_world = World{
            world_texture,
            world_encode,
            world_sprites: {
                let new_sprites: Vec<Vec<Sprite>> = vec![vec![]];
                new_sprites
            }
        };
        new_world.load_sprites();
        return new_world;
    }

    fn load_sprites(&mut self) {
        self.world_sprites.pop();
        let mut i: usize = 0;
        while i < self.world_encode.len() {
            let mut j: usize = 0;
            let mut temp_sprites: Vec<Sprite> = vec![];
            while j < self.world_encode[i].len() {
                temp_sprites.push(Sprite::new(self.world_texture,
                    Rect::new(self.world_encode[i][j] * 64, 0, 64, 64),
                    Point::new(i as i32 * 50, j as i32 * 50), 50, 50));
                j += 1;
            }
            
            self.world_sprites.push(temp_sprites);
            i += 1;
        }
    }
    
    fn move_world_cam(&mut self, window_width: i32, window_height: i32,
                x_move: i32, y_move: i32) {
        let mut i: usize = 0;
        while i < self.world_sprites.len() {
            let mut j: usize = 0;
            while j < self.world_sprites[i].len() {
                self.world_sprites[i][j].location.x += x_move;
                self.world_sprites[i][j].location.x = min(
                    self.world_sprites[i][j].location.x, i as i32 * 50 + 5
                );
                self.world_sprites[i][j].location.x = max(
                    self.world_sprites[i][j].location.x,
                    (self.world_sprites.len() as i32 - i as i32 - 1)
                    * -50 - 5 + window_width 
                );

                self.world_sprites[i][j].location.y += y_move;
                self.world_sprites[i][j].location.y = min(
                    self.world_sprites[i][j].location.y, j as i32 * 50 + 5
                );
                self.world_sprites[i][j].location.y = max(
                    self.world_sprites[i][j].location.y,
                    (self.world_sprites[i].len() as i32 - j as i32)
                    * -50 - 5 + window_height 
                );
                j += 1;
            }
            i += 1;
        }

    }

    fn render(&mut self, canvas: &mut WindowCanvas) {
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

struct WorldObject<'o> {
    sprite: Sprite<'o>,
    collider: Rect,
    collider_type: Collidable,
}

enum Collidable {
    GroundCollidable,
    GroundUncollidable,
    AirCollidable,
    AirUncollidable
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let _image_context = image::init(InitFlag::PNG | InitFlag::JPG);
    
    //Window setup
    let window_width = 1920;
    let window_height = 1080;

    let window = video_subsystem.window("Random RTS", window_width, window_height)
        .position_centered().fullscreen()
        .build()
        .unwrap();
    
    let mut canvas = window.into_canvas().build().unwrap();
    let mut player_cam = Camera::new(Keycode::Up, Keycode::Down,
        Keycode::Left, Keycode::Right, 5);
    
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    //Load Textures
    let texture_loader = canvas.texture_creator(); 
    let none_texture = texture_loader.load_texture("assets/none_sprite.png");
    let ball_texture = texture_loader.load_texture("assets/ball.png");
    
    //Load Sprites
    let mut game_map = World::new(none_texture.as_ref().unwrap(),
        {
            let mut new_encode: Vec<Vec<i32>> = vec![vec![]];
            {
                let mut i: usize = 0;
                while i < 50 {
                    let mut j: usize = 0;
                    new_encode.push(vec![]);
                    while j < 50 {
                        if i == j {
                            new_encode[i].push(1);
                        } else {
                            new_encode[i].push(0);
                        }
                        j += 1;
                    }
                    i += 1;
                }
            }
            new_encode[0][2] = 1;
            new_encode[0][3] = 1;
            new_encode
        });
    
    

    let mut timer = Stopwatch::new();
    timer.start();

    let mut event_pump = sdl_context.event_pump().unwrap();
    'main: loop {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
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
            
            game_map.move_world_cam(window_width as i32, window_height as i32, x_move, y_move);
        }
        
        if timer.elapsed().as_millis() >= 400 {
            let mut i: usize = 0;
            while i < game_map.world_sprites.len() {
                let mut j: usize = 0;
                while j < game_map.world_sprites[i].len() {
                    if game_map.world_sprites[i][j].texture_loaction.x == 0 {
                        game_map.world_sprites[i][j].texture_loaction.set_x(64);
                    } else {
                        game_map.world_sprites[i][j].texture_loaction.set_x(0);
                    }
                    j += 1;
                }
                i += 1;
            }
            timer.restart();
        }

        //Rendering segment (order: world -> units/object -> UI)
        //Game world (map)
        game_map.render(&mut canvas);
    
        canvas.present();

        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 144));
    }
}
