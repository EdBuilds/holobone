use std::sync::{Arc, Mutex};
extern crate sdl2;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use sdl2::render::WindowCanvas;
use sdl2::Error;
use std::{thread, time};
use std::collections::VecDeque;
use pru_control::{CommandRegPair, Frequencies};
use pru_control::CommandReg;
use std::sync::mpsc::Sender;

pub struct VisDebug {
    update_handle: thread::JoinHandle<()>,
    rolling_buffer: Arc<Mutex<VecDeque<CommandRegPair>>>,
}

impl VisDebug {

    pub fn new(frequency: &Frequencies, buffer_size: usize, buffer_empty_sender: Sender<u32>) -> Result<VisDebug, Error > {
        let mut rolling_buffer: Arc<Mutex<VecDeque<CommandRegPair>>> = Arc::new(Mutex::new(VecDeque::new()));
        let mut local_rolling_buffer = rolling_buffer.clone();
        let mut delay = time::Duration::from_millis( match frequency {
            Frequencies::Hz1 => 1000,
            Frequencies::Hz10 => 100,
            Frequencies::Hz100 => 10,
            Frequencies::Hz1000 => 10,
            _ => 10,
        });
        let mut update_handle = thread::spawn(move || {
            const ON1_COLOR: Color = Color::RGB(0xFE, 0x80, 0x19);
            const OFF1_COLOR: Color = Color::RGB(0xB8, 0xBB, 0x26);
            const ON2_COLOR: Color = Color::RGB(0xB1, 0x62, 0x86);
            const OFF2_COLOR: Color = Color::RGB(0x45, 0x85, 0x88);
            const SCREEN_PERSISTENCE: u64 = 50;
            delay /= buffer_size as u32;
            println!("Starting visual debugger thread");
            let sdl_context = sdl2::init().expect("Ding");
            let video_subsystem = sdl_context.video().expect("Ding");
            let window = video_subsystem.window("Holobone visual debugger", 1024, 1024)
                .position_centered()
                .opengl()
                .build()
                .map_err(|e| e.to_string()).expect("Ding");
            let mut canvas = window.into_canvas().build().map_err(|e| e.to_string()).expect("Ding");

            loop {
                {
                    canvas.set_draw_color(Color::RGB(28, 28, 28));
                    canvas.clear();
                    (*local_rolling_buffer.lock().unwrap()).iter().for_each(|command_pair| {
                        if command_pair.channelA.contains(CommandReg::LASER_ENABLE) {
                            canvas.set_draw_color(ON2_COLOR);
                        } else {
                            canvas.set_draw_color(OFF2_COLOR);
                        }
                        let x = command_pair.channelA.data() as i32 / 4;
                        let y = command_pair.channelB.data() as i32 / 4;
                        canvas.draw_point(sdl2::rect::Point::new(x, y)).expect("Dong")
                    });
                    canvas.present();
                }
                (*local_rolling_buffer.lock().unwrap()).pop_front();
                if (*local_rolling_buffer.lock().unwrap()).len() < buffer_size {
                    buffer_empty_sender.send(0).unwrap();
                }
                thread::sleep(delay);
            }
        });

        Ok(VisDebug {
            update_handle,
            rolling_buffer,
        })
    }
    pub fn display_buffer(&self, data: Vec<CommandRegPair>) {
        let mut buffer = self.rolling_buffer.clone();
        (*buffer.lock().unwrap()).append(&mut VecDeque::from(data));
    }
}
