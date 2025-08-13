use rand::Rng;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Road Intersection", 1000, 800)
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

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
                    // if can_spawn_vehicle(&vehicles, spawn_x, spawn_y, "up") {
                    //     let new_car = spawn_car(spawn_x, spawn_y, "up");
                    //     vehicles.push(new_car.clone());
                    //     Roads.push(new_car.clone());
                    // }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    let spawn_x = 440;
                    let spawn_y = 0;
                    // if can_spawn_vehicle(&vehicles, spawn_x, spawn_y, "down") {
                    //     let new_car = spawn_car(spawn_x, spawn_y, "down");
                    //     vehicles.push(new_car.clone());
                    //     Roads.push(new_car.clone());
                    // }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => {
                    let spawn_x = 950;
                    let spawn_y = 335;
                    // if can_spawn_vehicle(&vehicles, spawn_x, spawn_y, "left") {
                    //     let new_car = spawn_car(spawn_x, spawn_y, "left");
                    //     vehicles.push(new_car.clone());
                    //     Roads.push(new_car.clone());
                    // }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    let spawn_x = 10;
                    let spawn_y = 415;
                    // if can_spawn_vehicle(&vehicles, spawn_x, spawn_y, "right") {
                    //     let new_car = spawn_car(spawn_x, spawn_y, "right");
                    //     vehicles.push(new_car.clone());
                    //     Roads.push(new_car.clone());
                    // }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::R),
                    ..
                } => {
                    let directions = ["up", "down", "left", "right"];
                    let mut rng = rand::rng();
                    let direction = directions[rng.random_range(0..4)];

                    let (spawn_x, spawn_y) = match direction {
                        "up" => (515, 750),
                        "down" => (440, 0),
                        "left" => (950, 335),
                        "right" => (10, 415),
                        _ => (515, 750),
                    };

                    // if can_spawn_vehicle(&vehicles, spawn_x, spawn_y, direction) {
                    //     let new_car = spawn_car(spawn_x, spawn_y, direction);
                    //     vehicles.push(new_car.clone());
                    //     Roads.push(new_car.clone());
                    // }
                }
                _ => {}
            }
        }

        let (up_color, down_color, left_color, right_color) = traffic_system.get_light_colors();
    }
}
