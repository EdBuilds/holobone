////! Flash the BeagleBone USR1 LED 5 times at 2Hz.
////!
////! The PRU code notifies the host processor every time the LED is flashed by triggering Evtout0.
////! The 6-th event out is interpreted as a completion notification.
//
//extern crate prusst;
//#[macro_use]
//extern crate bitfield;
//use prusst::{Pruss, IntcConfig, Evtout, Sysevt};
//use bitfield::Bit;
//use bitfield::BitRange;
//use std::fs::File;
//use std::io::Write;
//use std::{thread, time};
//use std::collections::VecDeque;
//
//static LED_TRIGGER_PATH: &'static str = "/sys/class/leds/beaglebone:green:usr1/trigger";
//static LED_DEFAULT_TRIGGER: &'static str = "mmc0";
//#[repr(C)]
//#[derive(Copy, Clone)]
//struct Ctrl {
//    delay: u32,
//    control: u32,
//    flag: u32,
//    read_bank: u32,
//}
//fn echo(value: &str, path: &str) {
//    let mut file = File::create(path).unwrap();
//    file.write_all(value.as_bytes()).unwrap();
//}
//
//bitfield!{
//  pub struct CommandReg_t(u16);
//  impl Debug;
//  u16;
//  data, set_data: 11,0;
//  shutdown_bar, set_shutdown_bar: 12;
//  output_gain_bar, set_output_gain_bar: 13;
//  input_buffer, set_input_buffer: 14;
//  channel, set_channel: 15;
//}
//
//bitfield!{
//  pub struct PWMControlReg_t(u32);
//  impl Debug;
//  u32;
//  enable, set_enable: 0;
//  reload, set_reload: 0;
//}
//
//fn main() {
//    // Get a view of the PRU subsystem.
//    let mut CommandReg = CommandReg_t(0);
//    CommandReg.set_data(4096);
//    CommandReg.set_shutdown_bar(true);
//    CommandReg.set_channel(true);
//    let brut = CommandReg.channel();
//    print!("{:#016b}", CommandReg.0);
//
//    return;
//    let mut pruss = match Pruss::new(&IntcConfig::new_populated()) {
//        Ok(p) => p,
//        Err(e) => match e {
//            prusst::Error::AlreadyInstantiated
//                => panic!("You can't instantiate more than one `Pruss` object at a time."),
//            prusst::Error::PermissionDenied
//                => panic!("You do not have permission to access the PRU subsystem: \
//                           maybe you should run this program as root?"),
//            prusst::Error::DeviceNotFound
//                => panic!("The PRU subsystem could not be found: are you sure the `uio_pruss` \
//                           module is loaded and supported by your kernel?"),
//            prusst::Error::OtherDeviceError
//                => panic!("An unidentified problem occured with the PRU subsystem: \
//                           do you have a valid overlay loaded?")
//        }
//    };
//
//    // Get a handle to an event out.
//    let mut data: i32 = 1;
//    let (mut bank1, mut bank2) = pruss.dram2.split_at(0x10100);
//    let ctrl = bank1.alloc(Ctrl{delay: 0, control: 0, flag: 0, read_bank: 0});
//    let (mut dbuf1, mut dbuf1after) = pruss.dram0.split_at(0x0014);
//    let databank1 = unsafe { dbuf1.alloc_uninitialized::<[u16; 10]>() };
//    let databank1after = unsafe { dbuf1after.alloc_uninitialized::<[u16; 10]>() };
//    let databank2 = unsafe { pruss.dram1.alloc_uninitialized::<[u16; 10]>() };
//
//    // Open and load the PRU binary.
//    let mut pru_binary = File::open("/home/debian/PRUClock.bin").unwrap();
//    let mut pru0_binary = File::open("/home/debian/PRUDAC.bin").unwrap();
//    for (i, val) in databank1.iter_mut().enumerate() {
//        *val = i as u16;
//    }
//    for (i, val) in databank1after.iter_mut().enumerate() {
//        *val = 0 as u16;
//    }
//    for (i, val) in databank2.iter_mut().enumerate() {
//        *val = 0xFFFF as u16;
//    }
//
//    let irq = pruss.intc.register_irq(Evtout::E0);
//
//    // Run the PRU binary.
//    unsafe { pruss.pru0.load_code(&mut pru0_binary).unwrap().run(); }
//    unsafe { pruss.pru1.load_code(&mut pru_binary).unwrap().run(); }
//
//    ctrl.flag=0;
//    ctrl.control=0;
//    thread::sleep(time::Duration::from_secs(1));
//    ctrl.delay=400000000;
//    println!("Delay loaded!");
//    thread::sleep(time::Duration::from_secs(1));
//    ctrl.control=2;
//    println!("control loaded!");
//    thread::sleep(time::Duration::from_secs(1));
//    ctrl.control=1;
//    println!("Start!");
//
//    loop {
//        irq.wait();
//        println!("Bank {} read.", ctrl.read_bank);
//
//        // Clear the triggering interrupt and re-enable the host irq.
//        pruss.intc.clear_sysevt(Sysevt::S19);
//        pruss.intc.enable_host(Evtout::E0);
//    }
//
//    thread::sleep(time::Duration::from_secs(300));
//    println!("Goodbye!");
//}
