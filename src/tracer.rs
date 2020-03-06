pub mod tracer {
   use svgtypes::{PathParser};
   use svgtypes::PathSegment;
   use lyon_geom::{QuadraticBezierSegment, CubicBezierSegment, Arc, SvgArc, ArcFlags};
   use lyon_geom::euclid::Point2D;
   use lyon_geom::euclid::UnknownUnit;
   use lyon_geom::euclid::Vector2D;
   use std::f64;
   use std::marker::PhantomData;
   use lyon_geom::math::Angle;

   pub struct LaserPoint {
      pub(crate) on: bool,
      pub(crate) x: f64,
      pub(crate) y: f64,
   }

   pub fn generate2dtrace(path: &Vec<PathSegment>,
                          max_dev: &f64,
                          max_vel: &f64,
                          max_acc: &f64,
                          update_rate: &f64
   ) -> Vec<LaserPoint> {
      let max_displacement: f64 = max_vel / update_rate;
      let mut points: Vec<LaserPoint> = Vec::new();
      let mut starting_point: LaserPoint = LaserPoint {
         on: false,
         x: 10.0,
         y: 10.0,
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
                  //end_x +=last_x;
                  //end_y +=last_y;
               }
               last_path_start_point.x = end_x;
               last_path_start_point.y = end_y;
               result = get_straight_travel(last_x, last_y, end_x, end_y, max_displacement, false);

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
            _ => {}
         }
         points.append(&mut result);
         println!("{:?}", path_seg);
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
         println!("{}, {}, {}", xloc, yloc, step_length);
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
         println!("{}, {}, {}", sample_point.x, sample_point.y, step_length);
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
         println!("{}, {}, {}", sample_point.x, sample_point.y, step_length);
      }
      return besier_points;
   }
}

