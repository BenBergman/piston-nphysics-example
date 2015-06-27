extern crate piston_window;

use std::rc::Rc;
use std::cell::RefCell;
use std::sync::Arc;
use std::collections::HashMap;
use rand::{SeedableRng, XorShiftRng, Rng};
use sfml::graphics::RenderWindow;
use na::{Pnt2, Pnt3, Iso2};
use na;
use nphysics::object::RigidBody;
use ncollide::inspection::Repr2;
use ncollide::shape;
use camera::Camera;
use objects::newball::Ball;

use self::piston_window::{
    Ellipse,
    PistonWindow,
    clear,
};

pub enum SceneNode<'a> {
    BallNode(Ball<'a>),
}

impl<'a> SceneNode<'a> {
    pub fn select(&mut self) {
        match *self {
            SceneNode::BallNode(ref mut n) => n.select(),
        }
    }

    pub fn unselect(&mut self) {
        match *self {
            SceneNode::BallNode(ref mut n) => n.unselect(),
        }
    }
}

pub struct GraphicsManager<'a> {
    rand:      XorShiftRng,
    rb2sn:     HashMap<usize, Vec<SceneNode<'a>>>,
    obj2color: HashMap<usize, Pnt3<u8>>
}

impl<'a> GraphicsManager<'a> {
    pub fn new() -> GraphicsManager<'a> {
        GraphicsManager {
            rand:      SeedableRng::from_seed([0, 1, 2, 3]),
            rb2sn:     HashMap::new(),
            obj2color: HashMap::new()
        }
    }

    pub fn add(&mut self, body: Rc<RefCell<RigidBody>>) {

        let nodes = {
            let rb    = body.borrow();
            let mut nodes = Vec::new();

            self.add_shape(body.clone(), na::one(), rb.shape_ref(), &mut nodes);

            nodes
        };

        self.rb2sn.insert(&*body as *const RefCell<RigidBody> as usize, nodes);
    }

    fn add_shape(&mut self,
                 body:  Rc<RefCell<RigidBody>>,
                 delta: Iso2<f32>,
                 shape: &Repr2<f32>,
                 out:   &mut Vec<SceneNode<'a>>) {
        type Pl = shape::Plane2<f32>;
        type Bl = shape::Ball2<f32>;
        type Cx = shape::Convex2<f32>;
        type Bo = shape::Cuboid2<f32>;
        type Cy = shape::Cylinder2<f32>;
        type Co = shape::Cone2<f32>;
        type Cm = shape::Compound2<f32>;
        type Ls = shape::Polyline2<f32>;
        type Se = shape::Segment2<f32>;

        let repr = shape.repr();

        if let Some(s) = repr.downcast_ref::<Pl>() {
            self.add_plane(body, s, out)
        }
        else if let Some(s) = repr.downcast_ref::<Bl>() {
            self.add_ball(body, delta, s, out)
        }
        else {
            panic!("Not yet implemented.")
        }

    }

    fn add_plane(&mut self,
                 _: Rc<RefCell<RigidBody>>,
                 _: &shape::Plane2<f32>,
                 _: &mut Vec<SceneNode>) {
    }

    fn add_ball(&mut self,
                body:  Rc<RefCell<RigidBody>>,
                delta: Iso2<f32>,
                shape: &shape::Ball2<f32>,
                out:   &mut Vec<SceneNode>) {
        let color = self.color_for_object(&body);
        let margin = body.borrow().margin();
        out.push(SceneNode::BallNode(Ball::new(body, delta, shape.radius() + margin, color)))
    }
    


    pub fn clear(&mut self) {
        self.rb2sn.clear();
    }

    pub fn draw_update(&mut self) {
        for (_, ns) in self.rb2sn.iter_mut() {
            for n in ns.iter_mut() {
                match *n {
                    SceneNode::BallNode(ref mut n) => n.update(),
                }
            }
        }
    }

    pub fn draw(&mut self, rw: &mut RenderWindow, c: &Camera) {
        c.activate_scene(rw);

        for (_, ns) in self.rb2sn.iter_mut() {
            for n in ns.iter_mut() {
                match *n {
                    SceneNode::BallNode(ref n) => n.draw(rw),
                }
            }
        }

        c.activate_ui(rw);
    }

    pub fn new_draw(&mut self, e: &mut PistonWindow, c: &Camera) {
        for (_, ns) in self.rb2sn.iter_mut() {
            for n in ns.iter_mut() {
                match *n {
                    SceneNode::BallNode(ref n) => n.new_draw(e),
                }
            }
        }
    }

    pub fn set_color(&mut self, body: &Rc<RefCell<RigidBody>>, color: Pnt3<u8>) {
        let key = &**body as *const RefCell<RigidBody> as usize;
        self.obj2color.insert(key, color);
    }

    pub fn color_for_object(&mut self, body: &Rc<RefCell<RigidBody>>) -> Pnt3<u8> {
        let key = &**body as *const RefCell<RigidBody> as usize;
        match self.obj2color.get(&key) {
            Some(color) => return *color,
            None => { }
        }

        let color = Pnt3::new(
            self.rand.gen_range(0usize, 256) as u8,
            self.rand.gen_range(0usize, 256) as u8,
            self.rand.gen_range(0usize, 256) as u8);


        self.obj2color.insert(key, color);

        color
    }

    pub fn body_to_scene_node(&mut self, rb: &Rc<RefCell<RigidBody>>) -> Option<&mut Vec<SceneNode<'a>>> {
        self.rb2sn.get_mut(&(&**rb as *const RefCell<RigidBody> as usize))
    }
}