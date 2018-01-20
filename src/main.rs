extern crate piston;
extern crate graphics;
extern crate glutin_window;
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
pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    rotation: f64,   // Rotation for the square.
    world:World,
    
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        let pallettes = [
        [[0.15, 0.72, 0.95, 1.0], [0.95, 0.6, 0.13, 1.0]],
        [[0.52, 0.84, 0.0, 1.0], [0.87, 0.29, 0.12, 1.0]],
    ];
    let my_palette = pallettes[0];
    let colors: Vec<_> = [0, 0, 1, 0, 1, 0, 1, 1]
        .iter()
        .map(|x| my_palette[*x as usize])
        .collect();
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED:   [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let square = rectangle::square(0.0, 0.0, 50.0);
        let rotation = self.rotation;
        let world=&self.world;
        let (x, y) = ((args.width / 2) as f64,
                      (args.height / 2) as f64);

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear([0.9, 0.9, 0.9, 1.0], gl);

            for (column, amount) in [1, 2, 3, 4, 4, 3, 2, 1].iter().enumerate() {
                for y in 0..*amount {
                    let w = world.size.0 as f64;
                    let h = world.size.1 as f64;
                    rectangle(
                        my_palette[PLAYERS[column]],
                        [x_pos - 5.0, y_pos - 25.0, 10.0, 50.0],
                        c.transform,
                        gl,
                    );
                }
            }

            let transform = c.transform.trans(x, y)
                                       .rot_rad(rotation)
                                       .trans(-25.0, -25.0);

            // Draw a box rotating around the middle of the screen.
            //rectangle(RED, square, transform, gl);
        });
    }
//update game here (args.dt is delta time)
    fn update(&mut self, args: &UpdateArgs) {
        // Rotate 2 radians per second.
        self.rotation += 2.0 * args.dt;
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
            position: 0.5,
        },
        p2: Player {
            score: 0,
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
    }
}