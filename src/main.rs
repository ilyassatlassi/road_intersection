use macroquad::prelude::*;
mod ligths;
mod roads;
mod vehicle;
use ::rand::Rng;
use ligths::{TrafficSystem};
use roads::Roads;
use vehicle::{create_car, can_create_car, Vehicle};

fn window_conf() -> Conf {
    Conf {
        window_title: "Road Intersection".to_owned(),
        window_width: 1000,
        window_height: 800,
        fullscreen: false,
        high_dpi: false,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut traffic_system = TrafficSystem::new();
    let mut vehicles: Vec<Vehicle> = Vec::new();
    let mut roads = Roads::new();

    // Light rectangles coordinates
    // We'll draw simple rectangles as lights
    let light_ne = (375.0, 275.0, 50.0, 50.0); // Down (north-east)
    let light_nw = (575.0, 275.0, 50.0, 50.0); // Right (north-west)
    let light_se = (375.0, 475.0, 50.0, 50.0); // Left (south-east)
    let light_sw = (575.0, 475.0, 50.0, 50.0); // Up (south-west)

    loop {
        clear_background(BLACK);

        // Input: spawn cars via arrow keys, R random
        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        if is_key_pressed(KeyCode::Up) {
            let spawn_x = 515.0;
            let spawn_y = 800.0;
            if can_create_car(&vehicles, spawn_x, spawn_y, "up") {
                let new_car = create_car(spawn_x, spawn_y, "up");
                vehicles.push(new_car);
            }
        }
        if is_key_pressed(KeyCode::Down) {
            let spawn_x = 440.0;
            let spawn_y = -50.0;
            if can_create_car(&vehicles, spawn_x, spawn_y, "down") {
                let new_car = create_car(spawn_x, spawn_y, "down");
                vehicles.push(new_car);
            }
        }
        if is_key_pressed(KeyCode::Left) {
            let spawn_x = 1000.0;
            let spawn_y = 335.0;
            if can_create_car(&vehicles, spawn_x, spawn_y, "left") {
                let new_car = create_car(spawn_x, spawn_y, "left");
                vehicles.push(new_car);
            }
        }
        if is_key_pressed(KeyCode::Right) {
            let spawn_x = -50.0;
            let spawn_y = 415.0;
            if can_create_car(&vehicles, spawn_x, spawn_y, "right") {
                let new_car = create_car(spawn_x, spawn_y, "right");
                vehicles.push(new_car);
            }
        }
        if is_key_pressed(KeyCode::R) {
            let directions = ["up", "down", "left", "right"];
            let mut rng = ::rand::thread_rng();
            let direction = directions[rng.gen_range(0..directions.len())];

            let (spawn_x, spawn_y) = match direction {
                "up" => (515.0, 800.0),
                "down" => (440.0, -50.0),
                "left" => (1000.0, 335.0),
                "right" => (-50.0, 415.0),
                _ => (515.0, 750.0),
            };

            if can_create_car(&vehicles, spawn_x, spawn_y, direction) {
                let new_car = create_car(spawn_x, spawn_y, direction);
                vehicles.push(new_car);
            }
        }

        // Rebuild roads representation from current vehicles
        roads = Roads::new();
        for v in &vehicles {
            roads.push(v.clone());
        }

        // Update traffic system
        traffic_system.update(&vehicles, &roads);
        let (up_color, down_color, left_color, right_color) = traffic_system.get_light_colors();

        // Draw traffic lights
        draw_rectangle(light_ne.0, light_ne.1, light_ne.2, light_ne.3, up_color);
        draw_rectangle(light_sw.0, light_sw.1, light_sw.2, light_sw.3, down_color);
        draw_rectangle(light_se.0, light_se.1, light_se.2, light_se.3, right_color);
        draw_rectangle(light_nw.0, light_nw.1, light_nw.2, light_nw.3, left_color);

        // Draw roads (lines)
        let thickness = 4.0;
        draw_line(500.0, 0.0, 500.0, 325.0, thickness, WHITE);
        draw_line(500.0, 475.0, 500.0, 800.0, thickness, WHITE);
        draw_line(575.0, 0.0, 575.0, 325.0, thickness, WHITE);
        draw_line(575.0, 475.0, 575.0, 800.0, thickness, WHITE);
        draw_line(425.0, 0.0, 425.0, 325.0, thickness, WHITE);
        draw_line(425.0, 475.0, 425.0, 800.0, thickness, WHITE);
        draw_line(0.0, 400.0, 425.0, 400.0, thickness, WHITE);
        draw_line(575.0, 400.0, 1000.0, 400.0, thickness, WHITE);
        draw_line(0.0, 325.0, 425.0, 325.0, thickness, WHITE);
        draw_line(575.0, 325.0, 1000.0, 325.0, thickness, WHITE);
        draw_line(0.0, 475.0, 425.0, 475.0, thickness, WHITE);
        draw_line(575.0, 475.0, 1000.0, 475.0, thickness, WHITE);

        // Update vehicles with traffic awareness (index loop to allow mutable update)
        for i in 0..vehicles.len() {
            // allowed by traffic
            let can_proceed = traffic_system.can_vehicle_proceed(&vehicles[i]);
            // check vehicle ahead
            let has_vehicle_ahead = {
                let current_vehicle = &vehicles[i];
                vehicles.iter().enumerate().any(|(j, other)| {
                    if i == j {
                        return false;
                    }
                    let safe_distance = 60.0;
                    match current_vehicle.direction.as_str() {
                        "up" => {
                            (current_vehicle.car.x - other.car.x).abs() < 30.0
                                && other.car.y < current_vehicle.car.y
                                && (current_vehicle.car.y - other.car.y) < safe_distance
                        }
                        "down" => {
                            (current_vehicle.car.x - other.car.x).abs() < 30.0
                                && other.car.y > current_vehicle.car.y
                                && (other.car.y - current_vehicle.car.y) < safe_distance
                        }
                        "left" => {
                            (current_vehicle.car.y - other.car.y).abs() < 30.0
                                && other.car.x < current_vehicle.car.x
                                && (current_vehicle.car.x - other.car.x) < safe_distance
                        }
                        "right" => {
                            (current_vehicle.car.y - other.car.y).abs() < 30.0
                                && other.car.x > current_vehicle.car.x
                                && (other.car.x - current_vehicle.car.x) < safe_distance
                        }
                        _ => false,
                    }
                })
            };

            if can_proceed && !has_vehicle_ahead {
                vehicles[i].update();
            }
        }

        // Remove off-screen vehicles
        vehicles.retain(|v| !v.is_off_screen());

        // Draw vehicles
        for v in &vehicles {
            v.draw();
        }

        // HUD text (simple one-line)
        let hud = format!(
            "UP {}/{}  DOWN {}/{}  LEFT {}/{}  RIGHT {}/{}",
            roads.count_cars("up"),
            10,
            roads.count_cars("down"),
            10,
            roads.count_cars("left"),
            10,
            roads.count_cars("right"),
            10
        );
        draw_text(&hud, 16.0, 22.0, 22.0, WHITE);

        // show queues
        let queues = format!(
            "Queues: up={} down={} left={} right={}",
            roads.count_waiting("up"),
            roads.count_waiting("down"),
            roads.count_waiting("left"),
            roads.count_waiting("right"),
        );
        draw_text(&queues, 16.0, 44.0, 18.0, YELLOW);

        // Controls
        draw_text(
            "Controls: ↑ spawn south, ↓ spawn north, ← spawn east, → spawn west, R random, Esc quit",
            16.0,
            66.0,
            14.0,
            GRAY,
        );

        next_frame().await
    }
}
