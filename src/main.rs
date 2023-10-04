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
use sdl2::image::{self, InitFlag};
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::{Point, Rect};
use sdl2::render::Texture;
use sdl2::mouse::MouseButton;

#[allow(unused_imports)]
use stopwatch::Stopwatch;

use std::time::Duration;

use sprite::*;
use camera::*;
use world::*;
use general::*;
use building::*;
use unit::*;
use player::*;

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
    let texture_loader = canvas.texture_creator();
    
    //Load texture atlas and create player cam
    let mut tx_mgr = TextureManager::new(&texture_loader);
    tx_mgr.update(&mut canvas);

    let mut player_cam = Camera::new(canvas.viewport(), Keycode::Up, Keycode::Down,
        Keycode::Left, Keycode::Right, 15);

    //Rendering vectors
    #[allow(unused_mut)]
    let mut objects: Vec<WorldObject> = vec![];
    let mut players: Vec<Player> = vec![];

    //Load Sprites
    let mut game_map = World::new(
        {
            let mut new_encode: Vec<Vec<i32>> = vec![vec![]];
            {
                let mut i: usize = 0;
                while i < 75 {
                    let mut j: usize = 0;
                    new_encode.push(vec![]);
                    while j < 75 {
                        new_encode[i].push(1);
                        j += 1;
                    }
                    i += 1;
                }
            }

            new_encode[1][1] = 2;
            new_encode[1][2] = 2;
            new_encode[1][3] = 2;
            new_encode[2][1] = 2;
            new_encode[2][2] = 2;
            new_encode[2][3] = 2;
            new_encode[3][1] = 2;
            new_encode[3][2] = 2;
            new_encode[3][3] = 2;

            new_encode
        },
        &tx_mgr
    );
      
    let mut buffer: Texture = texture_loader.create_texture_target(
        PixelFormatEnum::ARGB32, 
        game_map.world_sprites.len() as u32 * 
            game_map.world_sprites[0][0].loc_rect.w as u32 + 100,
        game_map.world_sprites[0].len() as u32 *
            game_map.world_sprites[0][0].loc_rect.h as u32 + 100).unwrap();
    
    buffer.set_blend_mode(sdl2::render::BlendMode::Blend);
    buffer.set_alpha_mod(255);
    
    canvas.set_blend_mode(sdl2::render::BlendMode::Blend);
    canvas.set_draw_color(Color::RGBA(0, 0, 0, 0));

    players.push(Player::new(Faction::PlaceholderFaction1, player_cam.viewport, &tx_mgr));
    
    {
        let temp = players[0].to_owned();

        players[0].buildings.push(Building::new(Point::new(50, 50), BuildingType::CommandCentre,
            Faction::PlaceholderFaction1, 0, temp.bottom_right_ui.to_owned(), &tx_mgr));
        
        players[0].selected = Selection::Building(0);
        players[0].place_building(&mut game_map);
    }

    let mut avg: f64 = 0f64;
    let mut count: f64 = 0f64;

    let mut event_pump = sdl_context.event_pump().unwrap();
    'main: loop {
        let mut temp_timer = Stopwatch::new();
        temp_timer.start();
        canvas.clear();

        player_cam.viewport.set_width(canvas.window().size().0);
        player_cam.viewport.set_height(canvas.window().size().1);

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
                Event::MouseMotion {x, y, .. } => { // Mouse moved
                    player_cam.mouse_panning(x, y); // Mouse map scrolling

                    let mouse_cam_point = Point::new(x, y);
                    if players[0].placing_building { // Move building ghost
                        players[0].dehighlight(&mut game_map);
                        let index = players[0].selected.index();
                        players[0].buildings[index].move_building(mouse_cam_point,
                            player_cam.viewport, &mut game_map.grid);
                        players[0].buildings[index].highlight_cells(&mut game_map);
                    }
                }
                Event::MouseButtonDown { /*mouse_btn, x, y,*/ .. } => {

                }
                Event::MouseButtonUp { mouse_btn, x, y, .. } => {
                    if mouse_btn == MouseButton::Left {
                        let temp_point = Point::new(x, y);
                        let mut interacted = false;
                        
                        //Check button clicks
                        if players[0].bottom_right_ui[0].collider.contains_point(temp_point) {    
                            let mut i: usize = 0;
                            while i < 16 {
                                if players[0].check_button(temp_point, i) {
                                    interacted = true;
                                    break;
                                }
                                i += 1;
                            }
                        } else if players[0].placing_building {//Place newly constructed building
                            players[0].place_building(&mut game_map);
                            interacted = true;
                        }

                        if !interacted {// Select a building / unit
                            interacted = players[0].try_selecting(temp_point);
                        }

                        if !interacted && !players[0].bottom_right_ui[0].collider // Deselect
                                .contains_point(temp_point){
                            players[0].deselect();
                        }
                    }
                }
                _ => {}
            }
        }

        //Logic Processing segment
        //Camera Movement 
        player_cam.move_cam(&game_map);
        
        { //Checks for completed constructions
            let mut i: usize = 0;
            while i < players[0].buildings.len() {
                if players[0].buildings[i].constructing.is_some() {
                    if players[0].buildings[i].construction_done() {
                        let temp_building = players[0].buildings[i].get_constructed(&tx_mgr)
                            .to_owned();
                        players[0].buildings.push(temp_building.unwrap().to_owned());
                        players[0].buildings[i].constructing = None;
                    }
                }
                i += 1;
            }
        } 

        //Rendering segment (order: world -> objects -> buildings/units -> UI)
        {
            let temp_players = players.to_owned();

            let _ = canvas.with_texture_canvas(&mut buffer, |texture_canvas| {
                //Game world (map)
                texture_canvas.clear();
                texture_canvas.set_viewport(Rect::new(45, 45,
                    (game_map.world_sprites.len() + 1) as u32 *
                        game_map.world_sprites[0][0].loc_rect.w as u32 + 50,
                    game_map.world_sprites[0].len() as u32 *
                        game_map.world_sprites[0][0].loc_rect.h as u32 + 50));
                game_map.render(texture_canvas, player_cam.viewport,
                    players[0].placing_building, &tx_mgr);
                
                //World objects (decorations, obsticles, cliffs and similar)
                for object in objects.iter() {
                    object.render(&tx_mgr, texture_canvas);
                }

                //Buildings/Units (all player or AI made buildings and units)
                for player in temp_players.iter() {
                    player.render_owned(&tx_mgr, texture_canvas);
                }
            });

            //Copy vieport from buffer
            canvas.copy(&buffer, player_cam.viewport, canvas.viewport())
                .expect("buffer coppy error");
        }

        //UI
        players[0].render_ui(&tx_mgr, &mut canvas);

        canvas.present();
        
        avg += temp_timer.elapsed().as_nanos() as f64 / 1_000_000f64;
        count += 1f64;
        temp_timer.stop();
        temp_timer.reset();
        
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 72));
    }

    println!("avg: {} ms", avg / count);
}
