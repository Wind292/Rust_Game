
use sdl2::rect::Rect;
use crate::Data::Direction::Direction;
use crate::Data::UtilType::UtilType;
pub struct UtilEntity{
    pub RectObj: Rect,
    pub Dir: Direction,
    pub Type: UtilType,
    pub Speed: i32,
    pub Damage: i32,
    pub Health: i32,
}