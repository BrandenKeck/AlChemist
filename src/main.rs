// Custom Modules
use rusty_pixels::game::{Game};
use rusty_pixels::camera::Camera;
use rusty_pixels::world::{Material, Pixel, Seed, World};
use rusty_pixels::config::{get_config, Config};

// Required Libraries
use minifb::{Key, Scale, Window, WindowOptions};
use std::{
    time::{Duration, Instant},
};

fn main() {

    // Load in the configuration
    let mut config: Config = get_config();
    println!("{:?}", config); // HERE

    let sand = Material { density: 5.0, r: 194, g: 178, b: 128 };
    let water = Material { density: 2.0, r: 50, g: 100, b: 200 };
    let seeds = [
        Seed { x: 10, y: 10, pixel: Pixel { material: sand } },
        Seed { x: 20, y: 20, pixel: Pixel { material: water } },
    ];

    let mut world = World::grow_from_seeds(1000, 600, &seeds, Camera::new(320, 180, 40));
    let mut buffer = vec![0u32; 320 * 180];


    

    // Create the minifb window object
    let mut window = Window::new(
        "Rusty Pixels",
        config.window.width,
        config.window.height,
        WindowOptions { scale: Scale::X4, ..WindowOptions::default() },
    )
    .expect("failed to create window");
    window.set_target_fps(60);

    let mut game = Game::new(config.window.width, config.window.height);
    let mut last = Instant::now();
    let mut lag = Duration::ZERO;
    const TICK: Duration = Duration::from_micros(16_667);

    // The game loop: input -> update (fixed steps) -> draw -> present.
    while window.is_open() && !window.is_key_down(Key::Escape) {
        let now = Instant::now();
        lag += now - last;
        last = now;

        while lag >= TICK {
            game.update();
            lag -= TICK;
        }

        // To do
        // world.camera.follow(player_x, player_y, world.width, world.height);

        world.render(&mut buffer);
        window.update_with_buffer(&buffer, 320, 180).expect("failed to present frame");

        // game.draw();
        // window
        //     .update_with_buffer(&game.pixels, config.window.width, config.window.height)
        //     .expect("failed to present frame");
    }
}