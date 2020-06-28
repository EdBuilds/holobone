extern crate pruif;
use std::{thread, time};
use pruif::{Sample};
fn main() {
    let mut capemgr = pruif::Cape::new().unwrap();
    capemgr.start(pruif::Frequencies::Hz1);
    capemgr.push_command(vec![Sample{
        voltage_x: 0.0,
        voltage_y: 0.0,
        laser_on: false
    }], true);
    thread::sleep(time::Duration::from_millis(10));

}
