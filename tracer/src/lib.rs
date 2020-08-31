mod segments;
use crate::segments::{TraceableBoundingRect, NormalizeableSegment};
    use svgtypes::{PathParser};
    use svgtypes::PathSegment;
    use lyon_geom::{QuadraticBezierSegment, CubicBezierSegment, Arc, SvgArc, ArcFlags, LineSegment, Scalar};
    use lyon_geom::euclid::Point2D;
    use lyon_geom::euclid::UnknownUnit;
    use lyon_geom::euclid::Vector2D;
    use lyon_geom::euclid::default::Transform2D;
use lyon_geom::Segment;
    use lyon_geom::traits::Transformation;
    use std::f64;
    use std::marker::PhantomData;
    use lyon_geom::math::{Angle, Rect};
    use std::cmp::Ordering;
    use svg::node::element::path::Command;
use std::borrow::Borrow;
use num_traits::Float;
use lyon::path::Path;
use lyon::algorithms::walk::{RegularPattern, walk_along_path};
use lyon::algorithms::path::PathSlice;
use lyon::algorithms::path::iterator::*;
use lyon::algorithms::path::math::Point;
use pruif::Sample;


#[derive(Debug)]
    pub struct LaserPoint {
        pub on: bool,
        pub x: f64,
        pub y: f64,
    }
    pub struct TraceableSegment {
        //TODO: add more modifiers for the segment, eg: velocity/Opacity
       segment: usize
    }
    pub struct Tracer {
        max_displacement: f64,
        max_velocity: f64,
        max_acceleration: f64,
        path: Path,
    }
    impl Tracer {
        pub fn new(max_displacement:f64, max_velocity:f64, max_acceleration:f64, ) -> Self {
            Self {
                max_displacement,
                max_velocity,
                max_acceleration,
                path: Path::new(),
            }
        }

        pub fn trace_path(&self ,path: PathSlice) -> Vec<Sample> {
            let mut dots = Vec::new();
            let mut pattern = RegularPattern {
                callback: &mut |position: Point, _tangent, _distance| {
                    dots.push(convert_to_sample(LaserPoint{
                        on: true,
                        x: position.x as f64,
                        y: position.y as f64
                    }));
                    true // Return true to continue walking the path.
                },
                // Invoke the callback above at a regular interval of 3 units.
                interval: 0.01,
            };

            let tolerance = 0.01; // The path flattening tolerance.
            let start_offset = 0.0; // Start walking at the beginning of the path.
            walk_along_path(
                path.iter().flattened(tolerance),
                start_offset,
                &mut pattern
            );
            dots
        }
    }
    pub fn normalize_path(path: &mut Vec<Box<&mut dyn NormalizeableSegment<f32>>>) {
        let mut bounding_box = Rect::default();
        for segment in path.iter_mut() {
            bounding_box = bounding_box.union((*segment).bounding_rect().borrow());
        }
        let translation = Transform2D::create_translation(-bounding_box.origin.x, -bounding_box.origin.y);
        let scaling = Transform2D::create_scale(bounding_box.size.width.recip(), bounding_box.size.height.recip());

        for segment in path.iter_mut() {
            (segment).transform(translation.post_transform(scaling.borrow()));
        }
    }
    pub fn generate_trace_from_segments(path: &Vec<PathSegment>,
                                        max_dev: &f64,
                                        max_vel: &f64,
                                        max_acc: &f64,
                                        update_rate: &f64
    ) -> Vec<LaserPoint> {
        let max_displacement: f64 = max_vel / update_rate;
        let mut points: Vec<LaserPoint> = Vec::new();
        let mut starting_point: LaserPoint = LaserPoint {
            on: false,
            x: 0.0,
            y: 0.0,
        };

        points.push(starting_point);

        let mut last_cubic_ctrl_point:Point2D<f64, UnknownUnit> = Point2D {
            x: 0.0,
            y: 0.0,
            _unit: PhantomData
        };

        let mut last_path_start_point:Point2D<f64, UnknownUnit> = Point2D {
            x: 0.0,
            y: 0.0,
            _unit: PhantomData
        };
        let mut last_quadrartic_ctrl_point:Point2D<f64, UnknownUnit> = Point2D {
            x: 0.0,
            y: 0.0,
            _unit: PhantomData
        };

        for path_seg in path {
            let last_x = points.last().expect("What").x;
            let last_y = points.last().expect("What").y;
            let mut result: Vec<LaserPoint> = Vec::new();
            match path_seg {
                &PathSegment::MoveTo { abs, x, y } => {
                    let mut end_x = x;
                    let mut end_y = y;
                    if !abs {
                        end_x +=last_x;
                        end_y +=last_y;
                    }
                    last_path_start_point.x = end_x;
                    last_path_start_point.y = end_y;
                    //result = get_straight_travel(last_x, last_y, end_x, end_y, max_displacement, false);
                    result = get_straight_travel(last_x, last_y, end_x, end_y, 10000000000.0, false);

                }
                &PathSegment::LineTo { abs, x, y } => {
                    if abs {
                        result = get_straight_travel(last_x, last_y, x, y, max_displacement, true);
                    } else {
                        result = get_straight_travel(last_x, last_y, last_x + x, last_y + y, max_displacement, true);
                    }
                }
                &PathSegment::ClosePath { abs } => {
                    result = get_straight_travel(last_x, last_y, last_path_start_point.x, last_path_start_point.y, max_displacement, true);
                }
                &PathSegment::HorizontalLineTo { abs, x } => {
                    if abs {
                        result = get_straight_travel(last_x, last_y, x, last_y, max_displacement, true);
                    } else {
                        result = get_straight_travel(last_x, last_y, last_x + x, last_y, max_displacement, true);
                    }
                }
                &PathSegment::VerticalLineTo { abs, y } => {
                    if abs {
                        result = get_straight_travel(last_x, last_y, last_x, y, max_displacement, true);
                    } else {
                        result = get_straight_travel(last_x, last_y, last_x, last_y + y, max_displacement, true);
                    }
                }
                &PathSegment::EllipticalArc { abs, rx, ry, x_axis_rotation, large_arc, sweep, x, y } => {
                    let mut x2 = x;
                    let mut y2 = y;
                    if !abs {
                        x2 += last_x;
                        y2 += last_y;
                    }
                    result =get_arc_travel(last_x, last_y, x2, y2, rx, ry, x_axis_rotation,
                                           large_arc, sweep, max_displacement, true);
                }
                &PathSegment::CurveTo { abs, x1, y1, x2, y2, x, y } => {
                    if !abs {
                        result = get_cubic_travel(last_x, last_y, x1 + last_x, y1 + last_y, x2 + last_x, y2 + last_y, x + last_x, y + last_y, max_displacement, true);
                        last_cubic_ctrl_point.x = x2 + last_x;
                        last_cubic_ctrl_point.y = y2 + last_y;
                    }
                    else {
                        result = get_cubic_travel(last_x, last_y, x1, y1, x2, y2, x, y, max_displacement, true);
                        last_cubic_ctrl_point.x = x2;
                        last_cubic_ctrl_point.y = y2;
                    }
                }
                _ => {panic!("Unhandled path type!");}
            }
            points.append(&mut result);
        }

        return points;
    }

    fn get_straight_travel(x1: f64, y1: f64, x2: f64, y2: f64, step_length: f64, on: bool) -> Vec<LaserPoint> {
        let mut line_points: Vec<LaserPoint> = Vec::new();
        line_points.push(LaserPoint {
            on,
            x: x1,
            y: y1
        });

        let x_disp = (x2 - x1);
        let y_disp = (y2 - y1);
        let total_length = (x_disp * x_disp + y_disp * y_disp).sqrt();
        let times_fit = (total_length / step_length).floor() as i32;
        let xstep = x_disp / times_fit as f64;
        let ystep = y_disp / times_fit as f64;
        let mut xloc = x1;
        let mut yloc = y1;

        for step in 0..times_fit {
            xloc += xstep;
            yloc += ystep;
            line_points.push(LaserPoint {
                on,
                x: xloc,
                y: yloc
            });
        }
        line_points.push(LaserPoint {
            on: false,
            x: x2,
            y: y2
        });
        return line_points;
    }

    fn get_arc_travel(x1: f64, y1: f64, x2: f64, y2: f64, rx: f64, ry: f64, xrot: f64,
                      large_arc: bool, sweep: bool, step_length: f64, on:bool) -> Vec<LaserPoint> {
        let mut arc_points: Vec<LaserPoint> = Vec::new();
        let svg_arc: SvgArc<f32> = SvgArc {
            from: Point2D {
                x: x1 as f32,
                y: y1 as f32,
                _unit: PhantomData
            },
            to: Point2D {
                x: x2 as f32,
                y: y2 as f32,
                _unit: PhantomData
            },
            radii: Vector2D { x: rx as f32, y: ry as f32, _unit: PhantomData },
            x_rotation: Angle { radians: xrot as f32 },
            flags: ArcFlags { large_arc, sweep }
        };
        let arc = svg_arc.to_arc();
        let length = arc.approximate_length((step_length/2.0) as f32);
        let num_of_steps:u32 = (length/ step_length as f32).ceil() as u32;
        for step in 0..num_of_steps {
            let t =(step + 1) as f32/num_of_steps as f32;
            let sample_point = arc.sample(t);
            arc_points.push(LaserPoint {
                on,
                x: sample_point.x as f64,
                y: sample_point.y as f64
            });
        }
        return arc_points;
    }
    fn get_cubic_travel(fromx: f64, fromy: f64, x1: f64, y1: f64, x2: f64, y2: f64, tox: f64, toy: f64
                        , step_length: f64, on:bool) -> Vec<LaserPoint> {

        let mut besier_points: Vec<LaserPoint> = Vec::new();
        let besier_seg = CubicBezierSegment {
            from: Point2D {
                x: fromx,
                y: fromy,
                _unit: PhantomData
            },
            ctrl1: Point2D {
                x: x1,
                y: y1,
                _unit: PhantomData
            },
            ctrl2: Point2D {
                x: x2,
                y: y2,
                _unit: PhantomData
            },
            to: Point2D {
                x: tox,
                y: toy,
                _unit: PhantomData
            }
        };
        let length = besier_seg.approximate_length(step_length/2.0);
        let num_of_steps:u32 = (length/ step_length).ceil() as u32;
        for step in 0..num_of_steps {
            let t =(step + 1) as f64/num_of_steps as f64;
            let sample_point = besier_seg.sample(t);
            besier_points.push(LaserPoint {
                on,
                x: sample_point.x as f64,
                y: sample_point.y as f64
            });
        }
        return besier_points;
    }

    #[cfg(test)]
    mod tests {
        // Note this useful idiom: importing names from outer (for mod tests) scope.
        use super::*;
        #[test]
        fn test_normlize_path() {
            let mut segment1= LineSegment{ from: Point2D {
                x: -100.0 as f32,
                y: -100.0 as f32,
                _unit: PhantomData
            }, to: Point2D {
                x: 10.0 as f32,
                y: 10.0 as f32,
                _unit: PhantomData
            } };
            let mut segment2= LineSegment{ from: Point2D {
                x: -20.0 as f32,
                y: -20.0 as f32,
                _unit: PhantomData
            }, to: Point2D {
                x: 100.0 as f32,
                y: 100.0 as f32,
                _unit: PhantomData
            } };
            let mut path = Vec::new();
            path.push(Box::new(&mut segment1 as &mut dyn NormalizeableSegment<f32>));
            path.push(Box::new(&mut segment2 as &mut dyn NormalizeableSegment<f32>));
            normalize_path(&mut path);
            assert_eq!(segment1, LineSegment{ from: Point2D { x: 0.0 as f32, y: 0.0 as f32, _unit: PhantomData },
                to: Point2D { x: 0.55 as f32, y: 0.55 as f32, _unit: PhantomData } });
            assert_eq!(segment2, LineSegment{ from: Point2D { x: 0.4 as f32, y: 0.4 as f32, _unit: PhantomData },
                to: Point2D { x: 1.0 as f32, y: 1.0 as f32, _unit: PhantomData } });
        }
    }


fn convert_to_sample(point: LaserPoint) -> Sample {
    Sample{
        voltage_x: ((point.x-0.5) * 10f64) as f32,
        voltage_y: ((point.y-0.5) * 10f64) as f32,
        laser_on: point.on}
}
