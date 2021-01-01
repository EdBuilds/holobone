use crate::renderer::Renderer;
use crate::renderer::RenderingError;
use lyon::lyon_svg::path::builder::SvgPathBuilder;
use lyon_path::Path;
use svg::parser::Event;
use lyon::lyon_svg::path_utils::build_path;
use std::borrow::Borrow;
use lyon::math::{Rect, Point, Size};
use lyon::lyon_algorithms::fit::{fit_path, FitStyle};
use std::marker::PhantomData;

pub struct SvgRenderer {
    normalized_path: lyon_path::Path,
}
impl SvgRenderer {
    pub fn new(file_path: &str) -> Result<SvgRenderer, RenderingError> {

        let mut decoded_path: lyon_path::Path = lyon_path::Path::new();

            for event in svg::open(file_path)? {
                if let Event::Tag(_path, _, attributes) =  event {
                    if let Some(data) = attributes.get("d") {
                        let decoding_result = build_path(SvgPathBuilder::new(Path::builder()), data)?;
                        decoded_path = decoded_path.merge(decoding_result.borrow());
                    }
                }
            }
        let unit_rect = Rect{
            origin: Point { x: -1.0, y: -1.0, _unit: PhantomData },
            size: Size { width: 2.0, height: 2.0, _unit: PhantomData } };
        let normalized_path = fit_path(&decoded_path, &unit_rect, FitStyle::Min);
        Result::Ok(SvgRenderer { normalized_path })
    }
}

impl Renderer for SvgRenderer {
    fn update_display(&mut self) -> Result<Path, RenderingError>{

        Result::Ok(self.normalized_path.clone())
    }
}