use crate::renderer::Renderer;
use crate::renderer::RenderingError;
use lyon::lyon_svg::path::builder::SvgPathBuilder;
use lyon_path::Path;
use svg::parser::Event;
use lyon::lyon_svg::path_utils::build_path;
use std::borrow::Borrow;
use crate::renderer::RenderingError::SvgParsingError;
use lyon::math::{Rect, Point, Size};
use lyon::lyon_algorithms::fit::{fit_path, FitStyle};
use std::marker::PhantomData;

pub struct LaserPoint {
    pub on: bool,
    pub x: f64,
    pub y: f64,
}
pub struct SvgRenderer {
    normalized_path: lyon_path::Path,
}
impl SvgRenderer {
    pub fn new(file_path: &str) -> Result<SvgRenderer, RenderingError> {

        let mut svg_path_builder = SvgPathBuilder::new(Path::builder());
        let mut decoded_path: lyon_path::Path = lyon_path::Path::new();

                for event in svg::open(file_path)? {
                    match event {
                        Event::Tag(Path, _, attributes) => {
                            match attributes.get("d") {
                                Some(data) =>{
                                    let mut decoding_result = build_path(SvgPathBuilder::new(Path::builder()), data)?;
                                    decoded_path = decoded_path.merge(decoding_result.borrow());
                                }
                                _ => {}
                            }

                        }
                        _ => {}
                    }
                }
        let unit_rect = Rect{
            origin: Point { x: 0.0, y: 0.0, _unit: PhantomData },
            size: Size { width: 1.0, height: 1.0, _unit: PhantomData } };
        let mut normalized_path = fit_path(&decoded_path, &unit_rect, FitStyle::Stretch);
        Result::Ok(SvgRenderer { normalized_path })
    }
}

impl Renderer for SvgRenderer {
    fn update_display(&mut self) -> Result<Path, RenderingError>{

        Result::Ok(self.normalized_path.clone())
    }
}