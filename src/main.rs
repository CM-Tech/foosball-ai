extern crate piston;
extern crate graphics;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
extern crate piston_window;

use piston_window::*;
use opengl_graphics::{ GlGraphics, OpenGL };
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
    gl: GlGraphics, // OpenGL drawing backend.
    rotation: f64,   // Rotation for the square.
    world:World,
    
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
    let my_palette = PALLETTES[0];
    let colors: Vec<_> = [0, 0, 1, 0, 1, 0, 1, 1]
        .iter()
        .map(|x| my_palette[*x as usize])
        .collect();
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED:   [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let square = rectangle::square(0.0, 0.0, 50.0);
        let rotation = self.rotation;
        let mut world=&mut self.world;
        let mut shrunkScale=(args.width as f64)/(world.size.0 as f64);
        if((args.height as f64)/(world.size.1 as f64)<shrunkScale){
            shrunkScale=(args.height as f64)/(world.size.1 as f64);
        }
        let (cx, cy) = ((args.width / 2) as f64,
                      (args.height / 2) as f64);
       
        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear([0.9, 0.9, 0.9, 1.0], gl);
             let w = shrunkScale*(world.size.0 as f64);
                    let h = shrunkScale*(world.size.1 as f64);
                    let transform = c.transform.trans(cx, cy)
                                       .trans(-w/2.0, -h/2.0);

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

            

            // Draw a box rotating around the middle of the screen.
            //rectangle(RED, square, transform, gl);
        });
    }
//update game here (args.dt is delta time)
    fn update(&mut self, args: &UpdateArgs) {
        // Rotate 2 radians per second.
        let mut world=&mut self.world;
        let step:f64=args.dt*60.0;
        let w = (world.size.0 as f64);
                    let h = (world.size.1 as f64);
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
    fn key(&mut self, key: Key) {
        if key == Key::W {

        }
        // Rotate 2 radians per second.
    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;
let mut world: World = World {
        size: (1050, 600),
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
    // Create an Glutin window.
    let mut window: PistonWindow = WindowSettings::new("Foosball",
            [world.size.0, world.size.1]
        )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        rotation: 0.0,
        world:world
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }
        if let Some(Button::Keyboard(key)) = e.press_args() {
            app.key(key);
            println!("Pressed keyboard key '{:?}'", key);
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