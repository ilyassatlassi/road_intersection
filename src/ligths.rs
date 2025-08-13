use crate::{Roads::Roads, vehicle::*};
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
    pub fn update(&mut self, vehicles: &Vec<Vehicle>, roads: &Roads) {
        self.timer += 1;

        let should_extend = self.should_extend_phase(vehicles);
        if self.timer >= self.phase_duration {
            self.is_timeover = true;
            // self.next_phase();
        }

        if self.timer >= self.phase_duration && !should_extend {
            self.next_phase(roads);
            self.timer = 0;
            self.is_timeover = false;
        }
    }

    pub fn should_extend_phase(&self, vehicles: &Vec<Vehicle>) -> bool {
        let current_direction = match self.phase {
            Direction::Up => "up",
            Direction::Down => "down",
            Direction::Left => "left",
            Direction::Right => "right",
        };

        vehicles
            .iter()
            .any(|v| v.direction == current_direction && v.is_in_intersection())
    }

    pub fn next_phase(&mut self, roads: &Roads) {
        let max = roads.full(); // (String, i32)

        if max.1 >= 4 && self.timer.saturating_sub(self.phase_duration) < 30 {
            self.phase = match max.0.as_str() {
                "up" => Direction::Up,
                "down" => Direction::Down,
                "left" => Direction::Left,
                "right" => Direction::Right,
                _ => self.phase.clone(), // fallback if somehow invalid
            };
            return; // Done updating phase
        }

        // Rotate to the next phase
        self.phase = match self.phase {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Up,
        };
    }

    pub fn can_vehicle_proceed(&self, vehicle: &Vehicle, traffic: &TrafficSystem) -> bool {
        if !vehicle.is_approaching_intersection() {
            return true;
        }

        if vehicle.is_approaching_intersection() && traffic.timer > traffic.phase_duration {
            return false;
        }

        let allowed_direction = match self.phase {
            Direction::Up => "up",
            Direction::Down => "down",
            Direction::Left => "left",
            Direction::Right => "right",
        };
        vehicle.direction == allowed_direction
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
