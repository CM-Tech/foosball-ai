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
    let CircleRad = 2.0*f64::consts::PI;
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
        let mut n:f64=0.0;
    while let Some(e) = window.next() {
        n=n+0.01;
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
                      if let Some(r) = e.render_args() {
            let args=&r;
        
                      
                      let (x, y) = ((args.width / 2) as f64,
                      (args.height / 2) as f64);
            let transform2 = c.transform.trans(x,y)
                            .rot_rad(n)
                            .trans(-50.0, -50.0);
            rectangle([0.87, 0.29, 0.12, 1.0],      // red
                      [0.0,0.0, 100.0, 100.0], // rectangle
                      transform2, g);
                      }
        });
    }
}
