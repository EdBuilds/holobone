mod tracer {
   use svgtypes::PathParser;
   use svgtypes::PathSegment;

   pub struct LaserPoint{
      on: bool,
      x: f64,
      y: f64,
   }
   pub fn generate2dtrace(path:&Vec<PathSegment>, max_dev:&f64, max_vel:&f64, max_acc:&f64) -> Vec<LaserPoint> {
      
   }
}

