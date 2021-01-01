use crate::renderer::RenderingError;
extern crate serde;
extern crate serde_xml_rs;
extern crate serde_path_to_error;
use std::fs::File;
use std::io::BufReader;
use std::convert::{TryFrom, TryInto};
use lyon::lyon_svg::path_utils::build_path;
use lyon_path::builder::SvgPathBuilder;
use std::collections::HashMap;

#[derive(Deserialize,Debug)]
struct Glyph {
   unicode: char,
   #[serde(rename = "glyph-name")]
   glyph_name: String,
   #[serde(rename = "horiz-adv-x")]
   horizontal_advancement: String,
   d: Option<String>,
}

#[derive(Deserialize,Debug)]
struct MissingGlyph {
   #[serde(rename = "horiz-adv-x")]
   horizontal_advancement: String,
}

#[derive(Deserialize,Debug)]
struct FontFace {
   #[serde(rename = "font-family")]
   font_family: String,
   #[serde(rename = "units-per-em")]
   units_per_em: String,
   ascent: String,
   descent: String,
   #[serde(rename = "cap-height")]
   cap_height: String,
   #[serde(rename = "x-height")]
   x_height: String
}

#[derive(Deserialize,Debug)]
pub struct Font {
   id: String,
   #[serde(rename = "horiz-adv-x")]
   horizontal_advancement: String,
   #[serde(rename = "font-face")]
   font_faces: Vec<FontFace>,
   #[serde(rename = "missing-glyph")]
   missing_glyph: MissingGlyph,
   #[serde(rename = "glyph")]
   glyphs: Vec<Glyph>,
}

#[derive(Deserialize,Debug)]
struct Defs {
   #[serde(rename = "font")]
   font: Font,
}

#[derive(Deserialize,Debug)]
struct Metadata {
   #[serde(rename = "$value")]
   metastring: String,
}
#[derive(Deserialize,Debug)]
struct Toplevel {
   version: String,
   metadata: Metadata,
   defs: Defs,
}

#[derive(Clone)]
pub struct MissingLineGlyph {
   pub horizontal_advancement: f32,
}

#[derive(Clone)]
pub struct LineGlyph {
   pub horizontal_advancement: f32,
   pub unicode_char: char,
   pub name: String,
   pub path: lyon_path::Path,
}

impl TryFrom<Glyph> for LineGlyph {
   type Error = RenderingError;
   fn try_from(xml_glyph: Glyph) -> Result<Self, Self::Error> {
      let glyph_path = match xml_glyph.d {
         Some(svg_data) => { build_path(SvgPathBuilder::new(lyon_path::Path::builder()), svg_data.as_ref())?}
         _ => {lyon_path::Path::new()}
      };
      Ok(LineGlyph{
         horizontal_advancement: xml_glyph.horizontal_advancement.parse()?,
         unicode_char: xml_glyph.unicode,
         name: xml_glyph.glyph_name,
         path: glyph_path
      })
   }
}

#[derive(Clone)]
pub struct LineFontFace {
   font_family: String,
   units_per_em: f32,
   pub ascent: f32,
   pub descent: f32,
   cap_height: f32,
   x_height: f32
}

impl TryFrom<FontFace> for LineFontFace {
   type Error = RenderingError;
   fn try_from(xml_font_face: FontFace) -> Result<Self, Self::Error> {
      Ok(LineFontFace{
         font_family: xml_font_face.font_family,
         units_per_em: xml_font_face.units_per_em.parse()?,
         ascent: xml_font_face.ascent.parse()?,
         descent: xml_font_face.descent.parse()?,
         cap_height: xml_font_face.cap_height.parse()?,
         x_height: xml_font_face.x_height.parse()?
      })
   }
}

#[derive(Clone)]
pub struct LineFont {
   pub id: String,
   pub metadata: String,
   pub horizontal_advancement: f32,
   pub font_faces: HashMap<String, LineFontFace>,
   pub missing_glyph: MissingLineGlyph,
   pub glyphs: HashMap<char, LineGlyph>,

}

pub fn load_font(file_path: &std::path::Path) -> Result<LineFont, RenderingError> {

   let f = File::open(file_path)?;
   let r = BufReader::new(f);
   let parsing_result: Result<Toplevel, _> = serde_xml_rs::de::from_reader(r);

   match parsing_result{
     Ok(parsed_xml_font) => {
       let default_horiz_adv : f32 = parsed_xml_font.defs.font.horizontal_advancement.parse()?;
       let missing_glyph_horiz_adv : f32 = parsed_xml_font.defs.font.missing_glyph.horizontal_advancement.parse()?;
       let mut parsed_line_font = LineFont{
          id: parsed_xml_font.defs.font.id,
          metadata: parsed_xml_font.metadata.metastring,
          horizontal_advancement: default_horiz_adv,
          font_faces: HashMap::new(),
          missing_glyph: MissingLineGlyph { horizontal_advancement: missing_glyph_horiz_adv },
          glyphs: HashMap::new()
       };
       // TODO: Find a better way to iterate other than a full blown for loop
       for font_face in parsed_xml_font.defs.font.font_faces {
         let maybe_line_font_face: Result<LineFontFace, _> = font_face.try_into();
         if let Ok(line_font_face) = maybe_line_font_face {
             parsed_line_font.font_faces.insert(line_font_face.font_family.clone(), line_font_face);
         }
       }

       for glyph in parsed_xml_font.defs.font.glyphs {
           let maybe_line_glyph: Result<LineGlyph, _> = glyph.try_into();
           if let Ok(line_glyph) = maybe_line_glyph {
               parsed_line_font.glyphs.insert(line_glyph.unicode_char, line_glyph);
           }
       }

       Ok(parsed_line_font)
     }
     _ =>{Err(RenderingError::XmlParsingError)}
   }
}
