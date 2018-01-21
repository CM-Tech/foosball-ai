extern crate find_folder;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
extern crate piston_window;

use piston_window::*;
use opengl_graphics::OpenGL;
#[derive(Debug)]
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
const PALLETTES: [[[f32; 4]; 2]; 2] = [
    [[0.15, 0.72, 0.95, 1.0], [0.95, 0.6, 0.13, 1.0]],
    [[0.52, 0.84, 0.0, 1.0], [0.87, 0.29, 0.12, 1.0]],
];
const PLAYERS: [usize; 8] = [0, 0, 1, 0, 1, 0, 1, 1];
pub struct App {
    world: World,
    glyphs: Glyphs,
}

impl App {
    fn render<E: piston::input::GenericEvent>(
        &mut self,
        args: &RenderArgs,
        e: &E,
        window: &mut PistonWindow,
    ) {
        let my_palette = PALLETTES[0];

        use graphics::*;

        let world = &mut self.world;
        let mut shrunk_scale = (args.width as f64) / (world.size.0 as f64);
        if (args.height as f64) / (world.size.1 as f64) < shrunk_scale {
            shrunk_scale = (args.height as f64) / (world.size.1 as f64);
        }
        let (cx, cy) = ((args.width / 2) as f64, (args.height / 2) as f64);
        let glyphs = &mut self.glyphs;
        window.draw_2d(e, |c, gl| {
            // Clear the screen.
            clear([0.9, 0.9, 0.9, 1.0], gl);
            let w = shrunk_scale * (world.size.0 as f64);
            let h = shrunk_scale * (world.size.1 as f64);
            let transform = c.transform.trans(cx, cy).trans(-w / 2.0, -h / 2.0);

            /*for (column, amount) in [1, 2, 3, 4, 4, 3, 2, 1].iter().enumerate() {
                for y in 0..*amount {
                    
                    ellipse(
                        colors[column as usize],
                        [
                            column as f64 / 7.0 * (w * 6.0 / 10.0) + w * 2.0 / 10.0 - 10.0,
                            h * (y as f64 + 1.0) / (*amount as f64 + 1.0) -30.0*shrunkScale,
                            20.0*shrunkScale,
                            20.0*shrunkScale,
                        ],
                        transform,
                        gl,
                    );
                    rectangle(
                        colors[column as usize],
                        [
                            column as f64 / 7.0 * (w * 6.0 / 10.0) + w * 2.0 / 10.0 - 10.0,
                            h * (y as f64 + 1.0) / (*amount as f64 + 1.0) -20.0*shrunkScale,
                            20.0*shrunkScale,
                            40.0*shrunkScale,
                        ],
                        transform,
                        gl,
                    );
                    
                    ellipse(
                        colors[column as usize],
                        [
                            column as f64 / 7.0 * (w * 6.0 / 10.0) + w * 2.0 / 10.0 - 10.0,
                            h * (y as f64 + 1.0) / (*amount as f64 + 1.0) +10.0*shrunkScale,
                            20.0*shrunkScale,
                            20.0*shrunkScale,
                        ],
                        transform,
                        gl,
                    );
                }
            }*/

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
                        transform,
                        gl,
                    );
                }
            }
            rectangle(
                [0.1, 0.1, 0.1, 1.0],
                [0.0, h / 3.0, 10.0, h / 3.0],
                transform,
                gl,
            );

            rectangle(
                [0.1, 0.1, 0.1, 1.0],
                [w - 10.0, h / 3.0, 10.0, h / 3.0],
                transform,
                gl,
            );

            ellipse(
                [0.1, 0.1, 0.1, 1.0],
                [
                    world.ball.position.0 - world.ball.radius,
                    world.ball.position.1 - world.ball.radius,
                    world.ball.radius * 2.0,
                    world.ball.radius * 2.0,
                ],
                transform,
                gl,
            );

            text(
                [0.7, 0.7, 0.7, 1.0],
                50,
                &(world.p1.score).to_string(),
                glyphs,
                transform.trans(w / 2.0 - 50.0, 50.0),
                gl,
            ).unwrap();

            text(
                [0.7, 0.7, 0.7, 1.0],
                50,
                &(world.p2.score).to_string(),
                glyphs,
                transform.trans(w / 2.0 + 50.0, 50.0),
                gl,
            ).unwrap();
        });
    }
    //update game here (args.dt is delta time)
    fn update(&mut self, args: &UpdateArgs) {
        // Rotate 2 radians per second.
        let world = &mut self.world;
        let step: f64 = args.dt * 60.0;
        let w = world.size.0 as f64;
        let h = world.size.1 as f64;
        world.p1.position += world.p1.dir as f64 * step;
        world.p2.position += world.p2.dir as f64 * step;

        world.p1.position = world.p1.position.min(1.0).max(-1.0);
        world.p2.position = world.p2.position.min(1.0).max(-1.0);

        world.ball.position.0 += world.ball.velocity.0 * 5.0 * step;
        world.ball.position.1 += world.ball.velocity.1 * 5.0 * step;

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
    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.

    let opengl = OpenGL::V3_2;
    let world: World = World {
        size: (1050, 600),
        ball: Ball {
            radius: 0.05,
            position: (0.5, 0.5),
            velocity: (0.0, 0.0),
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
    // Create an Glutin window.
    let mut window: PistonWindow = WindowSettings::new("Foosball", [world.size.0, world.size.1])
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();
    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets")
        .unwrap();
    println!("{:?}", assets);
    let ref font = assets.join("FiraSans-Regular.ttf");
    let factory = window.factory.clone();
    let glyphs = Glyphs::new(font, factory, TextureSettings::new()).unwrap();
    // Create a new game and run it.
    let mut app = App {
        world: world,
        glyphs: glyphs,
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r, &e, &mut window);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }
        let speed = 0.05;
        if let Some(Button::Keyboard(key)) = e.press_args() {
            match key {
                Key::W => app.world.p1.dir = -speed,
                Key::S => app.world.p1.dir = speed,
                Key::Up => app.world.p2.dir = -speed,
                Key::Down => app.world.p2.dir = speed,
                _ => (),
            }
        }
        if let Some(Button::Keyboard(key)) = e.release_args() {
            match key {
                Key::W if app.world.p1.dir == -speed => app.world.p1.dir = 0.0,
                Key::S if app.world.p1.dir == speed => app.world.p1.dir = 0.0,
                Key::Up if app.world.p2.dir == -speed => app.world.p2.dir = 0.0,
                Key::Down if app.world.p2.dir == speed => app.world.p2.dir = 0.0,
                _ => (),
            }
        }
    }
}
