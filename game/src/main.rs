extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::env;
use std::time::Duration;

const FPS: u32 = 60;

const CAPTION: &str = "GAME";

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;

// #[derive(PartialEq, Eq, Debug)] // lets you do !=, == and print it
struct KeyState {
    w: bool,
    s: bool,
    a: bool,
    d: bool,
}

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
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
    let mut enviroment: Vec<Rect> = vec![];

    // VAR DECLARES

    let player_speed = 5;
    let mut square = Rect::new(
        ((SCREEN_WIDTH / 2) - 50) as i32,
        ((SCREEN_HEIGHT / 2) - 50) as i32,
        100,
        100,
    );

    enviroment.push(Rect::new(12, 12, 100, 100));
    enviroment.push(Rect::new(200, 200, 100, 100));
    enviroment.push(Rect::new(100, 100, 100, 100));

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

        for rect in &mut enviroment {
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
        for rect in &enviroment{
            if rect.has_intersection(square){
                println!("intersected")
            }
        }
        // DRAW CODE BELOw

        //Set background
        canvas.set_draw_color(Color::RGB(255, 50, 50));
        canvas.present();
        canvas.clear();

        //Draw other things

        for rect in &enviroment {
            canvas.set_draw_color(Color::RGB(0, 255, 0));
            canvas.fill_rect(*rect).unwrap();
        }

        canvas.set_draw_color(Color::RGB(0, 0, 255));
        canvas.fill_rect(square).unwrap();

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / FPS));
    }

    Ok(())
}
