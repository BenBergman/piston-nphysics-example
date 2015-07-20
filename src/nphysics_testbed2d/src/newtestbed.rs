extern crate piston_window;

use std::env;
use std::rc::Rc;
use std::cell::RefCell;
use na::{Pnt2, Pnt3, Iso2};
use na;
use nphysics::world::World;
use nphysics::object::RigidBody;
use nphysics::detection::joint::{Fixed, Anchor};
use camera::Camera;
use newengine::GraphicsManager;
use draw_helper;

use self::piston_window::{
    PistonWindow,
    Size,
    WindowSettings,
    clear,
    Event,
    Input,
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

pub struct NewTestbed<'a> {
    world:    World,
    pwindow:  PistonWindow,
    graphics: GraphicsManager<'a>
}

struct TestbedState {
    running: RunMode,
    draw_colls: bool,
    camera: Camera,
    grabbed_object: Option<Rc<RefCell<RigidBody>>>,
    grabbed_object_joint: Option<Rc<RefCell<Fixed>>>,
}

impl TestbedState {
    fn new() -> TestbedState {
        TestbedState{
            running: RunMode::Running,
            draw_colls: false,
            camera: Camera::new(),
            grabbed_object: None,
            grabbed_object_joint: None,
        }
    }
}

impl<'a> NewTestbed<'a> {
    pub fn new_empty() -> NewTestbed<'a> {
        let graphics = GraphicsManager::new();

        let pwindow = WindowSettings::new(
            "nex",
            Size { width: 600, height: 500 })
            .exit_on_esc(true)
            .into();

        NewTestbed {
            world:    World::new(),
            pwindow:  pwindow,
            graphics: graphics
        }
    }

    pub fn new(world: World) -> NewTestbed<'a> {
        let mut res = NewTestbed::new_empty();

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
        let font_mem = include_bytes!("Inconsolata.otf");

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
        for mut e in self.pwindow.clone() {
            match e.clone().event.unwrap() {
                //Event::Input(Press(Keyboard(_))) => {
                Event::Update(_) => {
                    self.progress_world(&mut state);
                }

                Event::Render(_) => {
                    self.graphics.draw_update();
                    e.draw_2d(|c, g| {
                        clear([0.0, 0.0, 0.0, 1.0], g);
                        self.graphics.new_draw(c, g);
                    });
                }
                //}

                _ => ()
            }
        }
    }

    /*
    fn process_key_press(&mut self, state: &mut TestbedState, code: Key) {
        match code {
            Key::S      => state.running = RunMode::Step,
            Key::Space  => state.draw_colls = !state.draw_colls,
            Key::T      => {
                if state.running == RunMode::Stop {
                    state.running = RunMode::Running;
                }
                else {
                    state.running = RunMode::Stop;
                }
            },
            _                => { }
        }
    }

    fn process_mouse_press(&mut self, state: &mut TestbedState, button: MouseButton, x: i32, y: i32) {
        match button {
            MouseButton::MouseLeft => {
                let mapped_coords = state.camera.map_pixel_to_coords(Vector2i::new(x, y));
                let mapped_point = Pnt2::new(mapped_coords.x, mapped_coords.y);
                self.world.interferences_with_point(&mapped_point, |b| {
                    if b.borrow().can_move() {
                        state.grabbed_object = Some(b.clone())
                    }
                });

                match state.grabbed_object {
                    Some(ref b) => {
                        for node in self.graphics.body_to_scene_node(b).unwrap().iter_mut() {
                            match state.grabbed_object_joint {
                                Some(ref j) => self.world.remove_fixed(j),
                                None        => { }
                            }

                            let _1: Iso2<f32> = na::one();
                            let attach2 = na::append_translation(&_1, mapped_point.as_vec());
                            let attach1 = na::inv(&na::transformation(b.borrow().position())).unwrap() * attach2;
                            let anchor1 = Anchor::new(Some(state.grabbed_object.as_ref().unwrap().clone()), attach1);
                            let anchor2 = Anchor::new(None, attach2);
                            let joint = Fixed::new(anchor1, anchor2);
                            state.grabbed_object_joint = Some(self.world.add_fixed(joint));
                            node.select()
                        }
                    },
                    None => { }
                }
            },
            _ => {
                state.camera.handle_event(&event::MouseButtonPressed{ button: button, x: x, y: y })
            }
        }
    }

    fn process_mouse_release(&mut self, state: &mut TestbedState, button: MouseButton, x: i32, y: i32) {
        match button {
            MouseButton::MouseLeft => {
                match state.grabbed_object {
                    Some(ref b) => {
                        for node in self.graphics.body_to_scene_node(b).unwrap().iter_mut() {
                            node.unselect()
                        }
                    },
                    None => { }
                }

                match state.grabbed_object_joint {
                    Some(ref j) => self.world.remove_fixed(j),
                    None => { }
                }

                state.grabbed_object = None;
                state.grabbed_object_joint = None;
            },
            _ => {
                state.camera.handle_event(&event::MouseButtonReleased{ button: button, x: x, y: y })
            }
        }
    }

    fn process_mouse_moved(&mut self, state: &mut TestbedState, x: i32, y: i32) {
        let mapped_coords = state.camera.map_pixel_to_coords(Vector2i::new(x, y));
        let mapped_point = Pnt2::new(mapped_coords.x, mapped_coords.y);
        let _1: Iso2<f32> = na::one();
        let attach2 = na::append_translation(&_1, (mapped_point).as_vec());
        match state.grabbed_object {
            Some(_) => {
                let joint = state.grabbed_object_joint.as_ref().unwrap();
                joint.borrow_mut().set_local2(attach2);
            },
            None => state.camera.handle_event(&event::MouseMoved{x: x, y: y})
        };
    }
    */

    fn progress_world(&mut self, state: &mut TestbedState) {
        if state.running != RunMode::Stop {
            self.world.step(0.016);
        }

        if state.running == RunMode::Step {
            state.running = RunMode::Stop;
        }
    }

    fn draw_collisions(&mut self, state: &mut TestbedState) {
        if state.draw_colls {
            //draw_helper::draw_colls(&mut self.window, &mut self.world);
        }
    }
}
