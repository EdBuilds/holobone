use lyon_geom::{QuadraticBezierSegment, CubicBezierSegment, SvgArc, LineSegment, Scalar};
//use lyon_geon::euclid::default::Rect;
use lyon_geom::euclid::default::Transform2D;
use lyon_geom::euclid::default::Rect;
use lyon_geom::math::Size;

pub trait TraceableBoundingRect<S> {
    fn bounding_rect(& self) -> Rect<S>;
}

pub trait NormalizeableSegment<S> {

    fn bounding_rect(& self) -> Rect<S>;
    fn transform(&mut self, transform: Transform2D<S>);
}
impl<S: Scalar> TraceableBoundingRect<S> for CubicBezierSegment<S> {
    fn bounding_rect(& self) -> Rect<S> {
        self.bounding_rect()
    }
}
impl<S: Scalar> TraceableBoundingRect<S> for QuadraticBezierSegment<S> {
    fn bounding_rect(& self) -> Rect<S> {
        self.bounding_rect()
    }
}

impl<S: Scalar> TraceableBoundingRect<S> for LineSegment<S> {
    fn bounding_rect(& self) -> Rect<S> {
        self.bounding_rect()
    }
}

impl<S: Scalar> NormalizeableSegment<S> for SvgArc<S> {
    fn bounding_rect(& self) -> Rect<S> {
        self.bounding_rect()
    }
    fn transform(&mut self, transform: Transform2D<S>) {
        self.from = transform.transform_point(self.from);
        self.to = transform.transform_point(self.to);
    }
}

impl<S: Scalar> NormalizeableSegment<S> for LineSegment<S> {
    fn bounding_rect(& self) -> Rect<S> {
        self.bounding_rect()
    }

    fn transform(&mut self, transform: Transform2D<S>) {
       self.from = transform.transform_point(self.from);
        self.to = transform.transform_point(self.to);

    }
}
