extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::f64;
use std::time::Duration;


const FPS: u32 = 60;

const CAPTION: &str = "GAME";


// #[derive(PartialEq, Eq, Debug)] // lets you do !=, == and print it
// enum Dir {
//     Up,
//     Down,
//     Left,
//     Right,
// }



pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window(CAPTION, 800, 600)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let mut event_pump = sdl_context.event_pump()?;

    // VAR DECLARES

    let player_speed = 30;

    let mut square = Rect::new(100, 100, 100, 100);

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                
                Event::KeyDown {
                    keycode: Some(Keycode::W),
                    ..
                        } => square.y -= player_speed,

                Event::KeyDown {
                    keycode: Some(Keycode::S),
                    ..
                        } => square.y += player_speed,

                Event::KeyDown {
                    keycode: Some(Keycode::A),
                    ..
                        } => square.x -= player_speed,

                Event::KeyDown {
                    keycode: Some(Keycode::D),
                    ..
                        } => square.x += player_speed,
                        
                _ => {}
            }
                
        }
        // LOGIC CODE BELOW

        // square.y += 1; // adds 1 to the y value of square
        // square.x += 3;

        // DRAW CODE BELOW

        //Set background
        canvas.set_draw_color(Color::RGB(255, 50, 50));
        canvas.present();
        canvas.clear();

        //Draw other things
        canvas.set_draw_color(Color::RGB(0, 0, 255));
        canvas.fill_rect(square).unwrap();

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / FPS));
        // The rest of the game loop goes here...
    }

    Ok(())
}
