extern crate piston;
extern crate piston_window;
use piston_window::*;
use std::f64;

struct Ball {
    radius: f64,
    position: (f64, f64),
    velocity: (f64, f64),
}

struct Player {
    score: u32,
    dir: f32,
    position: f64,
}

struct World {
    ball: Ball,
    p1: Player,
    p2: Player,
}

const SCREEN: (u32, u32) = (1050, 600);
const PALLETTES: [[[f32; 4]; 2]; 2] = [
    [[0.15, 0.72, 0.95, 1.0], [0.95, 0.6, 0.13, 1.0]],
    [[0.52, 0.84, 0.0, 1.0], [0.87, 0.29, 0.12, 1.0]],
];
const PLAYERS: [usize; 8] = [0, 0, 1, 0, 1, 0, 1, 1];
fn main() {
    let my_palette = PALLETTES[0];

    let mut world: World = World {
        ball: Ball {
            radius: 0.05,
            position: (0.5, 0.5),
            velocity: (0.0, 0.0),
        },
        p1: Player {
            score: 0,
            dir: 0.0,
            position: 0.5,
        },
        p2: Player {
            score: 0,
            dir: 0.0,
            position: 0.5,
        },
    };
    let mut window: PistonWindow = WindowSettings::new("Foosball", SCREEN)
        .exit_on_esc(true)
        .build()
        .unwrap();
    while let Some(e) = window.next() {
        let speed = 0.1;
        if let Some(Button::Keyboard(key)) = e.press_args() {
            match key {
                Key::W => world.p1.dir = -speed,
                Key::S => world.p1.dir = speed,
                Key::Up => world.p2.dir = -speed,
                Key::Down => world.p2.dir = speed,
                _ => (),
            }
        }
        if let Some(Button::Keyboard(key)) = e.release_args() {
            match key {
                Key::W if world.p1.dir == -speed => world.p1.dir = 0.0,
                Key::S if world.p1.dir == speed => world.p1.dir = 0.0,
                Key::Up if world.p2.dir == -speed => world.p2.dir = 0.0,
                Key::Down if world.p2.dir == speed => world.p2.dir = 0.0,
                _ => (),
            }
        }
        window.draw_2d(&e, |c, g| {
            clear([0.9, 0.9, 0.9, 1.0], g);

            world.p1.position += world.p1.dir as f64;
            world.p2.position += world.p2.dir as f64;

            world.p1.position = world.p1.position.min(1.0).max(-1.0);
            world.p2.position = world.p2.position.min(1.0).max(-1.0);

            for (column, amount) in [1, 2, 3, 4, 4, 3, 2, 1].iter().enumerate() {
                for y in 0..*amount {
                    let w = SCREEN.0 as f64;
                    let h = SCREEN.1 as f64;
                    rectangle(
                        my_palette[PLAYERS[column as usize]],
                        [
                            column as f64 / 7.0 * (w * 6.0 / 10.0) + w * 2.0 / 10.0 - 5.0,
                            h * (y as f64 + 1.0) / (*amount as f64 + 1.0) - 25.0
                                + [&world.p1, &world.p2][PLAYERS[column as usize]].position
                                    / [3, 3, 4, 5, 5, 4, 3, 3][column] as f64
                                    * h,
                            10.0,
                            50.0,
                        ],
                        c.transform,
                        g,
                    );
                }
            }
        });
    }
}
