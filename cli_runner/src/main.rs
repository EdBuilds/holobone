use svgtypes::PathParser;
use svgtypes::PathSegment;
use clap::{Arg, App, value_t_or_exit};
use xmlparser;
use std::fs;
use tracer::{generate_trace_from_segments, LaserPoint};
use std::{thread, time};
use std::time::Duration;
use pruif::{Sample};
use pru_control::{Frequencies};
use pruif::{Cape};

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
        .arg(Arg::with_name("num")
            .short("n")
            .long("number")
            .takes_value(true)
            .help("Five less than your favorite number"))
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
        .get_matches();


    let myfile = matches.value_of("file");
    let freq = value_t_or_exit!(matches.value_of("Frequencies"), Frequencies);
    let mut paths:Vec<PathSegment> = Vec::new();
    match myfile {
        None => println!("No input file specified. Exiting now."),
        Some(s) => {
            let data = fs::read_to_string(s).expect("Unable to read file.");
            for wrapped_token in xmlparser::Tokenizer::from(&data[..]) {
                match wrapped_token {
                    Err(error) => {}
                    Ok(token) => {
                        match token {
                            xmlparser::Token::Attribute {prefix, local, value, span} => {
                                if local.as_str().eq("d") {
                                    let p = PathParser::from(value.as_ref());
                                    let mut line_to_initial = PathSegment::LineTo { abs: false, x: 0.0, y: 0.0};
                                    for wrapped_path_segment in p {
                                        match wrapped_path_segment {
                                            Err(error) => {}
                                            Ok(path_segment) => {
                                                // If the segment is a closure, we replace it with
                                                // a LineTo command
                                                match path_segment {
                                                    _ => {
                                                        println!("{:?}", path_segment);
                                                        paths.push(path_segment);
                                                    }
                                                }
                                            }
                                        }
                                    }

                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
            //let p = PathParser::from(&data[..]);
            //for token in p {
            //    println!("{:?}", token);
            //}
        }
    }
    let mut points = generate_trace_from_segments(&paths, &10.0f64, &100.0f64, &10000.0f64, &20000f64);
    let corners = find_corners(&points);

    println!("{:?}", corners);
    normalize(&mut points, corners);
    println!("{:?}", points);
    let samples = convert_to_sample(points);
    let mut capemgr = pruif::Cape::new().unwrap();
    capemgr.push_command(samples, true);
    capemgr.start(freq);
    thread::sleep(time::Duration::from_millis(10000000));
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
fn normalize(points: &mut Vec<LaserPoint>, corners: Corners) {
    let x_offset = (corners.left + corners.right) / 2.0;
    let y_offset = (corners.top + corners.bottom) / 2.0;
    let x_scale = (corners.right - corners.left) / 2.0;
    let y_scale = (corners.top - corners.bottom) / 2.0;
    for point in points.iter_mut(){
        point.x -= x_offset;
        point.y -= y_offset;
        point.x /= x_scale;
        point.y /= y_scale;
    }
}
fn convert_to_sample(points: Vec<LaserPoint>) -> Vec<Sample> {
    points.iter().map(|point|Sample{
        voltage_x: (point.x * 5f64) as f32,
        voltage_y: (point.y * 5f64) as f32,
        laser_on: true
    }).collect()
}
