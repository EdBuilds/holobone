#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate bitfield;
extern crate prusst;
use std::collections::VecDeque;
use std::{result, time};
use std::vec;
use visual_debugger::VisDebug;

// TODO: this value could be stored in the eeprom
const DAC_MAX: u16 = ((1<<12)-1);
const V_TO_DAC_CODE: f32 = (DAC_MAX as f32 / 10.0);
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
use std::sync::atomic::{AtomicBool, Ordering};
use pru_control::{SampleScaled, CommandRegPair, Frequencies, Ctrl, PWMControlReg_t, PRU_DBUF_CAPACITY};

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
    let max = DAC_MAX;
   let data_a = (sample.voltage_x * V_TO_DAC_CODE) as u16 + DAC_MAX / 2 ;
    let data_b = (sample.voltage_y * V_TO_DAC_CODE) as u16 + DAC_MAX / 2 ;
    // TODO: Finish scaling
    SampleScaled {
        data_a,
        data_b,
        laser_on: sample.laser_on,
        voltage_out: true
    }
}
pub struct Cape {
    member: u32,
    runner_handle: Option<thread::JoinHandle<()>>,
    rolling_buffer: Arc<Mutex<VecDeque<CommandRegPair>>>,
    staging_buffer: Arc<Mutex<Vec<CommandRegPair>>>,
    kill_switch: Arc<AtomicBool>,
}

impl Cape {
    pub fn new() -> Result<Cape, prusst::Error> {
        println!("{:?}", std::mem::size_of::<CommandRegPair>());
        Ok(Cape {
            member: 1,
            runner_handle: None,
            rolling_buffer: Arc::new(Mutex::new(VecDeque::new())),
            staging_buffer: Arc::new(Mutex::new(Vec::new())),
            kill_switch: Arc::new(AtomicBool::new(false)),
        })
    }

    fn runner(frequency: Frequencies) {
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

        //ctrl.flag = 0;
        //ctrl.control.set_enable(false);
        //ctrl.delay = frequency as u32;
        println!("Delay loaded!");
        //ctrl.control.set_reload(true);
        println!("control loaded!");
        //ctrl.control.set_enable(true);
        println!("Start!");

        loop {
            irq.wait();
            //println!("Bank {} read.", ctrl.read_bank);

            // Clear the triggering interrupt and re-enable the host irq.
            pruss.intc.clear_sysevt(Sysevt::S19);
            pruss.intc.enable_host(Evtout::E0);

        }
    }
    fn runner_pc(frequency: Frequencies,
                 rolling_buffer: Arc<Mutex<VecDeque<CommandRegPair>>>,
                 staging_buffer: Arc<Mutex<Vec<CommandRegPair>>>,
                 should_stop: Arc<AtomicBool>) {
        println!("Child thread started!");
        let visual_debugger = VisDebug::new().unwrap();
        let delay = match frequency {
            Hz1       => time::Duration::from_millis(1000),
        };
       loop{
           if should_stop.load(Ordering::Relaxed){
               break;
           }
           {
           let mut local_rolling_buffer = &mut (*rolling_buffer.lock().unwrap());
           while local_rolling_buffer.len() < PRU_DBUF_CAPACITY {
               let mut local_staging_buffer = (*staging_buffer.lock().unwrap()).clone();
               if local_staging_buffer.is_empty() { panic!("Uninitialized staging buffer!")}
               local_rolling_buffer.append(VecDeque::from(local_staging_buffer).borrow_mut());
               println!("Loaded staging to rolling buffer");
           }
           }
           thread::sleep(delay);
           println!("Load request received");

           {
               let mut local_rolling_buffer = &mut (*rolling_buffer.lock().unwrap());
               let mut drained_data = local_rolling_buffer.drain(0..PRU_DBUF_CAPACITY).collect::<Vec<CommandRegPair>>();
               println!("drained data: {:#?}",drained_data);
               visual_debugger.display_buffer(drained_data);
           }
       }

        println!("Child thread wrapping up!");
    }

    pub fn start(&mut self, frequency: Frequencies) {
        let mut rolling_buffer = self.rolling_buffer.clone();
        let mut staging_buffer = self.staging_buffer.clone();
        let mut kill_switch = self.kill_switch.clone();

        self.runner_handle = Some(thread::spawn(move || {
            Cape::runner_pc(frequency, rolling_buffer, staging_buffer, kill_switch);
        }));
    }
    pub fn stop(&mut self) {
       self.kill_switch.store(true, Ordering::Relaxed);

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
        self.stop();
        self.runner_handle.take().map(JoinHandle::join);
    }
}
