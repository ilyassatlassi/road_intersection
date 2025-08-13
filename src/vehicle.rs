use rand::Rng;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

#[derive(Debug, PartialEq, Clone)]
pub struct Vehicle {
    pub car: Rect,
    pub direction: String,
    pub random_route: String,
    pub color: Color,
    pub speed: i32,
}

impl Vehicle {
    pub fn new(car: Rect, direction: String, random_route: String, color: Color) -> Vehicle {
        Vehicle {
            car,
            direction,
            random_route,
            color,
            speed: 1,
        }
    }

    pub fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        canvas.set_draw_color(self.color);
        canvas.fill_rect(self.car).unwrap();
    }

    pub fn update(&mut self) {
        match self.random_route.as_str() {
            "GoStraight" => {
                if self.direction.as_str() == "up" {
                    self.car.y -= self.speed;
                } else if self.direction.as_str() == "down" {
                    self.car.y += self.speed;
                } else if self.direction.as_str() == "left" {
                    self.car.x -= self.speed;
                } else if self.direction.as_str() == "right" {
                    self.car.x += self.speed;
                } else {
                    todo!();
                }
            }
            "TurnRight" => {
                if self.direction.as_str() == "up" {
                    if self.car.y <= 415 {
                        self.car.x += self.speed;
                    } else {
                        self.car.y -= self.speed;
                    }
                } else if self.direction.as_str() == "down" {
                    if self.car.y >= 340 {
                        self.car.x -= self.speed;
                    } else {
                        self.car.y += self.speed;
                    }
                } else if self.direction.as_str() == "left" {
                    if self.car.x >= 515 {
                        self.car.x -= self.speed;
                    } else {
                        self.car.y -= self.speed;
                    }
                } else if self.direction.as_str() == "right" {
                    if self.car.x <= 435 {
                        self.car.x += self.speed;
                    } else {
                        self.car.y += self.speed;
                    }
                } else {
                    todo!();
                }
            }
            "TurnLeft" => {
                if self.direction.as_str() == "up" {
                    if self.car.y <= 340 {
                        self.car.x -= self.speed;
                    } else {
                        self.car.y -= self.speed;
                    }
                } else if self.direction.as_str() == "down" {
                    if self.car.y >= 410 {
                        self.car.x += self.speed;
                    } else {
                        self.car.y += self.speed;
                    }
                } else if self.direction.as_str() == "left" {
                    if self.car.x >= 440 {
                        self.car.x -= self.speed;
                    } else {
                        self.car.y += self.speed;
                    }
                } else if self.direction.as_str() == "right" {
                    if self.car.x <= 510 {
                        self.car.x += self.speed;
                    } else {
                        self.car.y -= self.speed;
                    }
                } else {
                    todo!();
                }
            }
            _ => {}
        }
    }

    pub fn is_off_screen(&self) -> bool {
        self.car.x < -75 || self.car.x > 1075 || self.car.y < -75 || self.car.y > 875
    }
}

pub fn create_car(x: i32, y: i32, direction: &str) -> Vehicle {
    let routes = ["TurnLeft", "TurnRight", "GoStraight"];
    let mut rng = rand::rng();
    let index = rng.random_range(0..3);
    let random_route = routes[index].to_owned();

    let color = match random_route.as_str() {
        "TurnLeft" => Color::YELLOW,
        "TurnRight" => Color::BLUE,
        "GoStraight" => Color::GREY,
        _ => Color::WHITE,
    };
    let car_rect = Rect::new(x, y, 50, 50);
    Vehicle::new(car_rect, direction.to_owned(), random_route, color)
}

pub fn can_create_car(
    vehicles: &Vec<Vehicle>,
    spawn_x: i32,
    spawn_y: i32,
    direction: &str,
) -> bool {
    let safe_distance = 100;

    for vehicle in vehicles {
        let distance = match direction {
            "up" => {
                if vehicle.direction == "up" && (vehicle.car.x - spawn_x).abs() < 30 {
                    (spawn_y - vehicle.car.y).abs()
                } else {
                    safe_distance + 1 // Not same lane
                }
            }
            "down" => {
                if vehicle.direction == "down" && (vehicle.car.x - spawn_x).abs() < 30 {
                    (vehicle.car.y - spawn_y).abs()
                } else {
                    safe_distance + 1
                }
            }
            "left" => {
                if vehicle.direction == "left" && (vehicle.car.y - spawn_y).abs() < 30 {
                    (spawn_x - vehicle.car.x).abs()
                } else {
                    safe_distance + 1
                }
            }
            "right" => {
                if vehicle.direction == "right" && (vehicle.car.y - spawn_y).abs() < 30 {
                    (vehicle.car.x - spawn_x).abs()
                } else {
                    safe_distance + 1
                }
            }
            _ => safe_distance + 1,
        };

        if distance < safe_distance {
            return false;
        }
    }
    true
}
