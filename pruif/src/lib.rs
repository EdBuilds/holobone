#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate bitfield;
extern crate prusst;
use std::collections::VecDeque;
use std::result;
use std::vec;

const PRU_DBUF_CAPACITY: usize = 10;

pub enum Frequencies {
    Hz1 = 20000000,
}

bitfield! {
  #[derive(Clone, Copy)]
  struct PWMControlReg_t(u32);
  impl Debug;
  impl Copy;
  u32;
  enable, set_enable: 0;
  reload, set_reload: 1;
}

bitflags! {
  struct CommandReg: u16 {
  const SHUTDOWN_BAR = 1 << 12;
  const LASER_ENABLE = 1 << 12;
  const OUTPUT_GAIN_BAR = 1 << 13;
  const INPUT_BUFFER = 1 << 14;
  const CHANNEL_A_BAR = 1 << 15;
  const CHANNEL_B = 1 << 15;
  }
}
impl Default for CommandReg {
    fn default() -> CommandReg {
        CommandReg::SHUTDOWN_BAR | CommandReg::OUTPUT_GAIN_BAR | CommandReg::INPUT_BUFFER
    }
}

struct SampleScaled {
    data_a: u16,
    data_b: u16,
    laser_on: bool,
    voltage_out: bool,
}

#[derive(Clone, Copy)]
struct CommandRegPair {
    channelA: CommandReg,
    channelB: CommandReg,
}
impl CommandRegPair {
    pub fn new(sample: SampleScaled) -> CommandRegPair {
        const DATA_MASK: u16 = (1 << 12) - 1;
        let mut channelA = CommandReg {
            bits: sample.data_a & DATA_MASK,
        } | CommandReg::OUTPUT_GAIN_BAR
            | CommandReg::INPUT_BUFFER;
        let mut channelB = CommandReg {
            bits: sample.data_b & DATA_MASK,
        } | CommandReg::OUTPUT_GAIN_BAR
            | CommandReg::INPUT_BUFFER
            | CommandReg::CHANNEL_B;
        channelA.set(CommandReg::LASER_ENABLE, sample.laser_on);
        channelB.set(CommandReg::SHUTDOWN_BAR, sample.voltage_out);
        CommandRegPair { channelA, channelB }
    }
}

#[derive(Clone, Copy)]
struct Ctrl {
    delay: u32,
    control: PWMControlReg_t,
    flag: u32,
    read_bank: u32,
}

use prusst::{Evtout, IntcConfig, MemSegment, Pruss, Sysevt};

use std::borrow::BorrowMut;
use std::fs::File;
use std::io::Write;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;
use std::iter::FromIterator;
use std::collections::vec_deque::Iter;

#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    AlreadyInstantiated,
    PermissionDenied,
    DeviceNotFound,
    OtherDeviceError,
}

pub struct Sample {
    pub voltage_x: f32,
    pub voltage_y: f32,
    pub laser_on: bool,
}
fn scale_sample(sample: &Sample) -> SampleScaled {
    // TODO: Finish scaling
    SampleScaled {
        data_a: 0,
        data_b: 0,
        laser_on: sample.laser_on,
        voltage_out: true
    }
}
pub struct Cape {
    member: u32,
    runner_handle: Option<thread::JoinHandle<()>>,
    rolling_buffer: Arc<Mutex<VecDeque<CommandRegPair>>>,
    staging_buffer: Arc<Mutex<Vec<CommandRegPair>>>,
}

impl Cape {
    pub fn new() -> Result<Cape, prusst::Error> {
        println!("{:?}", std::mem::size_of::<CommandRegPair>());
        Ok(Cape {
            member: 1,
            runner_handle: None,
            rolling_buffer: Arc::new(Mutex::new(VecDeque::new())),
            staging_buffer: Arc::new(Mutex::new(Vec::new())),
        })
    }
    fn runner() {
        println!("Child thread started!");
        let mut pruss = match Pruss::new(&IntcConfig::new_populated()) {
            Ok(p) => p,
            Err(e) => match e {
                prusst::Error::AlreadyInstantiated => {
                    panic!("You can't instantiate more than one `Pruss` object at a time.")
                }
                prusst::Error::PermissionDenied => panic!(
                    "You do not have permission to access the PRU subsystem: \
                         maybe you should run this program as root?"
                ),
                prusst::Error::DeviceNotFound => panic!(
                    "The PRU subsystem could not be found: are you sure the `uio_pruss` \
                         module is loaded and supported by your kernel?"
                ),
                prusst::Error::OtherDeviceError => panic!(
                    "An unidentified problem occured with the PRU subsystem: \
                         do you have a valid overlay loaded?"
                ),
            },
        };

        // Get a handle to an event out.
        let (mut bank1, mut bank2) = pruss.dram2.split_at(0x10100);
        let ctrl = bank1.alloc(Ctrl {
            delay: 0,
            control: PWMControlReg_t { 0: 0 },
            flag: 0,
            read_bank: 0,
        });
        let (mut dbuf1, mut dbuf1after) = pruss.dram0.split_at(0x0014);

        let databuffer1 = unsafe {
            pruss
                .dram0
                .alloc_uninitialized::<[CommandRegPair; PRU_DBUF_CAPACITY]>()
        };
        let databuffer2 = unsafe {
            pruss
                .dram1
                .alloc_uninitialized::<[CommandRegPair; PRU_DBUF_CAPACITY]>()
        };

        // Open and load the PRU binary.
        let mut pru1_binary = File::open("/home/debian/PRUClock.bin").unwrap();
        let mut pru0_binary = File::open("/home/debian/PRUDAC.bin").unwrap();

        // Register interrupt
        let irq = pruss.intc.register_irq(Evtout::E0);

        // Run the PRU binary.
        unsafe {
            pruss.pru0.load_code(&mut pru0_binary).unwrap().run();
        }
        unsafe {
            pruss.pru1.load_code(&mut pru1_binary).unwrap().run();
        }

        ctrl.flag = 0;
        ctrl.control.set_enable(false);
        ctrl.delay = frequency as u32;
        println!("Delay loaded!");
        ctrl.control.set_reload(true);
        println!("control loaded!");
        ctrl.control.set_enable(true);
        println!("Start!");

        loop {
            irq.wait();
            println!("Bank {} read.", ctrl.read_bank);

            // Clear the triggering interrupt and re-enable the host irq.
            pruss.intc.clear_sysevt(Sysevt::S19);
            pruss.intc.enable_host(Evtout::E0);
        }
    }
    fn runner_pc(rolling_buffer: Arc<Mutex<VecDeque<CommandRegPair>>>, staging_buffer: Arc<Mutex<Vec<CommandRegPair>>>, ) {

    }

    pub fn start(&mut self, frequency: Frequencies) {
        let mut rolling_buffer = self.rolling_buffer.clone();
        let mut staging_buffer = self.staging_buffer.clone();
        self.runner_handle = Some(thread::spawn(move || {
        }));
    }
    pub fn push_command(&mut self, samples: Vec<Sample>, repeat: bool) {

        let command_reg_pairs:Vec<CommandRegPair>  = samples.iter().map(|sample|CommandRegPair::new(scale_sample(sample))).collect();
        let mut rolling_buffer = self.rolling_buffer.clone();
        let mut staging_buffer = self.staging_buffer.clone();
        (*(rolling_buffer.lock().unwrap())).append(VecDeque::from(command_reg_pairs.clone()).borrow_mut());
        if repeat {

            (*(staging_buffer.lock().unwrap())) = command_reg_pairs.to_owned();
        }
    }
}
impl Drop for Cape {
    fn drop(&mut self) {
        self.runner_handle.take().map(JoinHandle::join);
        println!("Thread finished!");
    }
}
#[cfg(test)]
mod tests {

    #[test]
    fn fail() {}
}
