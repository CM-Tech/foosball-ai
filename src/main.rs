extern crate piston_window;
use piston_window::*;

#[derive(Debug)]
struct Ball {
    radius: f32,
    position: (f32, f32),
    velocity: (f32, f32),
}

#[derive(Debug)]
struct Player {
    score: u32,
    position: f32,
}

#[derive(Debug)]
struct World {
    size: (u32, u32),
    ball: Ball,
    p1: Player,
    p2: Player,
}
/*const pallettes = [
    [0x85D600, 0xDE4A1F, 0xc4ff66, 0xe98263],
    [0x26B8F2, 0xF29A21, 0x90dbf9, 0xf7c47d]
];*/

fn main() {
    let mut world: World = World {
        size: (640, 480),
        ball: Ball {
            radius: 0.05,
            position: (0.5, 0.5),
            velocity: (0.0, 0.0),
        },
        p1: Player {
            score: 0,
            position: 0.5,
        },
        p2: Player {
            score: 0,
            position: 0.5,
        },
    };
    let mut window: PistonWindow = WindowSettings::new("Foosball", world.size)
        .exit_on_esc(true)
        .build()
        .unwrap_or_else(|e| panic!("Failed to build PistonWindow: {}", e));
    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g| {
            clear([0.9, 0.9, 0.9, 1.0], g);
            rectangle([0.15, 0.72, 0.95, 1.0],  // blue
                      [0.0, 0.0, 100.0, 100.0], // rectangle
                      c.transform, g);
            rectangle([0.95, 0.6, 0.13, 1.0],     // orange
                      [0.0, 100.0, 100.0, 100.0], // rectangle
                      c.transform, g);

            rectangle([0.52, 0.84, 0.0, 1.0],     // green
                      [100.0, 0.0, 100.0, 100.0], // rectangle
                      c.transform, g);
            rectangle([0.87, 0.29, 0.12, 1.0],      // red
                      [100.0, 100.0, 100.0, 100.0], // rectangle
                      c.transform, g);
        });
    }
}
