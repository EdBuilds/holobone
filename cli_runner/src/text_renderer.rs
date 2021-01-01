use crate::font_manager::LineFont;
use crate::renderer::RenderingError;
use lyon::algorithms::aabb::fast_bounding_rect;
use lyon::algorithms::fit::{fit_path, FitStyle};
use lyon::math::Translation;
use lyon::math::{Rect, Transform};
use lyon_path::Path;
use std::borrow::Borrow;
use std::collections::HashMap;
use std::marker::PhantomData;

pub type TextFit = FitStyle;
pub enum TextAlignment {
    Left,
    Center,
    Right,
}

pub struct TextFormatter<'a> {
    pub fit_style: TextFit,
    pub alignment: TextAlignment,
    pub font: &'a str,
    pub font_face: &'a str,
}

impl Default for TextFormatter<'_> {
    fn default() -> Self {
        TextFormatter {
            fit_style: FitStyle::Min,
            alignment: TextAlignment::Center,
            font: "EMSOsmotron",
            font_face: "EMS Osmotron",
        }
    }
}

impl From<()> for TextFormatter<'_> {
    fn from(_: ()) -> Self {
        Self::default()
    }
}

impl From<TextAlignment> for TextFormatter<'_> {
    fn from(alignment: TextAlignment) -> Self {
        let mut return_formatter = Self::default();
        return_formatter.alignment = alignment;
        return_formatter
    }
}

pub struct TextRenderer {
    fonts: HashMap<String, LineFont>,
}
struct TextLine {
    path: Path,
    right_border: f32,
}
impl TextLine {
    pub fn new() -> TextLine {
        TextLine {
            path: Path::new(),
            right_border: 0.0,
        }
    }
}
impl TextRenderer {
    pub fn new() -> TextRenderer {
        TextRenderer {
            fonts: HashMap::new(),
        }
    }
    pub fn print<'a, FormatBuilder>(
        &self,
        text: &str,
        bounding_box: &Rect,
        format_option: FormatBuilder,
    ) -> Result<Path, RenderingError>
    where
        FormatBuilder: Into<TextFormatter<'a>>,
    {
        let format = format_option.into();
        let used_font = self
            .fonts
            .get(format.font)
            .ok_or(RenderingError::ArgumentError)?;
        let used_font_face = used_font
            .font_faces
            .get(format.font_face)
            .ok_or(RenderingError::ArgumentError)?;
        let mut return_path = Path::new();
        let mut y_offset = 0.0 as f32;
        let mut line_buffer: Vec<TextLine> = vec![TextLine::new()];
        // Building up the line buffer
        for c in text.chars() {
            match c {
                '\n' => {
                    y_offset -= used_font_face.ascent;
                    y_offset += used_font_face.descent;
                    line_buffer.push(TextLine::new());
                }
                _ => match used_font.glyphs.get(c.borrow()) {
                    Some(glyph) => {
                        let glyph_path = glyph.path.borrow();
                        let current_line =
                            line_buffer.last_mut().ok_or(RenderingError::DummyError)?;
                        let glyph_offset =
                            Transform::translation(current_line.right_border, y_offset)
                                .then_scale(1.0, -1.0);
                        current_line.path = current_line
                            .path
                            .merge(glyph_path.transformed(glyph_offset.borrow()).borrow());
                        current_line.right_border += glyph.horizontal_advancement;
                    }
                    _ => {
                        let current_line =
                            line_buffer.last_mut().ok_or(RenderingError::DummyError)?;
                        current_line.right_border += used_font.missing_glyph.horizontal_advancement;
                    }
                },
            }
        }

        // Find right limit
        let mut largest_right_boundary = 0.0 as f32;
        bounding_box.width();

        for line in &line_buffer {
            if largest_right_boundary < line.right_border {
                largest_right_boundary = line.right_border;
            }
        }
        // Set horizontal alignment
        for mut line in &mut line_buffer {
            match format.alignment {
                TextAlignment::Center => {
                    let bounding_box_center = bounding_box.min_x() + bounding_box.width() / 2.0;
                    let centering_offset = Translation {
                        x: bounding_box_center - line.right_border / 2.0,
                        y: 0.0,
                        _unit: PhantomData,
                    };
                    line.path = line.path.transformed(centering_offset.borrow());
                }
                TextAlignment::Right => {
                    let bounding_box_right = bounding_box.max_x();
                    let centering_offset = Translation {
                        x: bounding_box_right - line.right_border,
                        y: 0.0,
                        _unit: PhantomData,
                    };
                    line.path = line.path.transformed(centering_offset.borrow());
                }
                _ => {
                    // Left alignment is already used, no operation needed
                }
            }
            return_path = return_path.merge(line.path.borrow());
        }

        // scaling it to fit the box
        return_path = fit_path(&return_path, bounding_box, format.fit_style);
        // Realigning if the fit has misaligned it
        let misaligned_bounding_box = fast_bounding_rect(return_path.borrow().iter());
        let realigning_offset = match format.alignment {
            TextAlignment::Left => Translation {
                x: bounding_box.min_x() - misaligned_bounding_box.min_x(),
                y: 0.0,
                _unit: PhantomData,
            },
            TextAlignment::Center => Translation {
                x: bounding_box.center().x - misaligned_bounding_box.center().x,
                y: 0.0,
                _unit: PhantomData,
            },
            TextAlignment::Right => Translation {
                x: bounding_box.max_x() - misaligned_bounding_box.max_x(),
                y: 0.0,
                _unit: PhantomData,
            },
        };
        return_path = return_path.transformed(realigning_offset.borrow());
        Ok(return_path)
    }

    pub fn add_font(&mut self, font: LineFont) {
        let font_name = font.id.clone();
        self.fonts.insert(font_name, font);
    }
}
