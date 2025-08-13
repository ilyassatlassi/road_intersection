use sdl2::pixels::Color;
// use crate::Roads;
// use roads::*;
#[derive(PartialEq, Clone, Debug)]
pub enum Direction {
    Up,
    Down,
    Right,
    Left,
}

#[derive(PartialEq, Clone, Debug)]

pub struct TrafficSystem {
    phase: Direction,
    timer: u32,
    phase_duration: u32,
    is_timeover: bool,
}
impl TrafficSystem {
    pub fn new() -> Self {
        TrafficSystem {
            phase: Direction::Up,
            timer: 0,
            phase_duration: 240,
            is_timeover: false,
        }
    }
    pub fn get_light_colors(&self) -> (Color, Color, Color, Color) {
        if self.is_timeover {
            return (Color::RED, Color::RED, Color::RED, Color::RED);
        }
        match self.phase {
            Direction::Up => (Color::RED, Color::GREEN, Color::RED, Color::RED),
            Direction::Down => (Color::GREEN, Color::RED, Color::RED, Color::RED),
            Direction::Left => (Color::RED, Color::RED, Color::GREEN, Color::RED),
            Direction::Right => (Color::RED, Color::RED, Color::RED, Color::GREEN),
        }
    }
}
