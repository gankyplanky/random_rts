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
use building::{Building, BuildingType, BuildingStatus};
use unit::Unit;
use player::Player;
use ui::{Button, UiElement};

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
    let buttons_texture = texture_loader.load_texture("assets/UI/buttons_placeholders.png");
    let buildings_texture = texture_loader.load_texture("assets/buildings/placeholder_buildings.png");
    let grid_texture = texture_loader.load_texture("assets/ground/grid.png");

    //Rendering vectors
    let mut objects: Vec<WorldObject> = vec![];
    let mut players: Vec<Player> = vec![];

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
            new_encode[1][3] = 1;
            new_encode[2][1] = 1;
            new_encode[2][2] = 1;
            new_encode[2][3] = 1;
            new_encode[3][1] = 1;
            new_encode[3][2] = 1;
            new_encode[3][3] = 1;

            new_encode
        },
        grid_texture.as_ref().unwrap()
    );
    
    let mut buffer : Texture = texture_loader.create_texture_target(
        texture_loader.default_pixel_format(), 
        game_map.world_sprites.len() as u32 * 
            game_map.world_sprites[0][0].width + 100,
        game_map.world_sprites[0].len() as u32 *
            game_map.world_sprites[0][0].height + 100).unwrap();
    
    players.push(Player::new(Faction::PlaceholderFaction1, ui_texture.as_ref().unwrap(), 
        buttons_texture.as_ref().unwrap()));
    
    {
        let temp = players[0].to_owned();

        players[0].buildings.push(Building::new(Point::new(50, 50), BuildingType::CommandCentre,
            Faction::PlaceholderFaction1, 0, buildings_texture.as_ref().unwrap(),
            buttons_texture.as_ref().unwrap(), temp.bottom_right_ui.to_owned()));
        
        players[0].selected_building = Some(0);
        players[0].place_building(&mut game_map);
    }

    //let mut avg: f64 = 0f64;
    //let mut count: f64 = 0f64;

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
                Event::MouseMotion {x, y, .. } => { // Mouse moved
                    player_cam.mouse_panning(x, y); // Mouse map scrolling

                    let mouse_cam_point = Point::new(x, y);
                    if players[0].placing_building { // Move building ghost
                        players[0].dehighlight(&mut game_map);
                        let index = players[0].selected_building.unwrap().to_owned();
                        players[0].buildings[index].move_building(mouse_cam_point,
                            player_cam.viewport, &mut game_map.grid);
                        players[0].buildings[index].highlight_cells(&mut game_map);
                    }
                }
                Event::MouseButtonDown { mouse_btn, x, y, .. } => {

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

                        if !interacted {// Select a building
                            let mut i: usize = 0;
                            while i < players[0].buildings.len() {
                                if players[0].buildings[i].sprite.
                                        rect.contains_point(temp_point) {
                                    players[0].select_building(i);
                                    interacted = true;
                                    break;
                                }
                                i += 1;
                            }
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
                        let temp_building = players[0].buildings[i].get_constructed().to_owned();
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
                        game_map.world_sprites[0][0].width + 50,
                    game_map.world_sprites[0].len() as u32 *
                        game_map.world_sprites[0][0].height + 50));
                game_map.render(texture_canvas, player_cam.viewport,
                    players[0].placing_building);
                
                //World objects (decorations, obsticles, cliffs and similar)
                {
                    let mut i: usize = 0;
                    while i < objects.len() {
                        objects[i].render(texture_canvas);
                        i += 1;
                    }
                }
                //Buildings/Units (all player or AI made buildings and units)
                {
                    for player in temp_players {
                        for mut building in player.buildings {
                            building.render(texture_canvas);
                        }

                        for unit in player.units {
                            //unit.render(texture_canvas);
                        }
                        
                    }
                } 
            });
            
            //Copy vieport from buffer
            canvas.copy(&buffer, player_cam.viewport, canvas.viewport())
                .expect("buffer coppy error");
        }

        //UI
        players[0].render_ui(&mut canvas);

        canvas.present();
        
        //println!("{} ms", temp_timer.elapsed().as_nanos() as f64 / 1_000_000f64);
        //avg += temp_timer.elapsed().as_nanos() as f64 / 1_000_000f64;
        //count += 1f64;
        //temp_timer.stop();
        //temp_timer.reset();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 72));
    }

    //println!("avg: {} ms", avg / count);
}
