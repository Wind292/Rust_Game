use rand::Rng;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::time::Duration;

//==========GAME SETTINGS============\\
const BLOCK_SIZE: u32 = 25;
const SCREEN_WIDTH_IN_BLOCKS: u32 = 20;
const SCREEN_LENGTH_IN_BLOCKS:u32 = 20;

const GOD_MODE: bool  = false;
const FOOD_COUNT: u32 = 0; //How many food squares spawn when you eat one
const GAME_SPEED: u32 = 50;
//===================================\\



#[derive(PartialEq, Eq, Debug)] // lets you do !=, == and print it
enum Dir {
    Up,
    Down,
    Left,
    Right,
}
#[derive(PartialEq, Eq, Debug)]
enum State {
    Dead,
    Alive,
}
const ONE_BLOCK: i32 = BLOCK_SIZE as i32;
const SCREEN_WIDTH: u32 = BLOCK_SIZE * SCREEN_WIDTH_IN_BLOCKS;
const SCREEN_HEIGHT: u32 = BLOCK_SIZE * SCREEN_LENGTH_IN_BLOCKS;


fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("Snake game", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let starting_leng = 3;
    let mut tail_blocks: Vec<(i32, i32)> = vec![(0, 200)];
    let mut player_dir: Dir = Dir::Right;
    let mut player_state: State = State::Alive;

    let mut tail_length = starting_leng;
    let mut food_blocks: Vec<(i32, i32)> = vec![(400, 200)];

    let mut head_position = Rect::new(0, 200, BLOCK_SIZE, BLOCK_SIZE);

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,

                // Handle other key presses here
                Event::KeyDown { keycode, .. } => {
                    if let Some(key) = keycode {
                        match key {
                            Keycode::W => {
                                // Handle left arrow key press
                                if player_dir != Dir::Down {
                                    player_dir = Dir::Up;
                                    break;
                                }
                            }
                            Keycode::S => {
                                // Handle left arrow key press
                                if player_dir != Dir::Up {
                                    player_dir = Dir::Down;
                                    break;
                                }
                            }
                            Keycode::A => {
                                // Handle left arrow key press
                                if player_dir != Dir::Right {
                                    player_dir = Dir::Left;
                                    break;
                                }
                            }
                            Keycode::D => {
                                // Handle left arrow key press
                                if player_dir != Dir::Left {
                                    player_dir = Dir::Right;
                                    break;
                                }
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }

        let start_index = tail_blocks.len().saturating_sub(tail_length);
        let alive_tail_blocks = &tail_blocks[start_index..&tail_blocks.len() - 1];
        //DEAD CHECK
        for coords in alive_tail_blocks {
            //Loops for the last tail_lenth elements in tail_blocks
            if coords.0 == head_position.x() && coords.1 == head_position.y() {
                // if the head is colliding with the tail
                player_state = State::Dead;
            }
        }

        

        if player_dir == Dir::Up {
            head_position.set_y(head_position.y() - ONE_BLOCK);
        } else if player_dir == Dir::Down {
            head_position.set_y(head_position.y() + ONE_BLOCK);
        } else if player_dir == Dir::Left {
            head_position.set_x(head_position.x() - ONE_BLOCK);
        } else if player_dir == Dir::Right {
            head_position.set_x(head_position.x() + ONE_BLOCK);
        }

        if head_position.y() > SCREEN_HEIGHT as i32 - ONE_BLOCK {
            player_state = State::Dead;
        }
        if head_position.y() < 0 {
            player_state = State::Dead;
        }
        if head_position.x() > SCREEN_HEIGHT as i32 - ONE_BLOCK {
            player_state = State::Dead;
        }
        if head_position.x() < 0 {
            player_state = State::Dead;
        }

        if GOD_MODE {
            player_state = State::Alive;
        }

        if player_state == State::Alive {
            tail_blocks.push((head_position.x, head_position.y));

            canvas.set_draw_color(Color::RGB(0, 255, 0)); // background color

            canvas.clear();

            let mut new_food_blocks = Vec::new(); // Temporary vector to collect food blocks

            for food in &food_blocks {
                if food.0 == head_position.x() && food.1 == head_position.y() {
                    // food gets eaten
                    tail_length += 1;
                    for _foodpiece in 0..FOOD_COUNT + 1 {
                        let xloc = rand::thread_rng().gen_range(0..(SCREEN_WIDTH/BLOCK_SIZE) as i32);
                        let yloc = rand::thread_rng().gen_range(0..(SCREEN_HEIGHT/BLOCK_SIZE) as i32);
                        new_food_blocks.push((xloc * BLOCK_SIZE as i32, yloc * BLOCK_SIZE as i32));
                    }
                } else {
                    // food is not on head block so does not get eaten
                    new_food_blocks.push(*food);

                    canvas.set_draw_color(Color::RGB(255, 255, 0));
                    canvas
                        .fill_rect(Rect::new(food.0, food.1, BLOCK_SIZE, BLOCK_SIZE))
                        .unwrap();
                }
            }
            food_blocks = new_food_blocks;

            let start_index = tail_blocks.len().saturating_sub(tail_length);

            let alive_tail_blocks = &tail_blocks[start_index..&tail_blocks.len() - 1];

            let mut gradiant: u16 = 50;
            for coords in alive_tail_blocks {
                //Loops for the last tail_lenth elements in tail_blocks
                if gradiant > 255 * 5 {
                    canvas.set_draw_color(Color::RGB(255, 255 as u8, (255 * 5) as u8));
                } else if gradiant > 255 * 4 {
                    canvas.set_draw_color(Color::RGB((gradiant - 255 * 4) as u8, 255 as u8, 255));
                } else if gradiant > 255 * 3 {
                    canvas.set_draw_color(Color::RGB(
                        255 - (gradiant - 255 * 3) as u8,
                        255 as u8,
                        (gradiant - 255 * 3) as u8,
                    ));
                } else if gradiant > 255 * 2 {
                    canvas.set_draw_color(Color::RGB(255 as u8, (gradiant - 255 * 2) as u8, 0));
                } else if gradiant > 255 {
                    canvas.set_draw_color(Color::RGB(255 as u8, 0, 255 - ((gradiant - 255) as u8)));
                } else {
                    canvas.set_draw_color(Color::RGB(0 + gradiant as u8, 0, 255));
                }

                canvas
                    .fill_rect(Rect::new(coords.0, coords.1, BLOCK_SIZE, BLOCK_SIZE))
                    .unwrap();
                gradiant += 20;
            }

            canvas.set_draw_color(Color::RGB(255, 0, 0));
            canvas.fill_rect(head_position).unwrap();

            canvas.present();

            if tail_blocks.len() > tail_length {
                tail_blocks.drain(..tail_blocks.len().saturating_sub(tail_length));
            }

            // Add a short delay to control the frame rate
            std::thread::sleep(Duration::new(0, 1_000_000_000u32 / GAME_SPEED * 2));
        } else {

            std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 4));
            if tail_length - starting_leng < 10 {
                draw_string(
                    (tail_length - starting_leng).to_string(),
                    50,
                    &mut canvas,
                    (255, 255, 255),
                    100,
                    75,
                );
            } else {
                draw_string(
                    (tail_length - starting_leng).to_string(),
                    50,
                    &mut canvas,
                    (255, 255, 255),
                    50,
                    75,
                );
            }
            canvas.present();
            // canvas.clear();
            std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 4));
            if tail_length - starting_leng < 10 {
                draw_string(
                    (tail_length - starting_leng).to_string(),
                    50,
                    &mut canvas,
                    (210, 210, 0),
                    100,
                    75,
                );
            } else {
                draw_string(
                    (tail_length - starting_leng).to_string(),
                    50,
                    &mut canvas,
                    (210, 210, 0),
                    50,
                    75,
                );
            }
            canvas.present();
        }
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
