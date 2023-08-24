extern crate sdl2;

use sdl2::event;
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
    right: bool,
    up: bool,
    left: bool,
    down: bool
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
#[derive(PartialEq, Eq, Debug)]
enum Direction {
    Up,Down,Left,Right
}
#[derive(PartialEq, Eq, Debug)]
enum UtilType{
    Arrow
}
#[derive(PartialEq, Eq, Debug)]
struct UtilEntity{
    RectObj: Rect,
    Dir: Direction,
    Type: UtilType,
    Speed: i32,
    Damage: i32,
    Health: i32,
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

fn main() -> Result<(), String> {



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
        right: false,
        up: false,
        left: false,
        down: false
    };
    let mut keys_pressed_at_frame = KeyState{
        w: false,
        a: false,
        s: false,
        d: false,
        right: false,
        up: false,
        left: false,
        down: false
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


fn manage_player_class(player_class: &Class, keys:&KeyState, key_pressed_at_frame: KeyState ,canvas: &mut Canvas<Window>,enviroment: &mut (Vec<Rect>, Vec<Rect>, Vec<Rect>, Vec<UtilEntity>, Vec<Enemy>),util_count:&mut i32){
    println!("{:?}",util_count);
    draw_string(util_count.to_string(), 20, canvas, (255,255,0), 0, 0);
    match player_class {
        Class::Archer => {
            if util_count > &mut 0{
                if key_pressed_at_frame.up{
                    enviroment.3.push(UtilEntity{
                        RectObj:Rect::new((SCREEN_WIDTH/2) as i32, (SCREEN_HEIGHT/2) as i32, 10, 30),
                        Damage: 0,
                        Dir: Direction::Up,
                        Type: UtilType::Arrow,
                        Speed: -25,
                        Health: 1,
                    });
                    *util_count -= 1;
                }
                else if key_pressed_at_frame.down{
                    enviroment.3.push(UtilEntity{
                        RectObj:Rect::new((SCREEN_WIDTH/2) as i32, (SCREEN_HEIGHT/2) as i32, 10, 30),
                        Damage: 0,
                        Dir: Direction::Down,
                        Type: UtilType::Arrow,
                        Speed: 25,
                        Health: 1,
                    });
                    *util_count -= 1;
                }  
                else if key_pressed_at_frame.left{
                    enviroment.3.push(UtilEntity{
                        RectObj:Rect::new((SCREEN_WIDTH/2) as i32, (SCREEN_HEIGHT/2) as i32, 30, 10),
                        Damage: 0,
                        Dir: Direction::Left,
                        Type: UtilType::Arrow,
                        Speed: -25,
                        Health: 1,
                    });
                    *util_count -= 1;
                }
                else if key_pressed_at_frame.right{
                    enviroment.3.push(UtilEntity{
                        RectObj:Rect::new((SCREEN_WIDTH/2) as i32, (SCREEN_HEIGHT/2) as i32, 30, 10),
                        Damage: 0,
                        Dir: Direction::Right,
                        Type: UtilType::Arrow,
                        Speed: 25,
                        Health: 1,
                    });
                    *util_count -= 1;
                }  
            }



        }, _=>{}
    
    }

    let mut elements_to_remove: Vec<usize> = Vec::new(); // Store indices of elements to remove

    for (index, util) in enviroment.3.iter_mut().enumerate() {
        match util.Type {
            UtilType::Arrow => {
                if util.Health > 0{
                    match util.Dir {
                        Direction::Down | Direction::Up => {
                            util.RectObj.y += util.Speed;
                            if util.RectObj.y > (SCREEN_HEIGHT * 2) as i32 || util.RectObj.y < -((SCREEN_HEIGHT * 2) as i32){
                                elements_to_remove.push(index); // Mark for removal
                            }
                        },
                        Direction::Left | Direction::Right => {
                            util.RectObj.x += util.Speed;
                            if util.RectObj.x > (SCREEN_WIDTH * 2) as i32 || util.RectObj.x < -((SCREEN_WIDTH * 2) as i32){
                                elements_to_remove.push(index); // Mark for removal
                            }
                        },
                    _ => {}
                    }    
                }
            }
        }
    }

    for &index in elements_to_remove.iter().rev() {
        enviroment.3.remove(index);
    }
    
}

fn open_file(dir: &str) -> String {
    let contents = fs::read_to_string(dir).expect("Should have been able to read the file");
    contents
}
//red     //green  //yellow
fn load_map(enviroment: &mut (Vec<Rect>, Vec<Rect>, Vec<Rect>, Vec<UtilEntity>, Vec<Enemy>), file: String) {
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

fn handle_movement(
    keys: &KeyState,
    player_speed: &i32,
    environment: &mut (Vec<Rect>, Vec<Rect>, Vec<Rect>, Vec<UtilEntity>, Vec<Enemy>),
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
        for rect in environment.0.iter_mut()
        .chain(environment.1.iter_mut())
        .chain(environment.2.iter_mut())
        .chain(environment.3.iter_mut().map(|item| &mut item.RectObj))
    
        {
            if rect.has_intersection(square) {
                move_back = true;
            }
        }
        if move_back {
            square.y += player_speed;
        } else {
            for rect in environment.0.iter_mut()
            .chain(environment.1.iter_mut())
            .chain(environment.2.iter_mut())
            .chain(environment.3.iter_mut().map(|item| &mut item.RectObj))
        
            {
                rect.y += player_speed;
            }
        }
    }

    if keys.s {
        let mut move_back = false;
        square.y += player_speed;
        for rect in environment.0.iter_mut()
        .chain(environment.1.iter_mut())
        .chain(environment.2.iter_mut())
        .chain(environment.3.iter_mut().map(|item| &mut item.RectObj))
    
        {
            if rect.has_intersection(square) {
                move_back = true;
            }
        }
        if move_back {
            square.y -= player_speed;
        } else {
            for rect in environment.0.iter_mut()
            .chain(environment.1.iter_mut())
            .chain(environment.2.iter_mut())
            .chain(environment.3.iter_mut().map(|item| &mut item.RectObj))
        
            {
                rect.y -= player_speed;
            }
        }
    }

    if keys.a {
        let mut move_back = false;
        square.x -= player_speed;
        for rect in environment.0.iter_mut()
        .chain(environment.1.iter_mut())
        .chain(environment.2.iter_mut())
        .chain(environment.3.iter_mut().map(|item| &mut item.RectObj))
    
        {
            if rect.has_intersection(square) {
                move_back = true;
            }
        }
        if move_back {
            square.x += player_speed;
        } else {
            for rect in environment.0.iter_mut()
            .chain(environment.1.iter_mut())
            .chain(environment.2.iter_mut())
            .chain(environment.3.iter_mut().map(|item| &mut item.RectObj))
        
            {
                rect.x += player_speed;
            }
        }
    }

    if keys.d {
        let mut move_back = false;
        square.x += player_speed;
        for rect in environment.0.iter_mut()
        .chain(environment.1.iter_mut())
        .chain(environment.2.iter_mut())
        .chain(environment.3.iter_mut().map(|item| &mut item.RectObj))
    
        {
            if rect.has_intersection(square) {
                move_back = true;
            }
        }
        if move_back {
            square.x -= player_speed;
        } else {
            for rect in environment.0.iter_mut()
            .chain(environment.1.iter_mut())
            .chain(environment.2.iter_mut())
            .chain(environment.3.iter_mut().map(|item| &mut item.RectObj))
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
    
    let mut util_count = 10; 

    let mut enviroment: (Vec<Rect>, Vec<Rect>, Vec<Rect>, Vec<UtilEntity>, Vec<Enemy>) = (vec![], vec![], vec![], vec![], vec![]);

    let mut square = Rect::new(
        ((SCREEN_WIDTH / 2) - 50) as i32,
        ((SCREEN_HEIGHT / 2) - 50) as i32,
        100,
        100,
    );

    load_map(&mut enviroment, open_file(MAP_DIRECTORY));

    'running: loop {
        let mut keys_pressed_at_frame = KeyState{
            w: false,
            a: false,
            s: false,
            d: false,
            right: false,
            up: false,
            left: false,
            down: false
        };
    


        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown { keycode, .. } => match keycode {
                    Some(Keycode::Escape) => break 'running,
                    Some(Keycode::A) => {keys.a = true;keys_pressed_at_frame.a = true},
                    Some(Keycode::D) => {keys.d = true;keys_pressed_at_frame.d = true},
                    Some(Keycode::S) => {keys.s = true;keys_pressed_at_frame.s = true},
                    Some(Keycode::W) => {keys.w = true;keys_pressed_at_frame.w = true},
                    Some(Keycode::Right) => {keys.right = true;keys_pressed_at_frame.right = true},
                    Some(Keycode::Up) => {keys.up = true;keys_pressed_at_frame.up = true},
                    Some(Keycode::Down) => {keys.down = true;keys_pressed_at_frame.down = true},
                    Some(Keycode::Left) => {keys.left = true;keys_pressed_at_frame.left = true},                
                    _ => {}
                },
                Event::KeyUp { keycode, .. } => match keycode {
                    Some(Keycode::A) => keys.a = false,
                    Some(Keycode::D) => keys.d = false,
                    Some(Keycode::S) => keys.s = false,
                    Some(Keycode::W) => keys.w = false,
                    Some(Keycode::Right) => keys.right = false,
                    Some(Keycode::Up) => keys.up = false,
                    Some(Keycode::Down) => keys.down = false,
                    Some(Keycode::Left) => keys.left = false,




                    
                    _ => {}
                },
                
                
                _ => {}
            }
        }

        // LOGIC CODE BELOW

        handle_movement(&keys, &player_speed, &mut enviroment); // handle movement and camera movement

        manage_player_class(&player_class, keys, keys_pressed_at_frame, canvas,&mut enviroment, &mut util_count);
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
        for util in &enviroment.3 {
            //RED
            if util.RectObj.has_intersection(check_in_frame_rect) {
                canvas.set_draw_color(Color::RGB(255, 0, 0));
                canvas.fill_rect(util.RectObj).unwrap();
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



fn draw_string(
    number: String,
    size: u32,
    canvas: &mut Canvas<Window>,
    rgb: (u8, u8, u8),
    x: i32,
    y: i32,
) {
    let char1 = String::from(
        "
    01100
    00100
    00100
    00100
    01110",
    );
    let char2 = String::from(
        "
    01100
    00010
    00100
    01000
    01110",
    );
    let char3 = String::from(
        "
    01100
    00010
    01100
    00010
    01100",
    );
    let char4 = String::from(
        "
    01010
    01010
    01110
    00010
    00010",
    );
    let char5 = String::from(
        "
    01110
    01000
    00100
    00010
    01110",
    );
    let char6 = String::from(
        "
    01110
    01000
    01110
    01010
    01110",
    );
    let char7 = String::from(
        "
    01110
    00010
    00100
    00100
    01000",
    );
    let char8 = String::from(
        "
    01110
    01010
    00100
    01010
    01110",
    );
    let char9 = String::from(
        "
    01110
    01010
    01110
    00010
    00010",
    );
    let char0 = String::from(
        "
    01110
    01010
    01010
    01010
    01110",
    );
    let charspc = String::from(
        "
    00000
    00000
    00000
    00000
    00000",
    );

    canvas.set_draw_color(Color::RGB(rgb.0, rgb.1, rgb.2));
    let mut string_location: u32 = 0;
    for input_char in number.chars() {
        match input_char {
            '1' => draw_char(input_char, canvas, size, &string_location, &char1, x, y),
            '2' => draw_char(input_char, canvas, size, &string_location, &char2, x, y),
            '3' => draw_char(input_char, canvas, size, &string_location, &char3, x, y),
            '4' => draw_char(input_char, canvas, size, &string_location, &char4, x, y),
            '5' => draw_char(input_char, canvas, size, &string_location, &char5, x, y),
            '6' => draw_char(input_char, canvas, size, &string_location, &char6, x, y),
            '7' => draw_char(input_char, canvas, size, &string_location, &char7, x, y),
            '8' => draw_char(input_char, canvas, size, &string_location, &char8, x, y),
            '9' => draw_char(input_char, canvas, size, &string_location, &char9, x, y),
            '0' => draw_char(input_char, canvas, size, &string_location, &char0, x, y),

            ' ' => draw_char(input_char, canvas, size, &string_location, &charspc, x, y),
            _ => {}
        }

        string_location += 1;
    }
}
fn draw_char(
    _c: char,
    canvas: &mut Canvas<Window>,
    size: u32,
    string_location: &u32,
    charbit: &String,
    x: i32,
    y: i32,
) {
    let mut pos = 0;
    for bit in charbit.chars() {
        if bit == '1' {
            canvas
                .fill_rect(rect_on_5x5_grid(pos, size, *string_location, x, y))
                .unwrap();
        }
        pos += 1;
    }
}

fn rect_on_5x5_grid(number: i32, size: u32, string_location: u32, x: i32, y: i32) -> Rect {
    Rect::new(
        ((number % 5) * size as i32) + (string_location * size * 4) as i32 + x,
        (number / 5) * (size / 2) as i32 + y,
        size,
        size,
    )
}
