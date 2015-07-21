extern crate piston_window;
extern crate gfx_graphics;
extern crate gfx_device_gl;
extern crate gfx;

use std::f32;
use std::rc::Rc;
use std::cell::RefCell;
use na::{Pnt2, Pnt3, Iso2};
use na;
use nphysics::object::RigidBody;

use self::piston_window::{
    Ellipse,
    default_draw_state,
    DrawState,
};

use self::piston_window::ellipse::circle;
use self::piston_window::context::Context;

use self::gfx_graphics::GfxGraphics;
use self::gfx_device_gl::{Resources, Output};
use self::gfx::device::command::CommandBuffer;

pub static DRAW_SCALE: f32 = 20.0;

pub struct Ball {
    color: Pnt3<u8>,
    delta: Iso2<f32>,
    body:  Rc<RefCell<RigidBody>>,
    pos: Pnt2<f32>,
    rot: Pnt2<f32>,
    gfx: Ellipse,
    draw_state: &'static DrawState,
}

impl Ball {
    pub fn new(body:   Rc<RefCell<RigidBody>>,
               delta:  Iso2<f32>,
               radius: f32,
               color:  Pnt3<u8>) -> Ball {
        let dradius = radius as f32 * DRAW_SCALE;

        Ball {
            color: color,
            delta: delta,
            pos: Pnt2{x: dradius, y: dradius},
            rot: Pnt2{x: 0.0, y: 0.0},
            gfx: Ellipse::new([color.x as f32 / 255.0, color.y as f32 / 255.0, color.z as f32 / 255.0, 1.0]),
            body:  body,
            draw_state: default_draw_state(),
        }
    }
}

impl Ball {
    pub fn update(&mut self) {
        let body = self.body.borrow();
        let transform = *body.position() * self.delta;
        let pos = na::translation(&transform);
        let rot = na::rotation(&transform);

        self.pos.x = pos.x as f32 * DRAW_SCALE;
        self.pos.y = pos.y as f32 * DRAW_SCALE;

        self.rot.x = rot.x * 180.0 / f32::consts::PI as f32;

        if body.is_active() {
            self.gfx = Ellipse::new([self.color.x as f32 / 255.0, self.color.y as f32 / 255.0, self.color.z as f32 / 255.0, 1.0]);
        }
        else {
            self.gfx = Ellipse::new([self.color.x as f32 / 1023.0, self.color.y as f32 / 1023.0, self.color.z as f32 / 1023.0, 1.0]);
        }
    }

    pub fn draw(&self, c: Context, g: &mut GfxGraphics<Resources, CommandBuffer<Resources>, Output>) {
        self.gfx.draw(circle(self.pos.x as f64, self.pos.y as f64, 10.8), self.draw_state, c.transform, g);
    }
}
