use crate::vehicle::Vehicle;
use crate::ligths::Direction;

#[derive(Debug, Clone)]
pub struct Roads {
    pub up: Vec<Vehicle>,
    pub down: Vec<Vehicle>,
    pub left: Vec<Vehicle>,
    pub right: Vec<Vehicle>,
}

impl Roads {
    pub fn new() -> Self {
        Roads {
            up: Vec::new(),
            down: Vec::new(),
            left: Vec::new(),
            right: Vec::new(),
        }
    }

    /// Push a vehicle into the appropriate direction queue
    pub fn push(&mut self, v: Vehicle) {
        match v.direction.as_str() {
            "up" => self.up.push(v),
            "down" => self.down.push(v),
            "left" => self.left.push(v),
            "right" => self.right.push(v),
            _ => eprintln!("Unknown direction: {}", v.direction),
        }
    }

    /// Remove first matching vehicle (by equality)
    pub fn pop(&mut self, v: &Vehicle) -> Option<Vehicle> {
        let vec = match v.direction.as_str() {
            "up" => &mut self.up,
            "down" => &mut self.down,
            "left" => &mut self.left,
            "right" => &mut self.right,
            _ => return None,
        };

        if let Some(pos) = vec.iter().position(|x| x == v) {
            Some(vec.remove(pos))
        } else {
            None
        }
    }

    /// Count total vehicles in a direction
    pub fn count_cars(&self, direction: &str) -> usize {
        match direction {
            "up" => self.up.len(),
            "down" => self.down.len(),
            "left" => self.left.len(),
            "right" => self.right.len(),
            _ => 0,
        }
    }

    /// Count vehicles that are approaching or inside the intersection (waiting)
    pub fn count_waiting(&self, direction: &str) -> usize {
        let vec = match direction {
            "up" => &self.up,
            "down" => &self.down,
            "left" => &self.left,
            "right" => &self.right,
            _ => return 0,
        };
        vec.iter()
            .filter(|v| v.is_approaching_intersection() || v.is_in_intersection())
            .count()
    }

    /// Full lanes above threshold
    pub fn full_lanes(&self, threshold: usize) -> Vec<Direction> {
        let mut result = Vec::new();
        if self.up.len() >= threshold {
            result.push(Direction::Up);
        }
        if self.down.len() >= threshold {
            result.push(Direction::Down);
        }
        if self.left.len() >= threshold {
            result.push(Direction::Left);
        }
        if self.right.len() >= threshold {
            result.push(Direction::Right);
        }
        result
    }
}
