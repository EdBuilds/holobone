#[macro_use]
extern crate bitflags;
extern crate bitfield;
extern crate clap;
use clap::arg_enum;

pub const PRU_DBUF_CAPACITY: usize = 500;
arg_enum!{
    pub enum Frequencies {
        Hz1 = 50420496,
        Hz2 = 25210243,
        Hz5 = 10084090,
        Hz10 = 5042040,
        Hz20 = 2521014,
        Hz50 = 1008399,
        Hz100 = 504194,
        Hz200 = 252092,
        Hz500 = 100830,
        Hz1000 = 50409,
        Hz2000 = 25200,
        Hz5000 = 10073,
        Hz10000 = 5031,
        Hz20000 = 2511,
        Hz30000 = 1670,
        Hz40000 = 1250,
        Hz50000 = 998,
        Hz60000 = 830,
        Hz70000 = 710,
        Hz80000 = 620,
    }
}

bitflags! {
  pub struct PWMControlReg: u32{
    const ENABLE = 1 << 0;
    const RELOAD = 1 << 1;
  }
}

bitflags! {
  pub struct CommandReg: u16 {
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
impl CommandReg {
    pub fn data(&self) -> u16 {
        self.bits() & ((1 << 12) - 1) as u16
    }
}

pub struct SampleScaled {
    pub data_a: u16,
    pub data_b: u16,
    pub laser_on: bool,
    pub voltage_out: bool,
}

#[derive(Clone, Copy, Debug)]
pub struct CommandRegPair {
    pub channel_b: CommandReg,
    pub channel_a: CommandReg,
}

impl CommandRegPair {
    pub fn new(sample: SampleScaled) -> CommandRegPair {
        const DATA_MASK: u16 = (1 << 12) - 1;
        let mut channel_a = CommandReg {
            bits: sample.data_a & DATA_MASK,
        } | CommandReg::OUTPUT_GAIN_BAR
            | CommandReg::INPUT_BUFFER;
        let mut channel_b = CommandReg {
            bits: sample.data_b & DATA_MASK,
        } | CommandReg::OUTPUT_GAIN_BAR
            | CommandReg::INPUT_BUFFER
            | CommandReg::CHANNEL_B;
        channel_a.set(CommandReg::SHUTDOWN_BAR, sample.voltage_out);
        channel_b.set(CommandReg::SHUTDOWN_BAR, sample.voltage_out);
        CommandRegPair { channel_b, channel_a }
    }
}

#[derive(Clone, Copy)]
pub struct Ctrl {
    pub delay: u32,
    pub control: PWMControlReg,
    pub flag: u32,
    pub read_bank: u32,
}

