use crate::SCREEN_WIDTH;
use crate::SCREEN_HEIGHT;
use  crate::Data::Class::Class;
use crate::Data::KeyState::KeyState;
use crate::Window;
use sdl2::render::Canvas;
use sdl2::rect::Rect;
use crate::Data::UtilEntity::UtilEntity;
use crate::Enemy;
use crate::Data::Direction::Direction;
use  crate::Data::UtilType::UtilType;
pub fn manage_player_class(player_class: &Class, keys:&KeyState, key_pressed_at_frame: KeyState ,canvas: &mut Canvas<Window>,enviroment: &mut (Vec<Rect>, Vec<Rect>, Vec<Rect>, Vec<UtilEntity>, Vec<Enemy>),util_count:&mut i32){
    println!("{:?}",util_count);
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
