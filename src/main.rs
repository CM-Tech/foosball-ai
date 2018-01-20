extern crate piston;
extern crate piston_window;
use piston_window::*;
use piston::input::*;
use std::f64;

#[derive(Debug)]
struct Ball {
    radius: f64,
    position: (f64, f64),
    velocity: (f64, f64),
}

#[derive(Debug)]
struct Player {
    score: u32,
    position: f64,
}

#[derive(Debug)]
struct World {
    ball: Ball,
    p1: Player,
    p2: Player,
}

/*const pallettes = [
    [0x85D600, 0xDE4A1F, 0xc4ff66, 0xe98263],
    [0x26B8F2, 0xF29A21, 0x90dbf9, 0xf7c47d]
];*/
const SCREEN: (u32, u32) = (1050, 600);
const PALLETTES: [[[f32; 4]; 2]; 2] = [
    [[0.15, 0.72, 0.95, 1.0], [0.95, 0.6, 0.13, 1.0]],
    [[0.52, 0.84, 0.0, 1.0], [0.87, 0.29, 0.12, 1.0]],
];

fn main() {
    let my_palette = PALLETTES[0];
    let colors: Vec<_> = [0, 0, 1, 0, 1, 0, 1, 1]
        .iter()
        .map(|x| my_palette[*x as usize])
        .collect();

    let mut world: World = World {
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
    let mut window: PistonWindow = WindowSettings::new("Foosball", SCREEN)
        .exit_on_esc(true)
        .build()
        .unwrap();
    let mut n: f64 = 0.0;
    while let Some(e) = window.next() {
        n = n + 0.01;
        window.draw_2d(&e, |c, g| {
            clear([0.9, 0.9, 0.9, 1.0], g);

            for (column, amount) in [1, 2, 3, 4, 4, 3, 2, 1].iter().enumerate() {
                for y in 0..*amount {
                    let w = SCREEN.0 as f64;
                    let h = SCREEN.1 as f64;
                    rectangle(
                        colors[column as usize],
                        [
                            column as f64 / 7.0 * (w * 6.0 / 10.0) + w * 2.0 / 10.0 - 5.0,
                            h * (y as f64 + 1.0) / (*amount as f64 + 1.0) - 25.0,
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
