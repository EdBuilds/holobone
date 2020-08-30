use svgtypes::PathParser;
use svgtypes::PathSegment;

use svg::node::element::path::{Command, Data, Position};
use svg::node::element::tag::Path;
use svg::parser::Event;

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
use std::borrow::Borrow;
use tracer::Tracer;
use lyon::lyon_tessellation::math::{Translation, Scale, Transform};
use lyon::lyon_algorithms::math::{Rect, size, Size};
use lyon::lyon_algorithms::fit::FitStyle;
use svgtypes::TransformListToken::Translate;

mod renderer;
use crate::renderer::Renderer;

fn main() {
    let matches = App::new("Holobone")
        .version("0.1.0")
        .author("Tamas Feher <fehertamas11@gmail.com>")
        .about("Hologram interface software")


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
        .arg_from_usage("<Frequencies> 'Dac sampling frequency'")
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
    let freq = value_t_or_exit!(matches.value_of("Frequencies"), Frequencies);
    let scale_arg = matches.values_of("scale");
    let mut user_scale = Size{
        width: 1.0,
        height: 1.0,
        _unit: PhantomData
    };

    let mut user_offset = Point{
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

    let mut paths:Vec<PathSegment> = Vec::new();
    let mut svg_path_builder = SvgPathBuilder::new(Path::builder());
    let mut decoded_path = lyon_path::Path::new();
    match myfile {
        None => println!("No input file specified. Exiting now."),
        Some(s) =>{
            for event in svg::open(s).unwrap() {
                match event {
                    Event::Tag(Path, _, attributes) => {
                        let data = attributes.get("d").unwrap().as_ref();
                        let mut decoding_result = build_path(SvgPathBuilder::new(Path::builder()), data);
                        match  decoding_result{
                            Ok(path) => {decoded_path = decoded_path.merge(path.borrow());},
                            Err(_) => {panic!("ugh"); },
                        }
                    }
                    _ => {}
                }
            }
        }
    }
    let bounds = bounding_rect(decoded_path.iter());
    let bounds = bounding_rect(decoded_path.iter());
    let unit_rect = Rect{
        origin: Point { x: 0.0, y: 0.0, _unit: PhantomData },
        size: Size { width: 1.0, height: 1.0, _unit: PhantomData } };
    let mut normalized_path = fit_path(&decoded_path, &unit_rect, FitStyle::Stretch);
    normalized_path = normalized_path.transformed(&Transform::scale(user_scale.width, user_scale.height));
    normalized_path = normalized_path.transformed(&Translation{
        x: -user_offset.x,
        y: -user_offset.y,
        _unit: PhantomData
    });

    let tracer = Tracer::new(0.0, 0.0, 0.0);
    let mut points = tracer.trace_path(normalized_path.as_slice());
    println!("{:?}", points.len());

    let samples = convert_to_sample(points);
    let mut capemgr = pruif::Cape::new().unwrap();
    capemgr.push_command(samples, true);
    capemgr.start(freq);
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    }).expect("Error setting Ctrl-C handler");

    println!("Waiting for Ctrl-C...");
    while running.load(Ordering::SeqCst) {

    }
    drop(capemgr);
    println!("Got it! Exiting...");
}

#[derive(Debug)]
pub struct Corners {
    pub bottom: f64,
    pub left: f64,
    pub top: f64,
    pub right: f64,
}

fn find_corners(points: &Vec<LaserPoint>) -> Corners {
    points.iter().fold(Corners{bottom: std::f64::MAX, left: std::f64::MAX, top: std::f64::MIN, right: std::f64::MIN},
                       |mut corner, point| {
                           if point.x.lt(&corner.left) {
                               corner.left = point.x;
                           }
                           if point.x.gt(&corner.right) {
                               corner.right = point.x;
                           }
                           if point.y.lt(&corner.bottom) {
                               corner.bottom = point.y;
                           }
                           if point.y.gt(&corner.top) {
                               corner.top = point.y;
                           }
                           corner})
}
fn convert_to_sample(points: Vec<LaserPoint>) -> Vec<Sample> {
    points.iter().map(|point|Sample{
        voltage_x: ((point.x-0.5) * 10f64) as f32,
        voltage_y: ((point.y-0.5) * 10f64) as f32,
        laser_on: true
    }).collect()
}
