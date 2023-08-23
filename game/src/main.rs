extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::time::Duration;


 

const FPS: u32 = 60;

const CAPTION: &str = "GAME";


// #[derive(PartialEq, Eq, Debug)] // lets you do !=, == and print it
struct KeyState {

    w: bool,
    s: bool,
    a: bool,
    d: bool

}


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

    let player_speed = 5;
    let mut square = Rect::new(100, 100, 100, 100);
    
    let mut keys = KeyState{
        w: false,
        a: false,
        s: false,
        d: false,
    };


    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown { keycode, .. } => {
                    match keycode {
                        Some(Keycode::Escape) => break 'running,
                        Some(Keycode::A) => keys.a = true,
                        Some(Keycode::D) => keys.d = true,
                        Some(Keycode::S) => keys.s = true,
                        Some(Keycode::W) => keys.w = true,
                        _=>{}
                    }
                }
                Event::KeyUp { keycode, .. } => {
                    match keycode {
                        Some(Keycode::A) => keys.a = false,
                        Some(Keycode::D) => keys.d = false,
                        Some(Keycode::S) => keys.s = false,
                        Some(Keycode::W) => keys.w = false,
                        _=>{}
                    }
                }
                _ => {}
            }
        }
        // LOGIC CODE BELOW

        if keys.w{
            square.y -= player_speed;
        }

        if keys.s{
            square.y += player_speed;
        }
        
        if keys.a{
            square.x -= player_speed;
        }
        
        if keys.d{
            square.x += player_speed;
        }
        

        



        // DRAW CODE BELOW

        //Set background
        canvas.set_draw_color(Color::RGB(255, 50, 50));
        canvas.present();
        canvas.clear();

        //Draw other things
        canvas.set_draw_color(Color::RGB(0, 0, 255));
        canvas.fill_rect(square).unwrap();

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / FPS));
        
        
    }

    Ok(())
}
