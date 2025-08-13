use rand::Rng;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::time::Duration;
mod ligths;
use ligths::*;
mod Roads;
mod vehicle;
use vehicle::*;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Road Intersection", 1000, 800)
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut traffic_system = TrafficSystem::new();
    let mut vehicles: Vec<Vehicle> = Vec::new();

   // Traffic light positions
    let light_ne = Rect::new(375, 275, 50, 50); // Down
    let light_nw = Rect::new(575, 275, 50, 50); // Rigth
    let light_se = Rect::new(375, 475, 50, 50); // left
    let light_sw = Rect::new(575, 475, 50, 50); // Up

    let mut Roads = Roads::Roads::new();

    'running: loop {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => {
                    let spawn_x = 515;
                    let spawn_y = 700;
                    if can_create_car(&vehicles, spawn_x, spawn_y, "up") {
                        let new_car = create_car(spawn_x, spawn_y, "up");
                        vehicles.push(new_car.clone());
                        Roads.push(new_car.clone());
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    let spawn_x = 440;
                    let spawn_y = 0;
                    if can_create_car(&vehicles, spawn_x, spawn_y, "down") {
                        let new_car = create_car(spawn_x, spawn_y, "down");
                        vehicles.push(new_car.clone());
                        Roads.push(new_car.clone());
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => {
                    let spawn_x = 950;
                    let spawn_y = 335;
                    if can_create_car(&vehicles, spawn_x, spawn_y, "left") {
                        let new_car = create_car(spawn_x, spawn_y, "left");
                        vehicles.push(new_car.clone());
                        Roads.push(new_car.clone());
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    let spawn_x = 10;
                    let spawn_y = 415;
                    if can_create_car(&vehicles, spawn_x, spawn_y, "right") {
                        let new_car = create_car(spawn_x, spawn_y, "right");
                        vehicles.push(new_car.clone());
                        Roads.push(new_car.clone());
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::R),
                    ..
                } => {
                    let directions = ["up", "down", "left", "right"];
                    let mut rng = rand::rng();
                    let direction = directions[rng.random_range(0..4)];

                    let (spawn_x, spawn_y) = match direction {
                        // "up" => (515, 750),
                        "down" => (440, 0),
                        "left" => (950, 335),
                        "right" => (10, 415),
                        _ => (515, 750),
                    };

                    if can_create_car(&vehicles, spawn_x, spawn_y, direction) {
                        let new_car = create_car(spawn_x, spawn_y, direction);
                        vehicles.push(new_car.clone());
                        Roads.push(new_car.clone());
                    }
                }
                _ => {}
            }
        }
        // Update traffic light system
        traffic_system.update(&vehicles, &Roads);

         let (up_color, down_color, left_color, right_color) = traffic_system.get_light_colors();

        // Draw traffic lights
        canvas.set_draw_color(up_color);
        canvas.fill_rect(light_ne).unwrap();
        canvas.set_draw_color(down_color);
        canvas.fill_rect(light_sw).unwrap();
        canvas.set_draw_color(right_color);
        canvas.fill_rect(light_se).unwrap();
        canvas.set_draw_color(left_color);
        canvas.fill_rect(light_nw).unwrap();
        // Draw roads
        canvas.set_draw_color(Color::WHITE);
        // North-South road (vertical)
        canvas.draw_line((500, 0), (500, 325)).unwrap();
        canvas.draw_line((500, 475), (500, 800)).unwrap();
        canvas.draw_line((575, 0), (575, 325)).unwrap();
        canvas.draw_line((575, 475), (575, 800)).unwrap();
        canvas.draw_line((425, 0), (425, 325)).unwrap();
        canvas.draw_line((425, 475), (425, 800)).unwrap();

        // East-West road (horizontal)
        canvas.draw_line((0, 400), (425, 400)).unwrap();
        canvas.draw_line((575, 400), (1000, 400)).unwrap();
        canvas.draw_line((0, 325), (425, 325)).unwrap();
        canvas.draw_line((575, 325), (1000, 325)).unwrap();
        canvas.draw_line((0, 475), (425, 475)).unwrap();
        canvas.draw_line((575, 475), (1000, 475)).unwrap();

        // Update vehicles with traffic light awareness (using indices to avoid borrowing conflicts)
        for i in 0..vehicles.len() {
            let can_proceed = traffic_system.can_vehicle_proceed(&vehicles[i], &traffic_system);
            let has_vehicle_ahead = {
                let current_vehicle = &vehicles[i];
                vehicles.iter().enumerate().any(|(j, other)| {
                    if i == j {
                        return false; // Skip self
                    }

                    let safe_distance = 60;
                    match current_vehicle.direction.as_str() {
                        "up" => {
                            (current_vehicle.car.x - other.car.x).abs() < 30
                                && other.car.y < current_vehicle.car.y
                                && (current_vehicle.car.y - other.car.y) < safe_distance
                        }
                        "down" => {
                            (current_vehicle.car.x - other.car.x).abs() < 30
                                && other.car.y > current_vehicle.car.y
                                && (other.car.y - current_vehicle.car.y) < safe_distance
                        }
                        "left" => {
                            (current_vehicle.car.y - other.car.y).abs() < 30
                                && other.car.x < current_vehicle.car.x
                                && (current_vehicle.car.x - other.car.x) < safe_distance
                        }
                        "right" => {
                            (current_vehicle.car.y - other.car.y).abs() < 30
                                && other.car.x > current_vehicle.car.x
                                && (other.car.x - current_vehicle.car.x) < safe_distance
                        }
                        _ => false,
                    }
                })
            };

            // Only update if vehicle can proceed and no vehicle ahead
            if can_proceed && !has_vehicle_ahead {
                vehicles[i].update();
            }
        }

        for vec in &vehicles {
            if vec.clone().is_off_screen() {
                Roads.pop(&vec);
            }
        }
        vehicles.retain(|vehicle| !vehicle.is_off_screen());

        for vehicle in &vehicles {
            vehicle.draw(&mut canvas);
        }
        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
