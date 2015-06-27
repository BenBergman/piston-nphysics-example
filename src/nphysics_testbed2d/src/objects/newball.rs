extern crate piston_window;

use std::f32;
use std::rc::Rc;
use std::cell::RefCell;
use sfml::graphics;
use sfml::graphics::{CircleShape, Color, RenderTarget};
use sfml::system::vector2;
use na::{Pnt3, Iso2};
use na;
use nphysics::object::RigidBody;
use draw_helper::DRAW_SCALE;

use self::piston_window::{
    Ellipse,
    PistonWindow,
    clear,
    default_draw_state,
};
use self::piston_window::ellipse::circle;

pub struct Ball<'a> {
    color: Pnt3<u8>,
    base_color: Pnt3<u8>,
    delta: Iso2<f32>,
    body:  Rc<RefCell<RigidBody>>,
    gfx:   CircleShape<'a>,
    newgfx: Ellipse,
}

impl<'a> Ball<'a> {
    pub fn new(body:   Rc<RefCell<RigidBody>>,
               delta:  Iso2<f32>,
               radius: f32,
               color:  Pnt3<u8>) -> Ball<'a> {
        let dradius = radius as f32 * DRAW_SCALE;

        let mut res = Ball {
            color: color,
            base_color: color,
            delta: delta,
            gfx:   CircleShape::new().unwrap(),
            newgfx: Ellipse::new([color.x as f32 / 255.0, color.y as f32 / 255.0, color.z as f32 / 255.0, 1.0]),
            body:  body
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

    pub fn draw(&self, rw: &mut graphics::RenderWindow) {
        rw.draw(&self.gfx);
    }

    pub fn new_draw(&self, e: &mut PistonWindow) {
        e.draw_2d(|c, g| {
            let pos = self.gfx.get_position();
            self.newgfx.draw(circle(pos.x as f64, pos.y as f64, self.gfx.get_radius() as f64), default_draw_state(), c.transform, g);
        });
    }

    pub fn select(&mut self) {
        self.color = Pnt3::new(200, 0, 0);
    }

    pub fn unselect(&mut self) {
        self.color = self.base_color;
    }
}
