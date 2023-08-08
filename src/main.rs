extern crate sdl2;
extern crate stopwatch;

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

//Single sprite to render, used in higher abstractions for rendering
#[derive(Clone, Copy)]
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

//Represents bindings for player camera
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
    viewport: Rect,
    speed: u32,
}

impl Camera {
    fn new(viewport: Rect, up: Keycode, down: Keycode,
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

//Represents current world or map, also used as camera boundary
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
    top_clamp: Point,
    bottom_clamp: Point,
}

enum Collidable {
    GroundCollidable,
    GroundUncollidable,
    AirCollidable,
    AirUncollidable,
    AbsoluteCollidable,
    World,
    UI,
}

impl<'o> WorldObject<'o> {
    fn new(texture_source: &'o Texture, collider: Rect, collider_type: Collidable,
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

    fn render(&mut self, canvas: &mut WindowCanvas) {
        self.sprite.render(canvas);
    }
}

struct Building<'b> {
    sprite: Sprite<'b>,
    team: i32,
    building_type: BuildingType,
    faction: Faction,
    collider_type: Collidable,
    collider: Rect,
    top_clamp: Point,
    bottom_clamp: Point,
}

impl<'b> Building<'b> {
    fn new(location: Point, building_type: BuildingType, faction: Faction,
            team: i32, bottom_clamp: Point, texture_source: &'b Texture<'b>) -> Building<'b> {
        let temp_building_type = building_type.clone();
        let new_building = Building {
            team,
            building_type,
            faction,
            top_clamp: location,
            bottom_clamp,
            collider_type: Collidable::GroundCollidable,
            collider: Rect::new(location.x, location.y, 0, 0),
            sprite: Sprite { 
                texture_source,
                location,
                texture_loaction: {
                    match temp_building_type {
                        BuildingType::CommandCentre => {
                            Rect::new(0, 0, 128, 128)  
                        },
                        BuildingType::Barracks => {
                            Rect::new(0, 0, 64, 64)
                        },
                    }
                },
                width: { 
                    match temp_building_type {
                        BuildingType::CommandCentre => {
                            100  
                        },
                        BuildingType::Barracks => {
                            50
                        },
                    }
                },
                height: { 
                    match temp_building_type {
                        BuildingType::CommandCentre => {
                            100  
                        },
                        BuildingType::Barracks => {
                            50
                        },
                    }
                },
            }
        };

        return new_building;
    }

    fn render(&mut self, canvas: &mut WindowCanvas) {
        self.sprite.render(canvas);
    } 
}

#[derive(Clone, Copy)]
enum BuildingType {
    CommandCentre,
    Barracks,
}

#[derive(Clone, Copy)]
enum Faction {
    PlaceholderFaction1,
}

struct Unit {

}

struct Player<'p> {
    buildings: Vec<Building<'p>>,
    units: Vec<Unit>,
    faction: Faction,
    top_right_ui: UiElement<'p>,
}

impl<'p> Player<'p> {
    fn new(faction: Faction, texture_source: &'p Texture<'p>) -> Player<'p> {
        let new_p = Player {
            buildings: vec![],
            units: vec![],
            faction,
            top_right_ui: UiElement::new(Sprite::new(texture_source, Rect::new(0, 0, 64, 64), 
                Point::new(0, 0), 100, 100)) 
        };

        return new_p;
    }
}

struct UiElement<'u> {
    sprite: Sprite<'u>,
    collider_type: Collidable,
    collider: Rect,
}

impl<'p> UiElement<'p> {
    fn new(sprite: Sprite<'p>) -> UiElement<'p> {
        let temp = sprite.clone();
        let collider = Rect::new(temp.location.x, temp.location.y, temp.width, temp.height);
        let new_ui = UiElement {
            collider_type: Collidable::UI,
            sprite,
            collider 
        };

        return new_ui;
    }

    fn render(&mut self, canvas: &mut WindowCanvas) {
        self.sprite.render(canvas);
    }
}

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
        Keycode::Left, Keycode::Right, 5);
    
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    //Load Textures
    let texture_loader = canvas.texture_creator();
    let none_texture = texture_loader.load_texture("assets/none_sprite.png");
    let ball_texture = texture_loader.load_texture("assets/ball.png"); 

    //Rendering vectors:w
    let mut objects: Vec<WorldObject> = vec![];
    let mut buildings: Vec<&mut Building> = vec![];
    let mut units: Vec<&mut Unit> = vec![];

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
    
    let mut buffer : Texture = texture_loader.create_texture_target(
        texture_loader.default_pixel_format(), 
        game_map.world_sprites.len() as u32 * 50,
        game_map.world_sprites[0].len() as u32 * 50).unwrap();

    let mut timer = Stopwatch::new();
    timer.start();
    
    let mut player = Player::new(Faction::PlaceholderFaction1, ball_texture.as_ref().unwrap());

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
                Event::MouseMotion {x, y, .. } => { // Mouse map scrolling
                    if y >= 0 && y <= 5 {
                        player_cam.move_up = true;
                    } else {
                        player_cam.move_up = false;
                    }
                    
                    if y >= window_height as i32 - 5 && y <= window_height as i32 {
                        player_cam.move_down = true;
                    } else {
                        player_cam.move_down = false;
                    }

                    if x >= 0 && x <= 5 {
                        player_cam.move_left = true;
                    } else {
                        player_cam.move_left = false;
                    }

                    if x >= window_width as i32 - 5 && x <= window_width as i32 {
                        player_cam.move_right = true;
                    } else {
                        player_cam.move_right = false;
                    }

                }
                _ => {}
            }
        }

        //Logic Processing segment
        //Camera Movement - must be done before any other position related calcs
        if player_cam.can_move {
            let mut x_move: i32 = 0;
            let mut y_move: i32 = 0;
            
            if player_cam.move_up {
                y_move -= player_cam.speed as i32;
            }
            if player_cam.move_down {
                y_move += player_cam.speed as i32;
            }
            if player_cam.move_left {
                x_move -= player_cam.speed as i32;
            }
            if player_cam.move_right {
                x_move += player_cam.speed as i32;
            }
            
            player_cam.viewport.set_x(player_cam.viewport.x + x_move);
            player_cam.viewport.set_y(player_cam.viewport.y + y_move);

            player_cam.viewport.set_x(max(player_cam.viewport.x, 0));
            player_cam.viewport.set_y(max(player_cam.viewport.y, 0));

            player_cam.viewport.set_x(min(player_cam.viewport.x,
                (game_map.world_sprites.len() as i32 - 1) * 50 - window_width as i32 + 10));
            player_cam.viewport.set_y(min(player_cam.viewport.y, 
                game_map.world_sprites[0].len() as i32 * 50 - window_height as i32 + 10));
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
        
        //Rendering segment (order: world -> object -> buildings -> units -> UI)
        let _ = canvas.with_texture_canvas(&mut buffer, |texture_canvas| {
            //Game world (map)
            texture_canvas.clear();
            texture_canvas.set_viewport(Rect::new(5, 5,
                (game_map.world_sprites.len() + 1) as u32 * 50 + 5,
                game_map.world_sprites[0].len() as u32 * 50 + 5));
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
        
        
            //UI
            /*player.top_right_ui.render(&mut canvas);*/
        });
         
    
        canvas.copy(&buffer, player_cam.viewport, canvas.viewport())
            .expect("buffer coppy error");
        canvas.present();
        
        /*println!("{}", temp_timer.elapsed().as_nanos());
        temp_timer.stop();
        temp_timer.reset();*/
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 144));
    }
}
