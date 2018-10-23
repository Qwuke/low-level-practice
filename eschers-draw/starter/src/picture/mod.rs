//! Descriptions of scenes

use std::rc::Rc;

use canvas::Box as Bx;
use canvas::*;
use shape::Shape;
use style::Style;

/// A collection of Shapes to draw
pub type Rendering = Vec<(Shape, Style)>;

/// the blank picture
pub fn blank() -> Rc<impl Fn(&Bx) -> Rendering> {
    Rc::new(|_bx: &Bx| Vec::new())
}

/// Turn the picture
pub fn turn<Picture>(picture: Rc<Picture>) -> Rc<impl Fn(&Bx) -> Rendering>
where
    Picture: Fn(&Bx) -> Rendering,
{
    let p = picture.clone();
    Rc::new(move |bx: &Bx| {
        let turned_box = turn_box(&bx);
        p(&turned_box)
    })
}

/// Flip the picture
pub fn flip<Picture>(picture: Rc<Picture>) -> Rc<impl Fn(&Bx) -> Rendering>
where
    Picture: Fn(&Bx) -> Rendering,
{
    let p = picture.clone();
    Rc::new(move |bx: &Bx| {
        let turned_box = flip_box(&bx);
        p(&turned_box)
    })
}

/// Toss the picture
pub fn toss<Picture>(picture: Rc<Picture>) -> Rc<impl Fn(&Bx) -> Rendering>
where
    Picture: Fn(&Bx) -> Rendering,
{
    let p = picture.clone();
    Rc::new(move |bx: &Bx| {
        let turned_box = toss_box(&bx);
        p(&turned_box)
    })
}

/// Stack pictures above each other according to weight
pub fn above_ratio<P, Q>(
    picture_p: Rc<P>,
    picture_q: Rc<Q>,
    m: u8,
    n: u8,
) -> Rc<impl Fn(&Bx) -> Rendering>
where
    P: Fn(&Bx) -> Rendering,
    Q: Fn(&Bx) -> Rendering,
{
    let p = picture_p.clone();
    let q = picture_q.clone();
    Rc::new(move |&bx: &Bx| {
        let factor: f64 = m as f64/((m + n ) as f64);
        let mut result = vec![];
        let (top_box, bottom_box) = split_box_horizontally(factor, &bx);
        result.extend(p(&top_box));
        result.extend(q(&bottom_box));
        result
    })
}

/// Stack pictures above each other with equal weight
pub fn above<P, Q>(p: Rc<P>, q: Rc<Q>) -> Rc<impl Fn(&Bx) -> Rendering>
where
    P: Fn(&Bx) -> Rendering,
    Q: Fn(&Bx) -> Rendering,
{
  
    above_ratio(p, q, 1, 1)
}

/// Stack pictures beside each other according to weight
pub fn beside_ratio<P, Q>(
    picture_p: Rc<P>,
    picture_q: Rc<Q>,
    m: u8,
    n: u8,
) -> Rc<impl Fn(&Bx) -> Rendering>
where
    P: Fn(&Bx) -> Rendering,
    Q: Fn(&Bx) -> Rendering,
{
    let p = picture_p.clone();
    let q = picture_q.clone();
    Rc::new(move |&bx: &Bx| {
        let factor: f64 = m as f64/((m + n ) as f64);
        let mut result = vec![];
        let (top_box, bottom_box) = split_box_vertically(factor, &bx);
        result.extend(p(&top_box));
        result.extend(q(&bottom_box));
        result
    })
}

/// Stack pictures above each other with equal weight
pub fn beside<P, Q>(p: Rc<P>, q: Rc<Q>) -> Rc<impl Fn(&Bx) -> Rendering>
where
    P: Fn(&Bx) -> Rendering,
    Q: Fn(&Bx) -> Rendering,
{
    beside_ratio(p, q, 1, 1)
}

/// Create a quartet of pictures
pub fn quartet<P, Q, R, S>(
    nw: Rc<P>,
    ne: Rc<Q>,
    sw: Rc<R>,
    se: Rc<S>,
) -> Rc<impl Fn(&Bx) -> Rendering>
where
    P: Fn(&Bx) -> Rendering,
    Q: Fn(&Bx) -> Rendering,
    R: Fn(&Bx) -> Rendering,
    S: Fn(&Bx) -> Rendering,
{

    Rc::new( move |&bx: &Bx| {
        let (nw, ne) = besides(ne, nw);
        let (sw, se) = above(se, sw);
        (nw, ne, sw, se)
    }
}

/// Create a nonet of pictures
pub fn nonet<P, Q, R, S, T, U, V, W, X>(
    nw: Rc<P>,
    nm: Rc<Q>,
    ne: Rc<R>,
    mw: Rc<S>,
    mm: Rc<T>,
    me: Rc<U>,
    sw: Rc<V>,
    sm: Rc<W>,
    se: Rc<X>,
) -> Rc<impl Fn(&Bx) -> Rendering>
where
    P: Fn(&Bx) -> Rendering,
    Q: Fn(&Bx) -> Rendering,
    R: Fn(&Bx) -> Rendering,
    S: Fn(&Bx) -> Rendering,
    T: Fn(&Bx) -> Rendering,
    U: Fn(&Bx) -> Rendering,
    V: Fn(&Bx) -> Rendering,
    W: Fn(&Bx) -> Rendering,
    X: Fn(&Bx) -> Rendering,
{
    blank()
}

fn column<P, Q, R>(n: Rc<P>, m: Rc<Q>, s: Rc<R>) -> Rc<impl Fn(&Bx) -> Rendering>
where
    P: Fn(&Bx) -> Rendering,
    Q: Fn(&Bx) -> Rendering,
    R: Fn(&Bx) -> Rendering,
{
    blank()
}

fn row<P, Q, R>(w: Rc<P>, m: Rc<Q>, e: Rc<R>) -> Rc<impl Fn(&Bx) -> Rendering>
where
    P: Fn(&Bx) -> Rendering,
    Q: Fn(&Bx) -> Rendering,
    R: Fn(&Bx) -> Rendering,
{
    blank()
}

/// Place two pictures over each other
pub fn over<P, Q>(picture_p: Rc<P>, picture_q: Rc<Q>) -> Rc<impl Fn(&Bx) -> Rendering>
where
    P: Fn(&Bx) -> Rendering,
    Q: Fn(&Bx) -> Rendering,
{
    blank()
}

/// The T-tile
pub fn ttile<P>(p: Rc<P>) -> Rc<impl Fn(&Bx) -> Rendering>
where
    P: Fn(&Bx) -> Rendering,
{
    blank()
}

/// The T-tile
pub fn utile<P>(p: Rc<P>) -> Rc<impl Fn(&Bx) -> Rendering>
where
    P: Fn(&Bx) -> Rendering,
{
    blank()
}

/// The side of the square limit
pub fn side<P>(picture_p: Rc<P>, n: u8) -> Rc<impl Fn(&Bx) -> Rendering>
where
    P: Fn(&Bx) -> Rendering,
{
    blank()
}

/// The corner of the square limit
pub fn corner<P>(picture_p: Rc<P>, n: u8) -> Rc<impl Fn(&Bx) -> Rendering>
where
    P: Fn(&Bx) -> Rendering,
{
    blank()
}

/// The ultimate goal: Escher's Square Limit
pub fn square_limit<P>(picture_p: Rc<P>, n: u8) -> Rc<impl Fn(&Bx) -> Rendering>
where
    P: Fn(&Bx) -> Rendering,
{
    blank()
}
