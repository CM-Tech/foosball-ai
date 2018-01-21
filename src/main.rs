extern crate find_folder;
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

    let w = SCREEN.0 as f64;
    let h = SCREEN.1 as f64;

    let mut world: World = World {
        ball: Ball {
            radius: 10.0,
            position: (w / 2.0, h / 2.0),
            velocity: (1.0, 1.0),
        },
        p1: Player {
            score: 0,
            dir: 0.0,
            position: 0.0,
        },
        p2: Player {
            score: 0,
            dir: 0.0,
            position: 0.0,
        },
    };
    let mut window: PistonWindow = WindowSettings::new("Foosball", SCREEN)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets")
        .unwrap();
    println!("{:?}", assets);
    let ref font = assets.join("FiraSans-Regular.ttf");
    let factory = window.factory.clone();
    let mut glyphs = Glyphs::new(font, factory, TextureSettings::new()).unwrap();

    while let Some(e) = window.next() {
        let speed = 0.05;
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

            world.ball.position.0 += world.ball.velocity.0 * 5.0;
            world.ball.position.1 += world.ball.velocity.1 * 5.0;

            if world.ball.position.1 - world.ball.radius > h / 3.0
                && world.ball.position.1 + world.ball.radius < h * 2.0 / 3.0
            {
                if world.ball.position.0 + world.ball.radius > w {
                    world.p1.score += 1;
                }
                if world.ball.position.0 - world.ball.radius < 0.0 {
                    world.p2.score += 1;
                }
            }

            if world.ball.position.1 - world.ball.radius < 0.0 {
                world.ball.velocity.1 = -world.ball.velocity.1;
                world.ball.position.1 = world.ball.radius;
            }
            if world.ball.position.1 + world.ball.radius > h {
                world.ball.velocity.1 = -world.ball.velocity.1;
                world.ball.position.1 = h - world.ball.radius;
            }
            if world.ball.position.0 - world.ball.radius < 0.0 {
                world.ball.velocity.0 = -world.ball.velocity.0;
                world.ball.position.0 = world.ball.radius;
            }
            if world.ball.position.0 + world.ball.radius > w {
                world.ball.velocity.0 = -world.ball.velocity.0;
                world.ball.position.0 = w - world.ball.radius;
            }

            for (column, amount) in [2, 3, 4, 5, 5, 4, 3, 2].iter().enumerate() {
                for y in 1..*amount {
                    let x_pos = (column as f64 + 1.0) / 9.0 * w;
                    let y_pos = h * (y as f64) / (*amount as f64)
                        + [&world.p1, &world.p2][PLAYERS[column]].position
                            / [3, 3, 4, 5, 5, 4, 3, 3][column] as f64 * h;

                    if world.ball.position.0 - world.ball.radius < x_pos + 5.0
                        && world.ball.position.0 + world.ball.radius > x_pos - 5.0
                        && world.ball.position.1 - world.ball.radius < y_pos + 25.0
                        && world.ball.position.1 + world.ball.radius > y_pos - 25.0
                    {
                        let ball_x = (world.ball.position.0 - x_pos).abs()
                            * (PLAYERS[column] as f64 * -2.0 + 1.0);
                        let ball_y = world.ball.position.1 - y_pos;
                        let length = ball_x.hypot(ball_y);
                        world.ball.velocity.0 = ball_x / length;
                        world.ball.velocity.1 = ball_y / length;
                    }

                    rectangle(
                        my_palette[PLAYERS[column]],
                        [x_pos - 5.0, y_pos - 25.0, 10.0, 50.0],
                        c.transform,
                        g,
                    );
                }
            }

            rectangle(
                [0.1, 0.1, 0.1, 1.0],
                [0.0, h / 3.0, 10.0, h / 3.0],
                c.transform,
                g,
            );

            rectangle(
                [0.1, 0.1, 0.1, 1.0],
                [w - 10.0, h / 3.0, 10.0, h / 3.0],
                c.transform,
                g,
            );

            ellipse(
                [0.1, 0.1, 0.1, 1.0],
                [
                    world.ball.position.0 - world.ball.radius,
                    world.ball.position.1 - world.ball.radius,
                    world.ball.radius * 2.0,
                    world.ball.radius * 2.0,
                ],
                c.transform,
                g,
            );

            text(
                [0.7, 0.7, 0.7, 1.0],
                50,
                &(world.p1.score).to_string(),
                &mut glyphs,
                c.transform.trans(w / 2.0 - 50.0, 50.0),
                g,
            ).unwrap();

            text(
                [0.7, 0.7, 0.7, 1.0],
                50,
                &(world.p2.score).to_string(),
                &mut glyphs,
                c.transform.trans(w / 2.0 + 50.0, 50.0),
                g,
            ).unwrap();
        });
    }
}
