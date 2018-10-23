//! The canvas that we will draw on

use vector::Vector;

/// A Box represents the area and position that we will draw in.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Box {
    /// Determines the origin of the drawing area, used to position the box.
    pub a: Vector<f64>,
    /// Determines the x axis of the box.
    pub b: Vector<f64>,
    /// Determines the y axis of the box.
    pub c: Vector<f64>,
}


impl Box {
    /// Create a box from corresponding vectors
    pub fn new(a: Vector<f64>, b: Vector<f64>, c: Vector<f64>) -> Box {
        Box { a, b, c }
    }
}

/// The identity function for Box
pub fn identity(bx: &Box) -> Box {
    Box::new(bx.a, bx.b, bx.c)
}

/// Rotate box through 90 degrees
pub fn turn_box(bx: &Box) -> Box {
    Box::new(flip_vec(bx.a), flip_vec(bx.b), flip_vec(bx.c))
}

fn flip_vec(vec: Vector<f64>) -> Vector<f64> {
    Vector::new(vec.y, vec.x)
}


/// Flip box vertically
pub fn flip_box(bx: &Box) -> Box {
    Box::new(bx.a.add(&bx.b), bx.b.neg(), bx.c)
}

/// Toss box
pub fn toss_box(bx: &Box) -> Box {
    Box::new(bx.b.add(&bx.c).scale(&0.5).add(&bx.a), bx.b.add(&bx.c).scale(&0.5), bx.c.sub(&bx.b).scale(&0.5))
}

/// Create two boxes above each other that together make up the original box.
pub fn split_box_horizontally(factor: f64, bx: &Box) -> (Box, Box) {
    let factor2 = 1.0 - factor;
    let box1 = flip_box(&scale_box_horizontally(factor, &identity(&bx)));
    let box2 = scale_box_horizontally(factor2, &identity(&bx));
    move_box_vertically(&box1, &box2)
}

fn scale_box_horizontally(factor: f64, bx: &Box) -> Box {
    Box::new(bx.a, bx.b, bx.c.scale(&factor))
}

fn move_box_vertically(bx1: &Box, bx2: &Box) -> (Box, Box) {
    let bx1 = Box::new(bx1.a.add(&bx2.c), bx1.b, bx1.c);
    (bx1, *bx2)
}

/// Create two boxes beside each other that together make up the original box.
pub fn split_box_vertically(factor: f64, bx: &Box) -> (Box, Box) {
    let factor2 = 1.0 - factor;
    let box1 = flip_box(&scale_box_vertically(factor, &bx));
    let box2 = scale_box_vertically(factor2, &bx);
    move_box_horizontally(&box1, &box2)
}

fn scale_box_vertically(factor: f64, bx: &Box) -> Box {
    Box::new(bx.a, bx.b.scale(&factor), bx.c)
}

fn move_box_horizontally(bx1: &Box, bx2: &Box) -> (Box, Box) {
    let bx1 = Box::new(bx1.a.add(&bx2.b), bx1.b, bx1.c);
    (bx1, *bx2)
}

