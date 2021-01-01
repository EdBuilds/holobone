use std::sync::{Arc, Mutex};
extern crate sdl2;
use sdl2::pixels::Color;
use sdl2::Error;
use std::thread;
use std::collections::VecDeque;
use pru_control::{CommandRegPair, Frequencies};
use pru_control::CommandReg;
use std::sync::mpsc::Sender;
use std::time::Duration;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread::JoinHandle;

pub const VIS_DEBUG_DBUF_CAPACITY: usize = 100;
pub struct VisDebug {
    update_handle: Option<thread::JoinHandle<()>>,
    rolling_buffer: Arc<Mutex<VecDeque<CommandRegPair>>>,
    kill_switch: Arc<AtomicBool>,
}

impl VisDebug {

    pub fn new(frequency: &Frequencies, buffer_size: usize, buffer_empty_sender: Sender<u32>) -> Result<VisDebug, Error > {
        let rolling_buffer: Arc<Mutex<VecDeque<CommandRegPair>>> = Arc::new(Mutex::new(VecDeque::new()));
        let local_rolling_buffer = rolling_buffer.clone();
        let mut delay = Duration::from_millis( match frequency {
            Frequencies::Hz1 => 1000,
            Frequencies::Hz10 => 100,
            Frequencies::Hz100 => 10,
            Frequencies::Hz1000 => 10,
            _ => 1,
        });
        let kill_switch = Arc::new(AtomicBool::new(false));
        let local_kill_switch = kill_switch.clone();
        let update_handle = Some(thread::spawn(move || {
            const _ON1_COLOR: Color = Color::RGB(0xFE, 0x80, 0x19);
            const _OFF1_COLOR: Color = Color::RGB(0xB8, 0xBB, 0x26);
            const ON2_COLOR: Color = Color::RGB(0xB1, 0x62, 0x86);
            const OFF2_COLOR: Color = Color::RGB(0x45, 0x85, 0x88);
            const _SCREEN_PERSISTENCE: u64 = 50;
            delay /= buffer_size as u32;
            let sdl_context = sdl2::init().expect("Ding");
            let video_subsystem = sdl_context.video().expect("Ding");
            let window = video_subsystem.window("Holobone visual debugger", 1024, 1024)
                .position_centered()
                .opengl()
                .build()
                .map_err(|e| e.to_string()).expect("Ding");
            let mut canvas = window.into_canvas().build().map_err(|e| e.to_string()).expect("Ding");

            loop {
                if local_kill_switch.load(Ordering::Relaxed){
                    break;
                }
                {
                    canvas.set_draw_color(Color::RGB(28, 28, 28));
                    canvas.clear();
                    match local_rolling_buffer.lock(){
                        Ok(mut roll_buf) => {
                            (*roll_buf).iter().for_each(|command_pair| {
                                if command_pair.channel_a.contains(CommandReg::LASER_ENABLE) {
                                    canvas.set_draw_color(ON2_COLOR);
                                } else {
                                    canvas.set_draw_color(OFF2_COLOR);
                                }
                                let x = command_pair.channel_a.data() as i32 / 4;
                                let y = command_pair.channel_b.data() as i32 / 4;
                                canvas.draw_point(sdl2::rect::Point::new(x, y)).expect("Dong")
                            });
                            (*roll_buf).clear();
                        }
                        _ => {break;}
                    }
                    canvas.present();
                }
                if let Err(_err) = buffer_empty_sender.send(0) {
                    break;
                }
                std::thread::sleep(Duration::from_millis(20));
            }
        }));

        Ok(VisDebug {
            update_handle,
            rolling_buffer,
            kill_switch,
        })
    }
    pub fn display_buffer(&self, data: Vec<CommandRegPair>) {
        let buffer = self.rolling_buffer.clone();
        (*buffer.lock().unwrap()).append(&mut VecDeque::from(data));
    }
    fn kill_thread(&mut self) {
        self.kill_switch.store(true, Ordering::Relaxed);
    }

}
impl Drop for VisDebug {
    fn drop(&mut self) {
        self.kill_thread();
        self.update_handle.take().map(JoinHandle::join);
    }
}
