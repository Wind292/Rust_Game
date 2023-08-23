extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::env;
use std::fs;
use std::time::{Duration, Instant};
use sdl2::video::Window;
const FPS: u32 = 60;

const CAPTION: &str = "GAME";

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;
const MAP_DIRECTORY: &str = "maps/testlevel.mp"; // can have a file extention of anything

const CUBE_SIZE: u32 = 100;

// #[derive(PartialEq, Eq, Debug)] // lets you do !=, == and print it
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
    current_fps: u32, // Add this field
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

pub fn main() -> Result<(), String> {
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

    // keys
    let mut keys = KeyState {
        w: false,
        a: false,
        s: false,
        d: false,
    };

    // Vectors for enviroment
    let mut enviroment: (Vec<Rect>, Vec<Rect>, Vec<Rect>) = (vec![], vec![], vec![]);

    // VAR DECLARES
    let mut fps_counter = FPSCounter::new();

    let player_speed = 5;
    let mut square = Rect::new(
        ((SCREEN_WIDTH / 2) - 50) as i32,
        ((SCREEN_HEIGHT / 2) - 50) as i32,
        100,
        100,
    );

    let check_in_frame_rect = Rect::new(0,0,SCREEN_WIDTH,SCREEN_HEIGHT);

    // enviroment.0.push(Rect::new(12, 12, 100, 100))
    // enviroment.1.push(Rect::new(200, 200, 100, 100));
    // enviroment.2.push(Rect::new(100, 100, 100, 100));

    compile_file(&mut enviroment, open_file(MAP_DIRECTORY));

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
        for rect in enviroment.0.iter_mut().chain(enviroment.1.iter_mut()).chain(enviroment.2.iter_mut())
        {
            if keys.w {
                rect.y += player_speed;
            }
            if keys.s {
                rect.y -= player_speed;
            }
            if keys.a {
                rect.x += player_speed;
            }
            if keys.d {
                rect.x -= player_speed;
            }
        }
        for rect in &mut enviroment.0.iter_mut().chain(enviroment.1.iter_mut()).chain(enviroment.2.iter_mut()) {
            if rect.has_intersection(square) {
                println!("intersected")
            }
        }
        // DRAW CODE BELOw

        //Set background
        canvas.set_draw_color(Color::RGB(100, 100, 100));
        canvas.present();
        canvas.clear();

        //Draw other things

        for rect in &enviroment.0 {
            //YELLOW
            if rect.has_intersection(check_in_frame_rect){
                canvas.set_draw_color(Color::RGB(255, 255, 0));
                canvas.fill_rect(*rect).unwrap();
            }
        }
        for rect in &enviroment.1 {
            //GREEN
            if rect.has_intersection(check_in_frame_rect){
                canvas.set_draw_color(Color::RGB(0, 255, 0));
                canvas.fill_rect(*rect).unwrap();
            }
        }
        for rect in &enviroment.2 {
            //RED
            if rect.has_intersection(check_in_frame_rect){
            canvas.set_draw_color(Color::RGB(255, 0, 0));
            canvas.fill_rect(*rect).unwrap();
            }
        }

        canvas.set_draw_color(Color::RGB(0, 0, 255));
        canvas.fill_rect(square).unwrap();


        fps_counter.tick();

        let current_fps = fps_counter.get_current_fps();
        let new_title = format!("FPS: {} / {}", current_fps, FPS);
        canvas.window_mut().set_title(&new_title).unwrap();
    

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / FPS));
    



        // LOGIC CODE BELOW
        for rect in enviroment.0.iter_mut().chain(enviroment.1.iter_mut()).chain(enviroment.2.iter_mut())
        {
            if keys.w {
                rect.y += player_speed;
            }
            if keys.s {
                rect.y -= player_speed;
            }
            if keys.a {
                rect.x += player_speed;
            }
            if keys.d {
                rect.x -= player_speed;
            }
        }
        for rect in &mut enviroment.0.iter_mut().chain(enviroment.1.iter_mut()).chain(enviroment.2.iter_mut()) {
            if rect.has_intersection(square) {
                let x_overlap = if square.x < rect.x() {
                    (square.x + square.width() as i32) - rect.x()
                } else {
                    square.x - (rect.x() + rect.width() as i32)
                };
        
                let y_overlap = if square.y < rect.y() {
                    (square.y + square.height() as i32) - rect.y()
                } else {
                    square.y - (rect.y() + rect.height() as i32)
                };
        
                // Adjust rectangle's position based on the smaller overlap value
                if x_overlap.abs() < y_overlap.abs() {
                    rect.x += x_overlap;
                } else {
                    rect.y += y_overlap;
                }
            }
            }
        }
        // DRAW CODE BELOw

        //Set background
        
    Ok(())
}

fn open_file(dir: &str) -> String {
    let contents = fs::read_to_string(dir).expect("Should have been able to read the file");
    contents
}
//red     //green  //yellow
fn compile_file(enviroment: &mut (Vec<Rect>, Vec<Rect>, Vec<Rect>), file: String) {
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

            xval += 1;
        }
    }
}
