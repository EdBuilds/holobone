//! Flash the BeagleBone USR1 LED 5 times at 2Hz.
//!
//! The PRU code notifies the host processor every time the LED is flashed by triggering Evtout0.
//! The 6-th event out is interpreted as a completion notification. 

extern crate prusst;
use std::{thread, time};

use prusst::{Pruss, IntcConfig, Evtout, Sysevt};

use std::fs::File;
use std::io::Write;

static LED_TRIGGER_PATH: &'static str = "/sys/class/leds/beaglebone:green:usr1/trigger";
static LED_DEFAULT_TRIGGER: &'static str = "mmc0";

#[repr(C)]
#[derive(Copy, Clone)]
struct Ctrl {
    delay: u32,
    control: u32,
    flag: u32,
}
fn echo(value: &str, path: &str) {
    let mut file = File::create(path).unwrap();
    file.write_all(value.as_bytes()).unwrap();
}


fn main() {
    // Get a view of the PRU subsystem.
    let mut pruss = match Pruss::new(&IntcConfig::new_populated()) {
        Ok(p) => p,
        Err(e) => match e {
            prusst::Error::AlreadyInstantiated
                => panic!("You can't instantiate more than one `Pruss` object at a time."),
            prusst::Error::PermissionDenied
                => panic!("You do not have permission to access the PRU subsystem: \
                           maybe you should run this program as root?"),
            prusst::Error::DeviceNotFound
                => panic!("The PRU subsystem could not be found: are you sure the `uio_pruss` \
                           module is loaded and supported by your kernel?"),
            prusst::Error::OtherDeviceError
                => panic!("An unidentified problem occured with the PRU subsystem: \
                           do you have a valid overlay loaded?")
        }
    };
    
    // Get a handle to an event out.
    let mut data: i32 = 1;
    let (mut bank1, mut bank2) = pruss.dram2.split_at(0x10100);
    let ctrl = bank1.alloc(Ctrl{delay: 0, control: 0, flag: 0});

    // Open and load the PRU binary.
    let mut pru_binary = File::open("/home/debian/PRUClock.bin").unwrap();
    let mut pru0_binary = File::open("/home/debian/PRUDAC.bin").unwrap();

    // Temporarily take control of the LED.
    echo("none", LED_TRIGGER_PATH);

    // Run the PRU binary.
    unsafe { pruss.pru0.load_code(&mut pru0_binary).unwrap().run(); }
    //unsafe { pruss.pru1.load_code(&mut pru_binary).unwrap().run(); }

    ctrl.flag=0;
    ctrl.control=0;
    thread::sleep(time::Duration::from_secs(1));
    ctrl.delay=20000000;
    println!("Delay loaded!");
    thread::sleep(time::Duration::from_secs(1));
    ctrl.control=2;
    println!("control loaded!");
    thread::sleep(time::Duration::from_secs(1));
    ctrl.control=1;
    println!("Start!");
    thread::sleep(time::Duration::from_secs(300));
    println!("Goodbye!");
}
