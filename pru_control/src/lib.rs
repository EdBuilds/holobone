#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate bitfield;

pub const PRU_DBUF_CAPACITY: usize = 10;
pub enum Frequencies {
    Hz1 = 20000000,
}

bitfield! {
  #[derive(Clone, Copy)]
  pub struct PWMControlReg_t(u32);
  impl Debug;
  impl Copy;
  u32;
   enable,  set_enable: 0;
   reload,  set_reload: 1;
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
    pub channelA: CommandReg,
    pub channelB: CommandReg,
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
pub struct Ctrl {
    pub delay: u32,
    pub control: PWMControlReg_t,
    pub flag: u32,
    pub read_bank: u32,
}

