extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::EventPump;
use std::collections::btree_map::Keys;
use std::env;
use std::fs;
use std::time::{Duration, Instant};

const FPS: u32 = 60;
const CAPTION: &str = "GAME";
const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;
const MAP_DIRECTORY: &str = "maps/testlevel.mp"; // can have a file extention of anything

const CUBE_SIZE: u32 = 100;

struct KeyState {
    w: bool,
    s: bool,
    a: bool,
    d: bool,
}
//FPS COUNTER STUFF
struct FPSCounter {
    frame_count: u32,
    last_update: Instant,
    current_fps: u32,
}
struct Enemy{
    Speed:i32 ,
    GuyRec: Rect
}
#[derive(PartialEq, Eq, Debug)] // lets you do !=, == and print it
enum Class {
    Archer,
    Swordsman,
    Mage,
    Tank,
}

#[derive(PartialEq, Eq, Debug)] // lets you do !=, == and print it
enum Stage {
    Testing,
    ChoosingClass,
    L1,
    L2,
    L3,
    L4,
    L5,
}

impl FPSCounter {
    fn new() -> Self {
        FPSCounter {
            frame_count: 0,
            last_update: Instant::now(),
            current_fps: 0, // Initialize current FPS
        }
    }

    fn tick(&mut self) {
        self.frame_count += 1;
        let now = Instant::now();
        if now.duration_since(self.last_update) >= Duration::from_secs(1) {
            self.current_fps = self.frame_count;
            self.frame_count = 0;
            self.last_update = now;
        }
    }

    fn get_current_fps(&self) -> u32 {
        self.current_fps
    }
}

fn main( ) -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let mut window = video_subsystem
        .window(CAPTION, SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let mut event_pump = sdl_context.event_pump()?;


    // constant vars through stages
    // keys
    let mut keys = KeyState {
        w: false,
        a: false,
        s: false,
        d: false,
    };

    let check_in_frame_rect = Rect::new(0, 0, SCREEN_WIDTH, SCREEN_HEIGHT);
    let mut fps_counter = FPSCounter::new();
    let player_speed = 10;
    let player_class = Class::Archer;
    let mut current_stage = Stage::Testing;
    
    
    match current_stage {

        Stage::Testing => stage_testing(&mut event_pump, &mut keys, &player_speed, &mut canvas, check_in_frame_rect, &mut fps_counter,player_class),

        _=>{}
    }




    Ok(())
}


fn mainmenu(
    event_pump: &mut EventPump,
    keys: &mut KeyState,
    player_speed: &i32,
    canvas: &mut Canvas<Window>,
    check_in_frame_rect: Rect,
    fps_counter: &mut FPSCounter,
    player_class: Class
) {
    let mut is_hovered = false;
    let sdl_context = sdl2::init().unwrap();
    let button_rect = Rect::new(100, 100, 100, 50);

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
       
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                }
                Event::MouseMotion { x, y, .. } => {
                    is_hovered = button_rect.contains_point((x, y));
                }
                Event::MouseButtonDown { x, y, .. } => {
                    if is_hovered {
                        println!("Button clicked!");
                    }
                }
                _ => {}
            }
        }

    

        // LOGIC CODE BELOW

      

        manage_player_class(&player_class, keys, canvas);
        // DRAW CODE BELOW

        //Set background
        canvas.set_draw_color(Color::RGB(100, 100, 100));
        canvas.present();
        canvas.clear();


        canvas.set_draw_color(Color::RGB(0, 0, 255));
        

        fps_counter.tick();

        let current_fps = fps_counter.get_current_fps();
        let new_title = format!("FPS: {} / {}", current_fps, FPS);
        canvas.window_mut().set_title(&new_title).unwrap();

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / FPS));
    }
}
fn DumbEnemyNav(enviroment: &mut (Vec<Rect>,Vec<Rect>,Vec<Rect>,Vec<Rect>,Vec<Enemy>, ),canvas: &mut Canvas<Window>,check_in_frame_rect: Rect,){
    
        for rect in &mut enviroment.4 {
            //RED
            if rect.GuyRec.has_intersection(check_in_frame_rect) {
                if rect.GuyRec.x <  ((SCREEN_WIDTH / 2) - 50) as i32{
                        rect.GuyRec.x += rect.Speed;
                }
            }
        }
        'running: loop {
       
            for rect in &mut enviroment.4 {
            //RED
            if rect.GuyRec.has_intersection(check_in_frame_rect) {
                if rect.GuyRec.x <  ((SCREEN_WIDTH / 2) - 50) as i32{
                        rect.GuyRec.x += rect.Speed;
                }
            }
        }
    }
}

fn manage_player_class(player_class: &Class,keys:&KeyState,canvas: &mut Canvas<Window>){

    match player_class {
        Class::Archer => {

        }
        _=>{}
    }
    

    



}

fn open_file(dir: &str) -> String {
    let contents = fs::read_to_string(dir).expect("Should have been able to read the file");
    contents
}
//red     //green  //yellow
fn load_map(enviroment: &mut (Vec<Rect>,Vec<Rect>,Vec<Rect>,Vec<Rect>,Vec<Enemy>, ), file: String) {
    let mut skip = false;
    let mut yval = 0;
    let mut xval = 0;

    for char in file.chars() {
        if char == '\n' {
            skip = false;
            xval = -1;
            yval += 1;
        }

        if skip == false {
            if char == '#' {
                // coment
                skip = true;
            } else if char == '&' {
                //red
                enviroment.0.push(Rect::new(
                    (xval as u32 * CUBE_SIZE) as i32,
                    (yval * CUBE_SIZE) as i32,
                    CUBE_SIZE,
                    CUBE_SIZE,
                ));
            } else if char == '%' {
                //green
                enviroment.1.push(Rect::new(
                    (xval as u32 * CUBE_SIZE) as i32,
                    (yval * CUBE_SIZE) as i32,
                    CUBE_SIZE,
                    CUBE_SIZE,
                ));
            } else if char == '@' {
                //yellow
                enviroment.2.push(Rect::new(
                    (xval as u32 * CUBE_SIZE) as i32,
                    (yval * CUBE_SIZE) as i32,
                    CUBE_SIZE,
                    CUBE_SIZE,
                ));
            }
            else if  char == '^' {
                //Dumb enemy
                enviroment.4.push(Enemy { Speed: (12), GuyRec: (Rect::new(0, 0, 100, 100)) })
            }

            xval += 1;
        }
    }
}

fn handle_movement(
    keys: &KeyState,
    player_speed: &i32,
    enviroment: &mut (Vec<Rect>,Vec<Rect>,Vec<Rect>,Vec<Rect>,Vec<Enemy>,),
) {
    let mut square = Rect::new(
        ((SCREEN_WIDTH / 2) - 50) as i32,
        ((SCREEN_HEIGHT / 2) - 50) as i32,
        100,
        100,
    );

    if keys.w {
        let mut move_back = false;
        square.y -= player_speed;
        for rect in &mut enviroment
            .0
            .iter_mut()
            .chain(enviroment.1.iter_mut())
            .chain(enviroment.2.iter_mut())
        {
            if rect.has_intersection(square) {
                move_back = true;
            }
        }
        if move_back {
            square.y += player_speed;
        } else {
            for rect in &mut enviroment
                .0
                .iter_mut()
                .chain(enviroment.1.iter_mut())
                .chain(enviroment.2.iter_mut())
                
            {
                rect.y += player_speed;
            }
        }
    }

    if keys.s {
        let mut move_back = false;
        square.y += player_speed;
        for rect in &mut enviroment
            .0
            .iter_mut()
            .chain(enviroment.1.iter_mut())
            .chain(enviroment.2.iter_mut())
        {
            if rect.has_intersection(square) {
                move_back = true;
            }
        }
        if move_back {
            square.y -= player_speed;
        } else {
            for rect in &mut enviroment
                .0
                .iter_mut()
                .chain(enviroment.1.iter_mut())
                .chain(enviroment.2.iter_mut())
                
            {
                rect.y -= player_speed;
            }
        }
    }

    if keys.a {
        let mut move_back = false;
        square.x -= player_speed;
        for rect in &mut enviroment
            .0
            .iter_mut()
            .chain(enviroment.1.iter_mut())
            .chain(enviroment.2.iter_mut())
        {
            if rect.has_intersection(square) {
                move_back = true;
            }
        }
        if move_back {
            square.x += player_speed;
        } else {
            for rect in &mut enviroment
                .0
                .iter_mut()
                .chain(enviroment.1.iter_mut())
                .chain(enviroment.2.iter_mut())
            {
                rect.x += player_speed;
            }
        }
    }

    if keys.d {
        let mut move_back = false;
        square.x += player_speed;
        for rect in &mut enviroment
            .0
            .iter_mut()
            .chain(enviroment.1.iter_mut())
            .chain(enviroment.2.iter_mut())
        {
            if rect.has_intersection(square) {
                move_back = true;
            }
        }
        if move_back {
            square.x -= player_speed;
        } else {
            for rect in &mut enviroment
                .0
                .iter_mut()
                .chain(enviroment.1.iter_mut())
                .chain(enviroment.2.iter_mut())
            {
                rect.x -= player_speed;
            }
        }
    }
}

/*
   _____ _                          _          _               _ 
  / ____| |                        | |        | |             | |
 | (___ | |_ __ _  __ _  ___  ___  | |__   ___| | _____      _| |
  \___ \| __/ _` |/ _` |/ _ \/ __| | '_ \ / _ \ |/ _ \ \ /\ / / |
  ____) | || (_| | (_| |  __/\__ \ | |_) |  __/ | (_) \ V  V /|_|
 |_____/ \__\__,_|\__, |\___||___/ |_.__/ \___|_|\___/ \_/\_/ (_)
                   __/ |                                         
                  |___/                          
*/
 
fn stage_testing(
    event_pump: &mut EventPump,
    keys: &mut KeyState,
    player_speed: &i32,
    canvas: &mut Canvas<Window>,
    check_in_frame_rect: Rect,
    fps_counter: &mut FPSCounter,
    player_class: Class
) {
    
    let mut enviroment: (Vec<Rect>,Vec<Rect>,Vec<Rect>,Vec<Rect>,Vec<Enemy>,) = (vec![], vec![], vec![], vec![],vec![]);

    let mut square = Rect::new(
        ((SCREEN_WIDTH / 2) - 50) as i32,
        ((SCREEN_HEIGHT / 2) - 50) as i32,
        100,
        100,
    );

    load_map(&mut enviroment, open_file(MAP_DIRECTORY));

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown { keycode, .. } => match keycode {
                    Some(Keycode::Escape) => break 'running,
                    Some(Keycode::A) => keys.a = true,
                    Some(Keycode::D) => keys.d = true,
                    Some(Keycode::S) => keys.s = true,
                    Some(Keycode::W) => keys.w = true,
                    _ => {}
                },
                Event::KeyUp { keycode, .. } => match keycode {
                    Some(Keycode::A) => keys.a = false,
                    Some(Keycode::D) => keys.d = false,
                    Some(Keycode::S) => keys.s = false,
                    Some(Keycode::W) => keys.w = false,
                    _ => {}
                },
                _ => {}
            }
        }

        // LOGIC CODE BELOW

        handle_movement(&keys, &player_speed, &mut enviroment); // handle movement and camera movement

        manage_player_class(&player_class, keys, canvas);
        // DRAW CODE BELOW

        //Set background
        canvas.set_draw_color(Color::RGB(100, 100, 100));
        canvas.present();
        canvas.clear();

        //Draw other things

        for rect in &enviroment.0 {
            //YELLOW
            if rect.has_intersection(check_in_frame_rect) {
                canvas.set_draw_color(Color::RGB(255, 255, 0));
                canvas.fill_rect(*rect).unwrap();
            }
        }
        for rect in &enviroment.1 {
            //GREEN
            if rect.has_intersection(check_in_frame_rect) {
                canvas.set_draw_color(Color::RGB(0, 255, 0));
                canvas.fill_rect(*rect).unwrap();
            }
        }
        for rect in &enviroment.2 {
            //RED
            if rect.has_intersection(check_in_frame_rect) {
                canvas.set_draw_color(Color::RGB(255, 0, 0));
                canvas.fill_rect(*rect).unwrap();
            }
        }
        for rect in &enviroment.4 {
            //RED
            if rect.GuyRec.has_intersection(check_in_frame_rect) {
                canvas.set_draw_color(Color::RGB(255, 0, 0));
                canvas.fill_rect(rect.GuyRec).unwrap();
            }
        }
        canvas.set_draw_color(Color::RGB(0, 0, 255));
        canvas.fill_rect(square).unwrap();

        fps_counter.tick();

        let current_fps = fps_counter.get_current_fps();
        let new_title = format!("FPS: {} / {}", current_fps, FPS);
        canvas.window_mut().set_title(&new_title).unwrap();

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / FPS));
    }
}




