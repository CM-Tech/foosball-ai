extern crate graphics;
extern crate piston_window;

use piston_window::*;

struct Ball {
    radius: f64,
    pos: (f64, f64),
    vel: (f64, f64),
}

struct Player {
    score: u32,
    dir: f32,
    pos: f64,
}

pub struct App {
    glyphs: Glyphs,
    palette: usize,
    size: (u32, u32),
    ball: Ball,
    p1: Player,
    p2: Player,
}

const PALETTES: [[[f32; 4]; 2]; 2] = [
    [[0.15, 0.72, 0.95, 1.0], [0.95, 0.6, 0.13, 1.0]],
    [[0.52, 0.84, 0.0, 1.0], [0.87, 0.29, 0.12, 1.0]],
];
const PLAYERS: [usize; 8] = [0, 0, 1, 0, 1, 0, 1, 1];

impl App {
    fn render(&mut self, args: &RenderArgs, c: graphics::Context, gl: &mut G2d) {
        let shrunk_scale =
            (args.width as f64 / self.size.0 as f64).min(args.height as f64 / self.size.1 as f64);
        let (cx, cy) = (args.width as f64 / 2.0, args.height as f64 / 2.0);

        clear([0.9, 0.9, 0.9, 1.0], gl);
        let w = shrunk_scale * (self.size.0 as f64);
        let h = shrunk_scale * (self.size.1 as f64);
        let transform = c.transform.trans(cx, cy).trans(-w / 2.0, -h / 2.0);

        for (column, amount) in [2, 3, 4, 5, 5, 4, 3, 2].iter().enumerate() {
            for y in 1..*amount {
                let x_pos = (column as f64 + 1.0) / 9.0 * w;
                let y_pos = (h * (y as f64) / (*amount as f64)
                    + [&self.p1, &self.p2][PLAYERS[column]].pos
                        / [3, 3, 4, 5, 5, 4, 3, 3][column] as f64 * h-h/2.0)*(1.0-1.0/100.0)+h/2.0;

                rectangle(
                    PALETTES[self.palette][PLAYERS[column]],
                    [x_pos - h/200.0, y_pos - h/20.0, h/100.0,  h/10.0],
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
            [0.7, 0.7, 0.7, 1.0],
            [
                self.ball.pos.0 - self.ball.radius,
                self.ball.pos.1 - self.ball.radius,
                self.ball.radius * 2.0,
                self.ball.radius * 2.0,
            ],
            transform,
            gl,
        );

        text(
            [0.7, 0.7, 0.7, 1.0],
            50,
            &(self.p1.score).to_string(),
            &mut self.glyphs,
            transform.trans(w / 2.0 - 50.0, 50.0),
            gl,
        ).unwrap();

        text(
            [0.7, 0.7, 0.7, 1.0],
            50,
            &(self.p2.score).to_string(),
            &mut self.glyphs,
            transform.trans(w / 2.0 + 50.0, 50.0),
            gl,
        ).unwrap();
    }
    //update game here (args.dt is delta time)
    fn update(&mut self, args: &UpdateArgs) {
        let step: f64 = args.dt * 60.0;
        let w = self.size.0 as f64;
        let h = self.size.1 as f64;
        self.p1.pos += self.p1.dir as f64 * step;
        self.p2.pos += self.p2.dir as f64 * step;

        self.p1.pos = self.p1.pos.min(1.0).max(-1.0);
        self.p2.pos = self.p2.pos.min(1.0).max(-1.0);

        self.ball.pos.0 += self.ball.vel.0 * 5.0 * step;
        self.ball.pos.1 += self.ball.vel.1 * 5.0 * step;

        if self.ball.pos.1 - self.ball.radius > h / 3.0
            && self.ball.pos.1 + self.ball.radius < h * 2.0 / 3.0
        {
            if self.ball.pos.0 + self.ball.radius > w {
                self.p1.score += 1;
            }
            if self.ball.pos.0 - self.ball.radius < 0.0 {
                self.p2.score += 1;
            }
        }

        if self.ball.pos.1 - self.ball.radius < 0.0 {
            self.ball.vel.1 = -self.ball.vel.1;
            self.ball.pos.1 = self.ball.radius;
        }
        if self.ball.pos.1 + self.ball.radius > h {
            self.ball.vel.1 = -self.ball.vel.1;
            self.ball.pos.1 = h - self.ball.radius;
        }
        if self.ball.pos.0 - self.ball.radius < 0.0 {
            self.ball.vel.0 = -self.ball.vel.0;
            self.ball.pos.0 = self.ball.radius;
        }
        if self.ball.pos.0 + self.ball.radius > w {
            self.ball.vel.0 = -self.ball.vel.0;
            self.ball.pos.0 = w - self.ball.radius;
        }

        for (column, amount) in [2, 3, 4, 5, 5, 4, 3, 2].iter().enumerate() {
            for y in 1..*amount {
                let x_pos = (column as f64 + 1.0) / 9.0 * w;
                let y_pos = (h * (y as f64) / (*amount as f64)
                    + [&self.p1, &self.p2][PLAYERS[column]].pos
                        / [3, 3, 4, 5, 5, 4, 3, 3][column] as f64 * h-h/2.0)*(1.0-1.0/100.0)+h/2.0;


                if self.ball.pos.0 - self.ball.radius < x_pos + h/200.0
                    && self.ball.pos.0 + self.ball.radius > x_pos - h/200.0
                    && self.ball.pos.1 - self.ball.radius < y_pos + h/20.0
                    && self.ball.pos.1 + self.ball.radius > y_pos - h/20.0
                {
                    let ball_x =
                        (self.ball.pos.0 - x_pos).abs() * (PLAYERS[column] as f64 * -2.0 + 1.0);
                    let ball_y = self.ball.pos.1 - y_pos;
                    let length = ball_x.hypot(ball_y);
                    self.ball.vel.0 = ball_x / length;
                    self.ball.vel.1 = ball_y / length;
                }
            }
        }
    }
}

fn main() {
    let size = (1050, 600);
    // Create an Glutin window.
    let mut window: PistonWindow = WindowSettings::new("Foosball", [size.0, size.1])
        .exit_on_esc(true)
        .build()
        .unwrap();
    let font = include_bytes!("../assets/FiraSans-Regular.ttf");
    let factory = window.factory.clone();
    let glyphs = Glyphs::from_bytes(font, factory, TextureSettings::new()).unwrap();
    // Create a new game and run it.
    let mut app = App {
        glyphs,
        palette: 0,
        size,
        ball: Ball {
            radius: 10.0,
            pos: (525.0, 300.0),
            vel: (1.0, 1.0),
        },
        p1: Player {
            score: 0,
            dir: 0.0,
            pos: 0.0,
        },
        p2: Player {
            score: 0,
            dir: 0.0,
            pos: 0.0,
        },
    };

    while let Some(e) = window.next() {
        if let Some(r) = e.render_args() {
            window.draw_2d(&e, |c, gl| {
                app.render(&r, c, gl);
            });
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }
        let speed = 0.05;
        if let Some(Button::Keyboard(key)) = e.press_args() {
            match key {
                Key::W => app.p1.dir = -speed,
                Key::S => app.p1.dir = speed,
                Key::Up => app.p2.dir = -speed,
                Key::Down => app.p2.dir = speed,
                Key::Space => app.palette = (app.palette + 1) % PALETTES.len(),
                _ => (),
            }
        }
        if let Some(Button::Keyboard(key)) = e.release_args() {
            match key {
                Key::W if app.p1.dir == -speed => app.p1.dir = 0.0,
                Key::S if app.p1.dir == speed => app.p1.dir = 0.0,
                Key::Up if app.p2.dir == -speed => app.p2.dir = 0.0,
                Key::Down if app.p2.dir == speed => app.p2.dir = 0.0,
                _ => (),
            }
        }
    }
}
