extern crate piston_window;
extern crate gfx_graphics;
extern crate gfx_device_gl;
extern crate gfx;

use std::f32;
use std::rc::Rc;
use std::cell::RefCell;
use sfml::graphics::{CircleShape, Color, RenderTarget};
use sfml::system::vector2;
use na::{Pnt3, Iso2};
use na;
use nphysics::object::RigidBody;
use draw_helper::DRAW_SCALE;

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

pub struct Ball<'a> {
    color: Pnt3<u8>,
    delta: Iso2<f32>,
    body:  Rc<RefCell<RigidBody>>,
    gfx:   CircleShape<'a>,
    newgfx: Ellipse,
    draw_state: &'static DrawState,
}

impl<'a> Ball<'a> {
    pub fn new(body:   Rc<RefCell<RigidBody>>,
               delta:  Iso2<f32>,
               radius: f32,
               color:  Pnt3<u8>) -> Ball<'a> {
        let dradius = radius as f32 * DRAW_SCALE;

        let mut res = Ball {
            color: color,
            delta: delta,
            gfx:   CircleShape::new().unwrap(),
            newgfx: Ellipse::new([color.x as f32 / 255.0, color.y as f32 / 255.0, color.z as f32 / 255.0, 1.0]),
            body:  body,
            draw_state: default_draw_state(),
        };

        res.gfx.set_fill_color(&Color::new_rgb(color.x, color.y, color.z));
        res.gfx.set_radius(dradius);
        res.gfx.set_origin(&vector2::Vector2f { x: dradius, y: dradius }); 

        res
    }
}

impl<'a> Ball<'a> {
    pub fn update(&mut self) {
        let body = self.body.borrow();
        let transform = *body.position() * self.delta;
        let pos = na::translation(&transform);
        let rot = na::rotation(&transform);

        self.gfx.set_position(&vector2::Vector2f {
            x: pos.x as f32 * DRAW_SCALE,
            y: pos.y as f32 * DRAW_SCALE
        });
        self.gfx.set_rotation(rot.x * 180.0 / f32::consts::PI as f32);

        if body.is_active() {
            self.gfx.set_fill_color(
                &Color::new_rgb(self.color.x, self.color.y, self.color.z));
        }
        else {
            self.gfx.set_fill_color(
                &Color::new_rgb(self.color.x / 4, self.color.y / 4, self.color.z / 4));
        }
    }

    pub fn new_draw(&self, c: Context, g: &mut GfxGraphics<Resources, CommandBuffer<Resources>, Output>) {
        let pos = self.gfx.get_position();
        self.newgfx.draw(circle(pos.x as f64, pos.y as f64, 10.8), self.draw_state, c.transform, g);
    }
}
