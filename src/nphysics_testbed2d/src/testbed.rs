extern crate piston_window;

use std::env;
use std::rc::Rc;
use std::cell::RefCell;
use na::Pnt3;
use nphysics::world::World;
use nphysics::object::RigidBody;
use engine::GraphicsManager;

use self::piston_window::{
    PistonWindow,
    Size,
    WindowSettings,
    clear,
    Event,
};

fn usage(exe_name: &str) {
    println!("Usage: {} [OPTION] ", exe_name);
    println!("");
    println!("Options:");
    println!("    --help  - prints this help message and exits.");
    println!("    --pause - do not start the simulation right away.");
    println!("");
    println!("The following keyboard commands are supported:");
    println!("    t     - pause/continue the simulation.");
    println!("    s     - pause then execute only one simulation step.");
    println!("    space - display/hide contacts.");
}


#[derive(PartialEq)]
enum RunMode {
    Running,
    Stop,
    Step
}

pub struct Testbed {
    world:    World,
    pwindow:  PistonWindow,
    graphics: GraphicsManager
}

struct TestbedState {
    running: RunMode,
}

impl TestbedState {
    fn new() -> TestbedState {
        TestbedState{
            running: RunMode::Running,
        }
    }
}

impl Testbed {
    pub fn new_empty() -> Testbed {
        let graphics = GraphicsManager::new();

        let pwindow = WindowSettings::new(
            "nex",
            Size { width: 600, height: 500 })
            .exit_on_esc(true)
            .into();

        Testbed {
            world:    World::new(),
            pwindow:  pwindow,
            graphics: graphics
        }
    }

    pub fn new(world: World) -> Testbed {
        let mut res = Testbed::new_empty();

        res.set_world(world);

        res
    }

    pub fn set_world(&mut self, world: World) {
        self.world = world;
        self.graphics.clear();

        for rb in self.world.bodies() {
            self.graphics.add(rb.clone());
        }
    }

    pub fn set_color(&mut self, body: &Rc<RefCell<RigidBody>>, color: Pnt3<f32>) {
        let color = Pnt3::new(
            (color.x * 255.0) as u8,
            (color.y * 255.0) as u8,
            (color.z * 255.0) as u8
        );

        self.graphics.set_color(body, color);
    }

    pub fn run(&mut self) {
        let mut state = TestbedState::new();

        let mut args    = env::args();

        if args.len() > 1 {
            let exname = args.next().unwrap();
            for arg in args {
                if &arg[..] == "--help" || &arg[..] == "-h" {
                    usage(&exname[..]);
                    return;
                }
                else if &arg[..] == "--pause" {
                    state.running = RunMode::Stop;
                }
            }
        }

        self.run_loop(state);
    }

    fn run_loop(&mut self, mut state: TestbedState) {
        for e in self.pwindow.clone() {
            match e.clone().event.unwrap() {
                Event::Update(_) => {
                    self.progress_world(&mut state);
                }
                Event::Render(_) => {
                    self.graphics.draw_update();
                    e.draw_2d(|c, g| {
                        clear([0.0, 0.0, 0.0, 1.0], g);
                        self.graphics.draw(c, g);
                    });
                }
                _ => ()
            }
        }
    }

    fn progress_world(&mut self, state: &mut TestbedState) {
        if state.running != RunMode::Stop {
            self.world.step(0.016);
        }

        if state.running == RunMode::Step {
            state.running = RunMode::Stop;
        }
    }
}
