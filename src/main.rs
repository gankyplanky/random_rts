extern crate sdl2;
extern crate stopwatch;

mod sprite;
mod camera;
mod building;
mod general;
mod player;
mod ui;
mod unit;
mod world;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::image::{self, InitFlag, LoadTexture};
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{WindowCanvas, Texture};
use sdl2::mouse::MouseButton;

use stopwatch::Stopwatch;

use std::time::Duration;
use std::cmp::{max, min}; 

use sprite::Sprite;
use camera::Camera;
use world::{World, WorldObject};
use general::{Collidable, Faction};
use building::Building;
use unit::Unit;
use player::Player;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let _image_context = image::init(InitFlag::PNG | InitFlag::JPG);
    
    //Window setup
    let window_width = 1920;
    let window_height = 1080;
    
    let window = video_subsystem.window("Random RTS", window_width, window_height)
        .fullscreen()
        .build()
        .unwrap();
    
    let mut canvas = window.into_canvas().build().unwrap();
    let mut player_cam = Camera::new(canvas.viewport(), Keycode::Up, Keycode::Down,
        Keycode::Left, Keycode::Right, 15);
    
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    //Load Textures
    let texture_loader = canvas.texture_creator();
    let none_texture = texture_loader.load_texture("assets/none_sprite.png");
    let ball_texture = texture_loader.load_texture("assets/ball.png");
    let grass_texture = texture_loader.load_texture("assets/ground/ground_grass.png");
    let dirt_texture = texture_loader.load_texture("assets/ground/ground_dirt.png");
    let ui_texture = texture_loader.load_texture("assets/UI/bottom_left_ui_placeholder.png");

    //Rendering vectors
    let mut objects: Vec<WorldObject> = vec![];
    let mut buildings: Vec<&mut Building> = vec![];
    let mut units: Vec<&mut Unit> = vec![];
    

    //Load Sprites
    let mut game_map = World::new(
        vec![grass_texture.as_ref().unwrap(), 
            dirt_texture.as_ref().unwrap()],
        {
            let mut new_encode: Vec<Vec<i32>> = vec![vec![]];
            {
                let mut i: usize = 0;
                while i < 75 {
                    let mut j: usize = 0;
                    new_encode.push(vec![]);
                    while j < 75 {
                        new_encode[i].push(0);
                        j += 1;
                    }
                    i += 1;
                }
            }

            new_encode[1][1] = 1;
            new_encode[1][2] = 1;
            new_encode[2][1] = 1;
            new_encode[2][2] = 1;

            new_encode
        });
    
    let mut buffer : Texture = texture_loader.create_texture_target(
        texture_loader.default_pixel_format(), 
        game_map.world_sprites.len() as u32 * 
            game_map.world_sprites[0][0].width + 100,
        game_map.world_sprites[0].len() as u32 *
            game_map.world_sprites[0][0].height + 100).unwrap();
    
    let mut player = Player::new(Faction::PlaceholderFaction1, ui_texture.as_ref().unwrap());

    let mut event_pump = sdl_context.event_pump().unwrap();
    'main: loop {
        //let mut temp_timer = Stopwatch::new();
        //temp_timer.start();

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
                    player_cam.check_down_key(keycode.unwrap());
                },
                Event::KeyUp { keycode, .. } => { // Key released
                    player_cam.check_up_key(keycode.unwrap());
                },
                Event::MouseMotion {x, y, .. } => { // Mouse map scrolling
                    player_cam.mouse_panning(x, y);
                }
                _ => {}
            }
        }

        //Logic Processing segment
        //Camera Movement - must be done before any other position related calcs
        player_cam.move_cam(&game_map);

        //Rendering segment (order: world -> object -> buildings -> units -> UI)
        let _ = canvas.with_texture_canvas(&mut buffer, |texture_canvas| {
            //Game world (map)
            texture_canvas.clear();
            texture_canvas.set_viewport(Rect::new(45, 45,
                (game_map.world_sprites.len() + 1) as u32 *
                    game_map.world_sprites[0][0].width + 50,
                game_map.world_sprites[0].len() as u32 *
                    game_map.world_sprites[0][0].height + 50));
            game_map.render(texture_canvas);
            
            //World objects (decorations, obsticles, cliffs and similar)
            {
                let mut i: usize = 0;
                while i < objects.len() {
                    objects[i].render(texture_canvas);
                    i += 1;
                }
            }
            //Buildings (all player or AI made buildings)
            

            //Units (all units controlled by player or AI)
        
        });
        
        //Copy vieport from buffer
        canvas.copy(&buffer, player_cam.viewport, canvas.viewport())
            .expect("buffer coppy error");

        //UI
        player.render_ui(&mut canvas);

        canvas.present();
        
        //println!("{} ms", temp_timer.elapsed().as_nanos() as f64 / 1_000_000f64);
        //temp_timer.stop();
        //temp_timer.reset();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 144));
    }
}
