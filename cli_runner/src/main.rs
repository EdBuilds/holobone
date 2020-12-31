use svgtypes::PathParser;
use svgtypes::PathSegment;

use svg::node::element::path::{Command, Data, Position};
use svg::node::element::tag::Path;
use svg::parser::Event;

#[macro_use]
extern crate serde_derive;

use clap::{Arg, App, value_t_or_exit, Values, SubCommand};
use xmlparser;
use std::fs;
use tracer::{generate_trace_from_segments, LaserPoint};
use std::{thread, time};
use std::time::Duration;
use pruif::{Sample, Cape};
use pru_control::{Frequencies};
extern crate ctrlc;
extern crate lyon;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use lyon_path::builder::*;
use lyon_path::{Path, Builder, PathEvent};
use lyon_path::math::{Point, Vector};
use lyon::algorithms::aabb::bounding_rect;
use lyon::algorithms::fit::fit_path;
use lyon::svg::path_utils::build_path;
use lyon::path::PathSlice;
use std::marker::PhantomData;
use std::borrow::{Borrow, BorrowMut};
use tracer::Tracer;
use lyon::lyon_tessellation::math::{Translation, Scale, Transform};
use lyon::lyon_algorithms::math::{Rect, size, Size};
use lyon::lyon_algorithms::fit::FitStyle;
use svgtypes::TransformListToken::Translate;

mod renderer;
mod svg_renderer;
mod asteroids_renderer;
mod asteroids_game;
mod font_manager;
mod text_renderer;
use crate::renderer::{Renderer, RenderingError};
use crate::svg_renderer::SvgRenderer;
use svg::node::NodeClone;
use crate::renderer::RenderingError::ArgumentError;
use crate::font_manager::load_font;
extern crate walkdir;
use walkdir::WalkDir;
use crate::text_renderer::TextAlignment;
use crate::asteroids_renderer::AsteroidsRenderer;

fn main() {

    let matches = App::new("Holobone")
        .version("0.1.0")
        .author("Tamas Feher <fehertamas11@gmail.com>")
        .about("Hologram interface software")
        .subcommand(SubCommand::with_name("svg").about("Draw the contents of an svg file")
            .arg(Arg::with_name("file")
                .short("f")
                .long("file")
                .takes_value(true)
                .required(true)
                .help("input SVG file"))
        )
        .subcommand(SubCommand::with_name("asteroids").about("play asteroids game"))

        .arg(Arg::with_name("file")
            .short("f")
            .long("file")
            .takes_value(true)
            .help("input SVG file"))
        .arg(Arg::with_name("vel")
            .short("v")
            .long("velocity")
            .takes_value(true)
            .help("Maximum allowable velocity in deg/s"))
        .arg(Arg::with_name("acc")
            .short("a")
            .long("acceleration")
            .takes_value(true)
            .help("Maximum allowable acceleration in deg/s/s"))
        .arg(Arg::with_name("dev")
            .short("e")
            .long("deviation")
            .takes_value(true)
            .help("Maximum allowable displacement in deg"))
        .arg(Arg::with_name("dist")
            .short("d")
            .long("distance")
            .takes_value(true)
            .help("projector distance from the surface"))

        .arg(Arg::with_name("scale")
            .short("s")
            .long("scale")
            .takes_value(true)
            .min_values(2)
            .help("scale of the projection"))

        .arg(Arg::with_name("offset")
            .short("o")
            .long("offset")
            .takes_value(true)
            .min_values(2)
            .help("offset of the projection"))

        .get_matches();


    let myfile = matches.value_of("file");
    let freq = Frequencies::Hz40000;
    let scale_arg = matches.values_of("scale");
    let mut user_scale = Size {
        width: 1.0,
        height: 1.0,
        _unit: PhantomData
    };

    let mut user_offset = Point {
        x: 0.0,
        y: 0.0,
        _unit: PhantomData
    };
    match scale_arg {
        None => {},
        Some(mut arg) => {
            user_scale.width =
                arg.next().unwrap_or_default().parse::<f32>().expect("Couldn't parse scale. Give me 2 numbers!");
            user_scale.height =
                arg.next().unwrap_or_default().parse::<f32>().expect("Couldn't parse scale. Give me 2 numbers!");
        }
    }


    match matches.values_of("scale") {
        None => {},
        Some(mut arg) => {
            user_offset.x =
                arg.next().unwrap_or_default().parse::<f32>().expect("Couldn't parse offset. Give me 2 numbers!");
            user_offset.y =
                arg.next().unwrap_or_default().parse::<f32>().expect("Couldn't parse offset. Give me 2 numbers!");
        }
    }
    // Populate text_renderer for global use
    let mut txt_renderer = text_renderer::TextRenderer::new();
    let maybe_paths = fs::read_dir("./resources/fonts");
    for e in WalkDir::new("./resources/fonts").into_iter().filter_map(|e| e.ok()) {
        if e.metadata().unwrap().is_file() {
            if let Ok(parsed_font) = load_font(e.path()){
                txt_renderer.add_font(parsed_font);
            }

        }
    }
    let mut renderer: Result<Box<dyn Renderer>, RenderingError> = Result::Err(RenderingError::DummyError);

    match matches.subcommand() {
        ("svg", Some(args)) => {
            // Now we have a reference to clone's matches
            renderer = match args.value_of("file") {
                None => Result::Err(RenderingError::ArgumentError),
                Some(path_string) => match SvgRenderer::new(path_string) {
                    Ok(initiated_renderer) => Result::Ok(Box::new(initiated_renderer)),
                    Err(initiation_error) => Result::Err(initiation_error),
                }
            }
        }
        ("asteroids", Some(args)) => {
            renderer = match AsteroidsRenderer::new(txt_renderer.borrow()) {
                Ok(initiated_renderer) => Result::Ok(Box::new(initiated_renderer)),
                Err(initiation_error) => Result::Err(initiation_error),
            }
        }
        _ => {}
    }

    match renderer {

        Err(error_message) => { eprintln!("Error initializing renderer: {}", error_message)}
        Ok(mut boxed_renderer) => {
            let initialized_renderer = boxed_renderer.as_mut();
            let running = Arc::new(AtomicBool::new(true));
            let r = running.clone();

            ctrlc::set_handler(move || {
                r.store(false, Ordering::SeqCst);
            }).expect("Error setting Ctrl-C handler");

            let tracer = Tracer::new(0.0, 0.0, 0.0);
            let mut capemgr = pruif::Cape::new();
            let mut first_run = true;
            while running.load(Ordering::SeqCst) {
                match initialized_renderer.update_display() {
                    Ok(path_to_display) => {
                        let samples = tracer.trace_path(path_to_display.as_slice());
                        capemgr.push_command(samples, true);
                    }
                    Err(_err) => {
                        running.store(false, Ordering::SeqCst);
                    }
                }
                if first_run {
                    capemgr.start(Frequencies::Hz40000);

                }
                first_run = false;
                std::thread::sleep_ms(33);
            }
            drop(capemgr);
        }
    }
}
