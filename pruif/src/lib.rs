extern crate bitflags;
extern crate bitfield;
//extern crate prusst;
use std::collections::VecDeque;
use std::time;

// TODO: this value could be stored in the eeprom
const DAC_MAX: u16 = (1<<12)-1;
const V_TO_DAC_CODE: f32 = DAC_MAX as f32 / 10.0;

use std::borrow::BorrowMut;
use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::thread::JoinHandle;
use std::sync::atomic::{AtomicBool, Ordering};
use pru_control::{SampleScaled, CommandRegPair, Frequencies};
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
use visual_debugger::{VisDebug, VIS_DEBUG_DBUF_CAPACITY};
#[cfg(target_arch = "arm")]
use prusst::{Evtout, IntcConfig, MemSegment, Pruss, Sysevt};

#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    AlreadyInstantiated,
    PermissionDenied,
    DeviceNotFound,
    OtherDeviceError,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Sample {
    pub voltage_x: f32,
    pub voltage_y: f32,
    pub laser_on: bool,
}
fn scale_sample(sample: &Sample) -> SampleScaled {
    let data_a = ((-sample.voltage_x * V_TO_DAC_CODE) + DAC_MAX as f32 / 2.0) as u16;
    let data_b = ((-sample.voltage_y * V_TO_DAC_CODE) + DAC_MAX as f32 / 2.0) as u16;
    // TODO: Finish scaling
    SampleScaled {
        data_a,
        data_b,
        laser_on: sample.laser_on,
        voltage_out: true
    }
}
pub struct Cape {
    runner_handle: Option<thread::JoinHandle<()>>,
    rolling_buffer: Arc<Mutex<VecDeque<CommandRegPair>>>,
    staging_buffer: Arc<Mutex<Vec<CommandRegPair>>>,
    kill_switch: Arc<AtomicBool>,
}

impl Cape {
    pub fn new() -> Cape {
        Cape {
            runner_handle: None,
            rolling_buffer: Arc::new(Mutex::new(VecDeque::new())),
            staging_buffer: Arc::new(Mutex::new(Vec::new())),
            kill_switch: Arc::new(AtomicBool::new(false)),
        }
    }

    #[cfg(target_arch = "arm")]
    fn runner(frequency: Frequencies,
              rolling_buffer: Arc<Mutex<VecDeque<CommandRegPair>>>,
              staging_buffer: Arc<Mutex<Vec<CommandRegPair>>>,
              should_stop: Arc<AtomicBool>)
    {
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
            control: PWMControlReg::RELOAD,
            flag: 0,
            read_bank: 0,
        });
        let (mut dbuf1, mut dbuf1after) = pruss.dram0.split_at(0x0014);

        let mut databuffer1 = unsafe {
            pruss
                .dram0
                .alloc_uninitialized::<[CommandRegPair; PRU_DBUF_CAPACITY]>()
        };
        let mut databuffer2 = unsafe {
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
        ctrl.control.set(PWMControlReg::ENABLE, false);
        ctrl.delay = frequency as u32;
        println!("Delay loaded!");
        ctrl.control.set(PWMControlReg::RELOAD, true);
        println!("control loaded!");
        ctrl.control.set(PWMControlReg::ENABLE, true);
        println!("Start!");
        ctrl.control.set(PWMControlReg::RELOAD, false);


        loop {

            if should_stop.load(Ordering::Relaxed){
                break;
            }
            {
                let mut local_rolling_buffer = &mut (*rolling_buffer.lock().unwrap());
                while local_rolling_buffer.len() < PRU_DBUF_CAPACITY {
                    let mut local_staging_buffer = (*staging_buffer.lock().unwrap()).clone();
                    if local_staging_buffer.is_empty() { panic!("Uninitialized staging buffer!")}
                    for sample in local_staging_buffer {
                        local_rolling_buffer.push_back(sample);
                    }
                }
            }
            irq.wait();

            {
                let mut drained_data = Vec::new();
                let mut local_rolling_buffer = &mut (*rolling_buffer.lock().unwrap());
                for num in 1..=PRU_DBUF_CAPACITY {
                    match local_rolling_buffer.pop_front(){
                       Some(sample) => {drained_data.push(sample)}
                        _ => {break;}
                    }
                }
                if (ctrl.read_bank == 1)  {
                    databuffer1.copy_from_slice(&drained_data[..]);
                } else if (ctrl.read_bank == 2) {
                    databuffer2.copy_from_slice(&drained_data[..]);
                }
            }

            // Clear the triggering interrupt and re-enable the host irq.
            pruss.intc.clear_sysevt(Sysevt::S19);
            pruss.intc.enable_host(Evtout::E0);

        }
        // Stop pru execution

        ctrl.control.set(PWMControlReg::ENABLE, false);
    }

    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    fn runner(frequency: Frequencies,
                 rolling_buffer: Arc<Mutex<VecDeque<CommandRegPair>>>,
                 staging_buffer: Arc<Mutex<Vec<CommandRegPair>>>,
                 should_stop: Arc<AtomicBool>) {
        let (buffer_empty_sender, buffer_empty_receiver) = mpsc::channel();

        let visual_debugger = VisDebug::new(&frequency, VIS_DEBUG_DBUF_CAPACITY, buffer_empty_sender).unwrap();
        let _delay = time::Duration::from_millis( match frequency {
            Frequencies::Hz1 => 1000,
            Frequencies::Hz10 => 100,
            Frequencies::Hz100 => 10,
            _ => 10,
        });
       loop{
           if should_stop.load(Ordering::Relaxed){
               break;
           }
           {
           let local_rolling_buffer = &mut (*rolling_buffer.lock().unwrap());
           let local_staging_buffer = (*staging_buffer.lock().unwrap()).clone();
           if local_staging_buffer.is_empty() { panic!("Uninitialized staging buffer!")}
           local_rolling_buffer.append(VecDeque::from(local_staging_buffer).borrow_mut());
               //println!("Loaded staging to rolling buffer");
           }
           let _empty_buffer = buffer_empty_receiver.recv().unwrap();
           //thread::sleep(delay);
           //println!("Load request received");

           {
               let local_rolling_buffer = &mut (*rolling_buffer.lock().unwrap());
               let drained_data = local_rolling_buffer.drain(0..local_rolling_buffer.len()).collect::<Vec<CommandRegPair>>();
               visual_debugger.display_buffer(drained_data);
           }
       }

    }

    pub fn start(&mut self, frequency: Frequencies) {
        let rolling_buffer = self.rolling_buffer.clone();
        let staging_buffer = self.staging_buffer.clone();
        let kill_switch = self.kill_switch.clone();

        self.runner_handle = Some(thread::spawn(move || {
            Cape::runner(frequency, rolling_buffer, staging_buffer, kill_switch);
        }));
    }
    pub fn stop(&mut self) {
       self.kill_switch.store(true, Ordering::Relaxed);

    }
    pub fn push_command(&mut self, samples: Vec<Sample>, repeat: bool) {

        let command_reg_pairs:Vec<CommandRegPair>  = samples.iter().map(|sample|CommandRegPair::new(scale_sample(sample))).collect();
        let rolling_buffer = self.rolling_buffer.clone();
        let staging_buffer = self.staging_buffer.clone();
        (*(rolling_buffer.lock().unwrap())).append(VecDeque::from(command_reg_pairs.clone()).borrow_mut());
        if repeat {
            (*(staging_buffer.lock().unwrap())) = command_reg_pairs;
        }
    }
}

impl Default for Cape {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for Cape {
    fn drop(&mut self) {
        self.stop();
        self.runner_handle.take().map(JoinHandle::join);
    }
}
