use sdl2::image::LoadTexture;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::{Rect, Point};
use sdl2::render::{Texture, WindowCanvas, TextureCreator};
use sdl2::video::WindowContext;
use glob::glob;
use rand::prelude::Distribution;
use rand::distributions::Uniform;
use strum_macros::Display;

use std::path::PathBuf;
use std::env;
use std::collections::hash_map::HashMap;
use std::cmp::max;

use crate::building::BuildingType;
use crate::general::Faction;

const TEXTURE_BUILDING_WIDTH: u32 = 128;
const TEXTURE_BUILDING_HEIGHT: u32 = 128;
const TEXTURE_UI_WIDTH: u32 = 64;
const TEXTURE_UI_HEIGHT: u32 = 64;
const TEXTURE_WORLD_WIDTH: u32 = 64;
const TEXTURE_WORLD_HEIGHT: u32 = 64;

//Single sprite to render, used in higher abstractions for rendering
#[derive(Clone, Copy)]
pub struct Sprite {
    pub t_type: TextureType,
    pub loc_rect: Rect,
    pub texture_rect: Rect,
}

impl Sprite {
    pub fn new<'f>(loc_rect: Rect, t_type: TextureType, atlas: &'f TextureManager) -> Sprite {
        Sprite {
            t_type,
            loc_rect,
            texture_rect: atlas.get_rect(t_type),
        }
    }
    
    pub fn _get_location<'f>(&'f self) -> Point {
        Point::new(self.loc_rect.x, self.loc_rect.y)
    }
    
    pub fn set_location<'f>(&'f mut self, new_location: Point) {
        self.loc_rect.x = new_location.x;
        self.loc_rect.y = new_location.y;
    }

    pub fn render<'f>(&'f self, atlas: &'f TextureManager, canvas: &'f mut WindowCanvas) {
        canvas.copy(atlas.get_atlas_ref(), self.texture_rect, self.loc_rect)
            .expect("Failed to render texture");
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Display)]
pub enum TextureType {
    Building {faction: Faction, b_type: BuildingType},
    World {tile_index: usize},
    UI {type_index: usize},
    Filler
}

pub struct TextureManager<'t> {
    atlas: Texture<'t>,
    locations: HashMap<TextureType, Rect>, 
    texture_creator: &'t TextureCreator<WindowContext>,
}

impl<'t> TextureManager<'t> {
    pub fn new<'f>(texture_creator: &'t TextureCreator<WindowContext>) -> TextureManager<'t> {
        TextureManager {
            atlas: {
                texture_creator.create_texture_target(texture_creator.default_pixel_format(),
                    1, 1).unwrap()
            },
            texture_creator,
            locations: HashMap::new(),
        }
    }
   
    pub fn _print_all_rects<'f>(&'f self) {
        println!("All available rects: ");
        let _ : HashMap<_, _> = self.locations.iter().map(|(key, value)| {
    
            match key {
                TextureType::Building { faction, b_type } => {
                    println!("  [[{key}: {b_type} {faction}] : [x:{} y:{} w:{} h:{}]]", 
                        value.x, value.y, value.w, value.h);
                },
                TextureType::World { tile_index } => {
                    println!("  [[{key}: {tile_index}] : [x:{} y:{} w:{} h:{}]]", 
                        value.x, value.y, value.w, value.h);
                },
                TextureType::UI { type_index } => {
                    println!("  [[{key}: {type_index}] : [x:{} y:{} w:{} h:{}]]", 
                        value.x, value.y, value.w, value.h);
                },
                TextureType::Filler => {
                    println!("  [[{key}] : [x:{} y:{} w:{} h:{}]]", 
                        value.x, value.y, value.w, value.h);
                },
            }

            (key, value)
        }).collect();
    }

    pub fn get_atlas_ref<'f>(&'f self) -> &'f Texture<'f> {
        &self.atlas
    }
    
    pub fn get_rect<'f>(&'f self, t_type: TextureType) -> Rect {
        let value = self.locations.get(&t_type).to_owned();

        let mut rng = rand::thread_rng();
        let die = Uniform::from(0..3);

        if value.is_some() {
            match t_type {
                TextureType::UI { .. } => {
                    Rect::new(
                        value.unwrap().x,
                        value.unwrap().y,
                        TEXTURE_UI_WIDTH,
                        TEXTURE_UI_HEIGHT)
                },
                TextureType::World { tile_index } => {
                    if tile_index != 0 {
                        Rect::new(
                            die.sample(&mut rng) * value.unwrap().x,
                            value.unwrap().y,
                            TEXTURE_WORLD_WIDTH,
                            TEXTURE_WORLD_HEIGHT )
                    } else {
                        Rect::new(
                            value.unwrap().x,
                            value.unwrap().y,
                            TEXTURE_WORLD_WIDTH,
                            TEXTURE_WORLD_HEIGHT )

                    }
                },
                TextureType::Building { .. } => {
                    Rect::new(
                        value.unwrap().x, 
                        value.unwrap().y, 
                        TEXTURE_BUILDING_WIDTH,
                        TEXTURE_BUILDING_HEIGHT ) 
                },
                TextureType::Filler => {
                    Rect::new(0, 0, 32, 32)
                },
            }
        } else {
            panic!("Request of unloaded texture")
        }
    }

    pub fn get_rect_raw<'f>(&'f self, t_type: TextureType) -> Rect {
        self.locations.get(&t_type).unwrap().to_owned()
    }

    fn get_faction_from_string<'f>(&'f self, str: String) -> Faction {
        if str.contains("placeholderfaction1") {
            return Faction::PlaceholderFaction1;
        }

        panic!("Error when parsing faction from string");
    }

    fn get_building_type_from_string<'f>(&'f self, str: String) -> BuildingType {
        let parts: Vec<&str> = str.split(&['_', '.'][..]).collect();
        let b_type = parts[1].to_lowercase();
        let mut variant: Option<BuildingType> = None;
        let variants = BuildingType::get_all_variants();

        let mut i: usize = 0;
        while i < variants.len() {
            if variants[i].to_string().to_lowercase() == b_type.to_lowercase() {
                variant = Some(variants[i]);
                break;
            }
            i += 1;
        }
        
        if variant.is_some() {
            variant.unwrap()
        } else {
            panic!("Unknows building type!")
        }
    }

    fn get_wh_of_texture<'f>(&'f self, str: String) -> (u32, u32) {
        let temp_texture = self.texture_creator.load_texture(str).unwrap();
        (temp_texture.query().width, temp_texture.query().height)
    }
    
    fn get_texture_type_from_path<'f>(&'f self, path: &'f PathBuf) -> TextureType {
        let parts: Vec<String> = path.to_str().unwrap().to_owned()
            .split("/").map(|str| str.to_string()).collect();
        let len = parts.len();

        let subfolder = parts[len - 2].to_string();
        let name = {
            let temp: Vec<String> = parts[len - 1].to_string().split(".")
                .map(|str| str.to_string()).collect();
            temp[0].to_string()
        };

        match subfolder.as_str() {
            "buildings" => {
                TextureType::Building { 
                    faction: 
                        self.get_faction_from_string(name.to_owned()),
                    b_type:
                        self.get_building_type_from_string(name.to_owned()) }
            },
            "ground" => {
                TextureType::World { 
                    tile_index: {
                        match name.as_str() {
                            "grid" => { 0 },
                            "grass" => { 1 },
                            "dirt" => { 2 },
                            _ => { panic!("Unknown world tile name!") }
                        }
                    } }
            },
            "UI" => {
                TextureType::UI { 
                    type_index: {
                        match name.as_str() {
                            "buttons" => { 0 },
                            "bottom_left_ui" => { 1 },
                            _ => { panic!("Unknown UI name!") }
                        }
                    } }
            },
            "filler" => {
                TextureType::Filler 
            },
            _ => {
                panic!("Error while loading textuers!")
            }
        }
    }

    pub fn update<'f>(&'f mut self, canvas: &'f mut WindowCanvas) {
        if env::current_dir().is_err() {
            panic!("Coulnd't get current directory");
        }

        let current_dir = env::current_dir().unwrap().as_path().to_owned();
        let all_pngs: Vec<PathBuf>;

        {
            let mut temp_str_path = current_dir.to_str().unwrap().to_owned();
            temp_str_path.push_str("/assets/sprites/**/*.png");
            
            all_pngs = glob(temp_str_path.as_str()).unwrap()
                .into_iter().map(|entry| entry.unwrap())
                .collect();
        }
        
        let mut max_width: u32 = 0;
        let mut total_height: u32 = 0;

        self.locations = all_pngs.iter()
            .map(|png| {
                
                let key: TextureType; 
                let value: Rect;
                
                let texture_wh: (u32, u32) = 
                    self.get_wh_of_texture(png.to_str().unwrap().to_string());
                
                key = self.get_texture_type_from_path(png);
                value = Rect::new(
                    0,
                    total_height as i32,
                    texture_wh.0,
                    texture_wh.1 );
                
                max_width = max(max_width, texture_wh.0);
                total_height += texture_wh.1;

                (key, value)
            }).collect();

        let mut buffer: Texture = self.texture_creator.create_texture_target(
            PixelFormatEnum::ARGB32, max_width, total_height).unwrap();
        
        buffer.set_alpha_mod(255);
        buffer.set_blend_mode(sdl2::render::BlendMode::Blend);

        let _ = canvas.with_texture_canvas(&mut buffer, |texture_canvas| {
            texture_canvas.set_viewport(Rect::new(0, 0, max_width, total_height));
            texture_canvas.set_blend_mode(sdl2::render::BlendMode::Blend);
            texture_canvas.set_draw_color(Color::RGBA(0, 0, 0, 0));
            texture_canvas.clear();

            let mut i: usize = 0;
            while i < all_pngs.len() {
                let temp_texture = self.texture_creator.
                    load_texture(all_pngs[i].to_str().unwrap().to_owned()).unwrap();

                texture_canvas.copy(&temp_texture, None, 
                    self.get_rect_raw(self.get_texture_type_from_path(&all_pngs[i])))
                        .expect("Failed to load texture into atlas!");
                i += 1;
            }
        });

        self.atlas = buffer;
    }
}
