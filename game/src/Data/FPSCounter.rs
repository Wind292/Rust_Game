use sdl2::rect::Rect;
use crate::Direction;
use crate::UtilType;
use std::time::Instant;
pub struct FPSCounter {
    pub frame_count: u32,
    pub last_update: Instant,
    pub current_fps: u32,
}