extern crate pruif;
use std::{thread, time};
use pruif::{Sample};
use pru_control::{Frequencies};
fn main() {
    let mut capemgr = pruif::Cape::new().unwrap();
    capemgr.start(Frequencies::Hz1);
    let mut commands = Vec::with_capacity(10);
    for i in 0..100 {
        commands.push(Sample{
            voltage_x: i as f32 / 10.0,
            voltage_y: 0.0,
            laser_on: false
        });
    }
    capemgr.push_command(commands, true);
    thread::sleep(time::Duration::from_millis(100000));

}
