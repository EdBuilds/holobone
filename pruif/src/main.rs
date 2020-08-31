extern crate pruif;
use std::{thread, time};
use pruif::{Sample};
use pru_control::{Frequencies};

fn main() {
    let mut capemgr = pruif::Cape::new();
    let mut commands = Vec::with_capacity(10);
    for i in 0..200 {
        let mut angle = i as f32 / 200.0 * std::f32::consts::PI * 2.0;
        commands.push(Sample{
            voltage_x: angle.sin()*2.0,
            voltage_y: angle.cos()*2.0,
            laser_on: true
        });
    }
    capemgr.push_command(commands, true);
    capemgr.start(Frequencies::Hz10000);
    thread::sleep(time::Duration::from_millis(10000000));
}
