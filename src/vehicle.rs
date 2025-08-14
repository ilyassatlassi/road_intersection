use ::rand::Rng;
use macroquad::prelude::*;

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
            println!("in inter {:?}, {:?}", self.color, self.direction);
            self.speed = 2.0;
        } else if self.is_approaching_intersection() {
            println!("approching {:?}, {:?}", self.color, self.direction);
            self.speed = 1.75;
        } else if self.has_passed_intersection() {
            println!("passed {:?}, {:?}", self.color, self.direction);
            self.speed = 2.0;
        } else {
            println!("else {:?}, {:?}", self.color, self.direction);
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
                        if self.car.x <= 425.0 { // finished turn
                            self.direction = "up".to_string();
                            self.car.x -= self.speed;
                             // update direction
                        }
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
                        if self.car.x <= 475.0 { // finished turn
                            self.direction = "left".to_string(); // update direction
                        }
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

        // If this vehicle had a turning route and has now passed the intersection,
        // we consider the turn finished: update its lane (`direction`) and mark the route done.
        if self.random_route != "GoStraight" && self.has_passed_intersection() {
            println!("sssssssssssssssssssssssssssssssssssssssssssss");
            let new_dir = match (self.direction.as_str(), self.random_route.as_str()) {
                ("up", "TurnLeft") => "left",
                ("up", "TurnRight") => "right",

                ("down", "TurnLeft") => "right",
                ("down", "TurnRight") => "left",

                ("left", "TurnLeft") => "down",
                ("left", "TurnRight") => "up",

                ("right", "TurnLeft") => "up",
                ("right", "TurnRight") => "down",

                // fallback keep same
                (d, _) => d,
            };

            // commit the lane update and mark the route finished
            self.direction = new_dir.to_string();
            self.random_route = "GoStraight".to_string();
        }
    }

    pub fn is_off_screen(&self) -> bool {
        self.car.x < -75.0 || self.car.x > 1000.0 || self.car.y < -75.0 || self.car.y > 800.0
    }

    pub fn is_approaching_intersection(&self) -> bool {
        let inter_left = 425.0;
        let inter_right = 575.0;
        let inter_top = 325.0;
        let inter_bottom = 475.0;
        let buffer = 50.0; // distance before entering

        match self.direction.as_str() {
            // Coming from bottom → just below intersection
            "up" => (self.car.y > inter_bottom) && (self.car.y <= inter_bottom + 15.0),

            // Coming from top → just above intersection
            "down" => (self.car.y < inter_top) && (self.car.y >= inter_top - buffer),

            // Coming from left → just before intersection
            "right" => (self.car.x < inter_left) && (self.car.x >= inter_left - buffer),

            // Coming from right → just before intersection
            "left" => (self.car.x > inter_right) && (self.car.x <= inter_right + 15.0),

            _ => false,
        }
    }

    pub fn has_passed_intersection(&self) -> bool {
        let car_left = self.car.x;
        let car_right = self.car.x + self.car.w;
        let car_top = self.car.y;
        let car_bottom = self.car.y + self.car.h;

        let inter_left = 425.0;
        let inter_right = 575.0;
        let inter_top = 325.0;
        let inter_bottom = 475.0;

        match self.direction.as_str() {
            "up" => {
                car_bottom < inter_top     // exited north
          || car_left < inter_left     // exited west (turn)
          || car_right > inter_right
            } // exited east (turn)

            "down" => {
                car_top > inter_bottom   // exited south
            || car_left < inter_left   // exited west (turn)
            || car_right > inter_right
            } // exited east (turn)

            "left" => {
                car_right < inter_left   // exited west
             || car_top > inter_bottom // exited south (turn)
             || car_bottom < inter_top
            } // exited north (turn)

            "right" => {
                car_left > inter_right  // exited east
              || car_top > inter_bottom// exited south (turn)
              || car_bottom < inter_top
            } // exited north (turn)

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
    let routes = ["TurnRight", "TurnLeft", "GoStraight"];
    let mut rng = ::rand::thread_rng();
    let index = rng.gen_range(0..routes.len());
    let random_route = routes[index].to_string();

    let color = match random_route.as_str() {
        "TurnRight" => BLUE,
        "TurnLeft" => YELLOW,
        "GoStraight" => GRAY,
        _ => WHITE,
    };

    let car_rect = Rect::new(x, y, 40.0, 40.0);
    Vehicle::new(car_rect, direction.to_owned(), random_route, color)
}

pub fn can_create_car(
    vehicles: &Vec<Vehicle>,
    spawn_x: f32,
    spawn_y: f32,
    direction: &str,
) -> bool {
    let safe_distance = 110.0;
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
