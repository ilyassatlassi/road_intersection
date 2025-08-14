use macroquad::prelude::*;
use crate::vehicle::Vehicle;
use crate::roads::Roads;

#[derive(PartialEq, Clone, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(PartialEq, Clone, Debug)]
pub struct TrafficSystem {
    pub phase: Direction,
    pub timer: u32,
    pub phase_duration: u32,
    pub is_timeover: bool,
    pub starve_up: u32,
    pub starve_down: u32,
    pub starve_left: u32,
    pub starve_right: u32,
    pub pending_full_lanes: Vec<Direction>, // lanes we still must serve this round
}

impl TrafficSystem {
    pub fn new() -> Self {
        TrafficSystem {
            phase: Direction::Up,
            timer: 0,
            phase_duration: 180, // ~4s @ 60 FPS
            is_timeover: false,
            starve_up: 0,
            starve_down: 0,
            starve_left: 0,
            starve_right: 0,
            pending_full_lanes: Vec::new(),
        }
    }
    
    // Called every frame
    pub fn update(&mut self, vehicles: &Vec<Vehicle>, roads: &Roads) {
        self.timer = self.timer.saturating_add(1);

        // Update starvation counters: reset the one that is being served and increment others
        match self.phase {
            Direction::Up => {
                self.starve_up = 0;
                self.starve_down = self.starve_down.saturating_add(1);
                self.starve_left = self.starve_left.saturating_add(1);
                self.starve_right = self.starve_right.saturating_add(1);
            }
            Direction::Down => {
                self.starve_down = 0;
                self.starve_up = self.starve_up.saturating_add(1);
                self.starve_left = self.starve_left.saturating_add(1);
                self.starve_right = self.starve_right.saturating_add(1);
            }
            Direction::Left => {
                self.starve_left = 0;
                self.starve_up = self.starve_up.saturating_add(1);
                self.starve_down = self.starve_down.saturating_add(1);
                self.starve_right = self.starve_right.saturating_add(1);
            }
            Direction::Right => {
                self.starve_right = 0;
                self.starve_up = self.starve_up.saturating_add(1);
                self.starve_down = self.starve_down.saturating_add(1);
                self.starve_left = self.starve_left.saturating_add(1);
            }
        }

        let should_extend = self.should_extend_phase(vehicles);
        if self.timer >= self.phase_duration {
            println!("essxt");
            self.is_timeover = true;
        }

        if self.timer >= self.phase_duration && !should_extend {
            println!("ext");
            self.timer = 0;
            self.is_timeover = false;
            self.next_phase(roads);
        }
        
    }

    pub fn should_extend_phase(&self, vehicles: &Vec<Vehicle>) -> bool {
        let curr_str = match self.phase {
            Direction::Up => "up",
            Direction::Down => "down",
            Direction::Left => "left",
            Direction::Right => "right",
        };
        vehicles
            .iter()
            .any(|v| v.direction == curr_str && v.is_in_intersection())
    }

    pub fn next_phase(&mut self, roads: &Roads) {
        // Priority 1: serve heavily loaded lanes
        let full_lanes = roads.full_lanes(5);
        if !full_lanes.is_empty() {
            if self.pending_full_lanes.is_empty() {
                self.pending_full_lanes = full_lanes.clone();
            }
            // remove current phase from pending
            self.pending_full_lanes.retain(|d| *d != self.phase);
            if let Some(next_lane) = self.pending_full_lanes.first().cloned() {
                self.phase = next_lane;
                return;
            }
        }

        // Priority 2: starvation prevention
        let max_starve = self
            .starve_up
            .max(self.starve_down)
            .max(self.starve_left)
            .max(self.starve_right);
        // println!("{}, up: {}, down: {}, left: {}, rigth: {}",max_starve, self.starve_up, self.starve_down, self.starve_left, self.starve_right);
        if max_starve > 60 * 10 {
            // println!("yepp{}, up: {}, down: {}, left: {}, rigth: {}",max_starve, self.starve_up, self.starve_down, self.starve_left, self.starve_right);
            self.phase = if self.starve_up == max_starve {
                Direction::Up
            } else if self.starve_down == max_starve {
                Direction::Down
            } else if self.starve_left == max_starve {
                Direction::Left
            } else {
                Direction::Right
            };
            return;
        }

        // Default rotation
        self.phase = match self.phase {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Up,
        };
    }

    // Decide whether a single vehicle may proceed (approaching = close to intersection)
    pub fn can_vehicle_proceed(&self, vehicle: &Vehicle) -> bool {
        if vehicle.is_in_intersection()  || !vehicle.is_approaching_intersection() || vehicle.has_passed_intersection() {
            return true;
        }

        if !vehicle.is_in_intersection() && self.timer > self.phase_duration {
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

    // light color tuple for drawing (up, down, left, right)
    pub fn get_light_colors(&self) -> (Color, Color, Color, Color) {
        if self.is_timeover {
            return (RED, RED, RED, RED);
        }
        match self.phase {
            Direction::Up => (RED, GREEN, RED, RED),
            Direction::Down => (GREEN, RED, RED, RED),
            Direction::Left => (RED, RED, GREEN, RED),
            Direction::Right => (RED, RED, RED, GREEN),
        }
    }
}
