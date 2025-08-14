use macroquad::prelude::*;
use ::rand::Rng;

#[derive(Debug, PartialEq, Clone)]
pub struct Vehicle {
    pub car: Rect, // x, y, w, h (f32)
    pub direction: String,
    pub random_route: String,
    pub color: Color,
    pub speed: f32,
}

impl Vehicle {
    pub fn new(car: Rect, direction: String, random_route: String, color: Color) -> Vehicle {
        Vehicle {
            car,
            direction,
            random_route,
            color,
            speed: 1.0,
        }
    }

    pub fn draw(&self) {
        draw_rectangle(self.car.x, self.car.y, self.car.w, self.car.h, self.color);
    }

    pub fn update(&mut self) {
        // simple speed logic
        if self.is_in_intersection() {
            self.speed = (self.speed + 0.5).min(3.0);
        } else if self.is_approaching_intersection() {
            self.speed = 3.0;
        } else {
            self.speed = 1.0;
        }

        match self.random_route.as_str() {
            "GoStraight" => {
                if self.direction == "up" {
                    self.car.y -= self.speed;
                } else if self.direction == "down" {
                    self.car.y += self.speed;
                } else if self.direction == "left" {
                    self.car.x -= self.speed;
                } else if self.direction == "right" {
                    self.car.x += self.speed;
                }
            }
            "TurnRight" => {
                if self.direction == "up" {
                    if self.car.y <= 415.0 {
                        self.car.x += self.speed;
                    } else {
                        self.car.y -= self.speed;
                    }
                } else if self.direction == "down" {
                    if self.car.y >= 340.0 {
                        self.car.x -= self.speed;
                    } else {
                        self.car.y += self.speed;
                    }
                } else if self.direction == "left" {
                    if self.car.x >= 515.0 {
                        self.car.x -= self.speed;
                    } else {
                        self.car.y -= self.speed;
                    }
                } else if self.direction == "right" {
                    if self.car.x <= 435.0 {
                        self.car.x += self.speed;
                    } else {
                        self.car.y += self.speed;
                    }
                }
            }
            "TurnLeft" => {
                if self.direction == "up" {
                    if self.car.y <= 340.0 {
                        self.car.x -= self.speed;
                    } else {
                        self.car.y -= self.speed;
                    }
                } else if self.direction == "down" {
                    if self.car.y >= 410.0 {
                        self.car.x += self.speed;
                    } else {
                        self.car.y += self.speed;
                    }
                } else if self.direction == "left" {
                    if self.car.x >= 440.0 {
                        self.car.x -= self.speed;
                    } else {
                        self.car.y += self.speed;
                    }
                } else if self.direction == "right" {
                    if self.car.x <= 510.0 {
                        self.car.x += self.speed;
                    } else {
                        self.car.y -= self.speed;
                    }
                }
            }
            _ => {}
        }
    }

    pub fn is_off_screen(&self) -> bool {
        self.car.x < -75.0 || self.car.x > 1000.0 || self.car.y < -75.0 || self.car.y > 800.0
    }

    pub fn is_approaching_intersection(&self) -> bool {
        match self.direction.as_str() {
            "up" => (self.car.y <= 475.0) && (self.car.y >= 425.0),
            "down" => (self.car.y >= 275.0) && (self.car.y <= 325.0),
            "right" => (self.car.x <= 425.0) && (self.car.x >= 375.0),
            "left" => (self.car.x >= 525.0) && (self.car.x <= 575.0),
            _ => false,
        }
    }

    pub fn is_in_intersection(&self) -> bool {
        // intersection rect [425,325] - [575,475]
        let car_left = self.car.x;
        let car_right = self.car.x + self.car.w;
        let car_top = self.car.y;
        let car_bottom = self.car.y + self.car.h;

        let inter_left = 425.0;
        let inter_right = 575.0;
        let inter_top = 325.0;
        let inter_bottom = 475.0;

        car_left < inter_right
            && car_right > inter_left
            && car_top < inter_bottom
            && car_bottom > inter_top
    }
}

pub fn create_car(x: f32, y: f32, direction: &str) -> Vehicle {
    let routes = ["TurnLeft", "TurnRight", "GoStraight"];
    let mut rng = ::rand::thread_rng();
    let index = rng.gen_range(0..routes.len());
    let random_route = routes[index].to_string();

    let color = match random_route.as_str() {
        "TurnLeft" => YELLOW,
        "TurnRight" => BLUE,
        "GoStraight" => GRAY,
        _ => WHITE,
    };

    let car_rect = Rect::new(x, y, 50.0, 50.0);
    Vehicle::new(car_rect, direction.to_owned(), random_route, color)
}

pub fn can_create_car(vehicles: &Vec<Vehicle>, spawn_x: f32, spawn_y: f32, direction: &str) -> bool {
    let safe_distance = 100.0;
    for vehicle in vehicles {
        let distance = match direction {
            "up" => {
                if vehicle.direction == "up" && (vehicle.car.x - spawn_x).abs() < 30.0 {
                    (spawn_y - vehicle.car.y).abs()
                } else {
                    safe_distance + 1.0
                }
            }
            "down" => {
                if vehicle.direction == "down" && (vehicle.car.x - spawn_x).abs() < 30.0 {
                    (vehicle.car.y - spawn_y).abs()
                } else {
                    safe_distance + 1.0
                }
            }
            "left" => {
                if vehicle.direction == "left" && (vehicle.car.y - spawn_y).abs() < 30.0 {
                    (spawn_x - vehicle.car.x).abs()
                } else {
                    safe_distance + 1.0
                }
            }
            "right" => {
                if vehicle.direction == "right" && (vehicle.car.y - spawn_y).abs() < 30.0 {
                    (vehicle.car.x - spawn_x).abs()
                } else {
                    safe_distance + 1.0
                }
            }
            _ => safe_distance + 1.0,
        };

        if distance < safe_distance {
            return false;
        }
    }
    true
}
