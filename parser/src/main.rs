use svgtypes::PathParser;
use svgtypes::PathSegment;
use clap::{Arg, App};
use xmlparser;
use std::fs;
use xmlparser::Token::Attribute;
mod tracer;


extern crate sdl2;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

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
        .arg(Arg::with_name("dist")
            .short("d")
            .long("distance")
            .takes_value(true)
            .help("projector distance from the surface"))
        .get_matches();


    let myfile = matches.value_of("file");
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
                                    let mut line_to_initial = PathSegment::LineTo { abs: true, x: 0.0, y: 0.0};
                                    for wrapped_path_segment in p {
                                        match wrapped_path_segment {
                                            Err(error) => {}
                                            Ok(path_segment) => {
                                                // If the segment is a closure, we replace it with
                                                // a LineTo command
                                                match path_segment {
                                                    PathSegment::MoveTo { abs, x, y } => {
                                                        println!("initial point detected! x:{}, y:{}", x, y);
                                                        line_to_initial = PathSegment::LineTo { abs, x, y };
                                                        paths.push(path_segment);
                                                    }
                                                    PathSegment::ClosePath { abs} => {
                                                        paths.push(path_segment);
                                                    }
                                                    _ => {
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
    println!("Starting visual debugging");
    let points1 = tracer::tracer::generate2dtrace(&paths, &10.0f64, &20000.0f64, &1000.0f64, &20000f64);
    let points2 = tracer::tracer::generate2dtrace(&paths, &10.0f64, &160000.0f64, &1000.0f64, &20000f64);
}
