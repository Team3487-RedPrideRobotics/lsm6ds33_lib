use rppal::i2c::{I2c};
use std::error::Error;

use log::{debug, info};

use byteorder::{BigEndian, ByteOrder, LittleEndian};

#[derive(Copy, Clone)]
pub enum Address {
    A = 0x6a,
    B = 0x6b,
}

impl std::fmt::Display for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Address::A => write!(f, "0x6A"),
            Address::B => write!(f, "0x6B")
        }
    }
}

pub struct Lsm6ds33 {
    addr: Address,
    bus: I2c,
    accel: xl::Accelerometer,
    gyro_active: bool,
}

impl Lsm6ds33 {
    pub fn new(mut bus: I2c, addr: Address) -> Result<Lsm6ds33, Box<dyn Error>>  {
        bus.set_slave_address(addr.clone() as u16)?;

        Ok(Lsm6ds33 {
            addr,
            bus,
            accel: xl::Accelerometer {
                active: false,
                scale: xl::scale::TWO,
            },
            gyro_active: false,
        })
    }

    pub fn start_xl(&mut self, rate: xl::DataRate, scale: xl::Scale, a_a: xl::AntiAlias) -> Result<(), Box<dyn Error>> {
        info!("Starting Accelerometer for {}", self.addr);

        let data = vec![rate as u8 | scale.0 | a_a as u8];
        self.bus.write(&data)?;
        self.accel.active = true;
        Ok(())
    }

    /// Returns a tuple (x, y, z) representing the 3D Vector
    pub fn get_data(&mut self) -> Result<Option<[f32; 3]>, Box<dyn Error>> {
        if self.accel.active == false {
            return Ok(None);
        }

        let mut t_data = [0.0, 0.0, 0.0];

        //For Each Axis, Get data
        for i in 0 .. 3 {
            let mut data = vec![0 as u8; 2];
            self.bus.write_read(&vec![xl::DataRegister::XL as u8 + (i*2) as u8], &mut data[0..1])?;
            self.bus.write_read(&vec![xl::DataRegister::XL as u8 + (i*2) as u8 + 1], &mut data[1..])?;

            let r_data =  LittleEndian::read_f32(&data);
            t_data[i] = r_data / self.accel.scale.1;
        }

        Ok(Some(t_data))

    }

    pub fn xl_stop (&mut self) -> Result<(), Box<dyn Error>> {

        info!("Turning the accelerometer off.");
        let off_setting = vec![xl::ControlRegister::Reg1 as u8, xl::DataRate::Off as u8];
        self.bus.write(&off_setting)?;

        Ok(())
    }
}

pub mod xl {
    pub struct Accelerometer {
        pub active: bool,
        pub scale: Scale,
    }

    pub enum ControlRegister {
        Reg1 = 0x10,
    }

    pub enum DataRegister {
        XL = 0x28,
        XH = 0x29,
        YL = 0x2A,
        YH = 0x2B,
        ZL = 0x2C,
        ZH = 0x2D,
    }

    /// These data rates are XL_HM_MODE = 1.
    /// If that register is 0, then high-performance mode is selected.
    pub enum DataRate {
        /// Accelerometer Turned Off
        Off = 0b0000,

        /// 12.5 Hz
        LowPower0 = 0b0001_0000,

        /// 26 Hz
        LowPower1 = 0b0010_0000,

        /// 52 Hz
        LowPower2 = 0b0011_0000,

        /// 104 Hz
        Normal0 = 0b0100_0000,

        /// 208 Hz
        Normal1 = 0b0101_0000,

        /// 416 Hz
        HiPf0 = 0b0110_0000,

        /// 833 Hz
        HiPf1 = 0b0111_0000,

        /// 1.66 kHz
        HiPf2 = 0b1000_0000,
        
        /// 3.33 kHz
        HiPf3 = 0b1001_0000,

        /// 6.66 kHz
        HiPf4 = 0b1010_0000,

    }

    pub type Scale = (u8, f32);

    pub mod scale {
        // +-2g
        pub const TWO: super::Scale = (0b0000_00_00, 0.061);
        // +- 16g
        pub const SIXTEEN: super::Scale = (0b0000_01_00, 0.488);
        // +- 4g
        pub const FOUR: super::Scale = (0b0000_10_00, 0.122);
        // +- 8g
        pub const EIGHT: super::Scale = (0b0000_11_00, 0.244);
    }

    pub enum AntiAlias {
        // 400Hz
        FourHundred = 0b0000_00_00,
        // 200Hz
        TwoHundred = 0b0000_00_01,
        // 100Hz 
        OneHundred = 0b0000_00_10,
        // 50Hz
        Fifty = 0b0000_00_11,
    }

}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
