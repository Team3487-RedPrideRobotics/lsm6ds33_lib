use rppal::i2c::{I2c};
use std::error::Error;

use log::{debug, info};

use byteorder::{BigEndian, ByteOrder, LittleEndian};

/// The only two possible addresses on the chip.
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

pub type Scale = (u8, f32);

mod xl {
    
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

    pub mod scale {
        use super::super::Scale;
        // +-2g
        pub const TWO: Scale = (0b0000_00_00, 0.061);
        // +- 16g
        pub const SIXTEEN: Scale = (0b0000_01_00, 0.488);
        // +- 4g
        pub const FOUR: Scale = (0b0000_10_00, 0.122);
        // +- 8g
        pub const EIGHT: Scale = (0b0000_11_00, 0.244);
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

mod gyro {
    pub enum ControlRegister {
        Reg1 = 0x11,
    }

    pub enum DataRate {
        /// Gyro Turned Off
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
    }

    pub enum DataRegister {
        XL = 0x22,
        XH = 0x23,
        YL = 0x24,
        YH = 0x25,
        ZL = 0x26,
        ZH = 0x27,
    }

    pub mod scale {
        use super::super::Scale;
        /// 125 dps
        pub const MIN: Scale = (0b0000_00_1_0, 0.061);
        /// 250 dps
        pub const LOW: Scale = (0b0000_00_0_0, 0.061); 
        /// 500 dps
        pub const MIDLO: Scale = (0b0000_01_0_0, 0.488);
        /// 1_000 dps
        pub const MIDHI: Scale = (0b0000_10_0_0, 0.122);
        /// 2_000 dps
        pub const MAX: Scale = (0b0000_11_0_0, 0.244);
    }
}

fn start_sensor(register: u8, rate: u8, scale: u8, filter: u8, bus: &mut I2c) -> rppal::i2c::Result<usize> {

    let starter = vec![register, rate | scale | filter];
    debug!(target: "LSM6DS33", "Writing to register: {:#b}", starter.get(1).unwrap());
    bus.write(&starter)

}

fn stop_sensor(register: u8, bus: &mut I2c) -> rppal::i2c::Result<usize> {
    let starter = vec![register, 0b0000_00_00];
    debug!(target: "LSM6DS33", "Writing to register: {:#b}", starter.get(1).unwrap());
    bus.write(&starter)
}

pub struct Accelerometer {
    pub active: bool,
}     

pub struct Gyroscope {
    pub active: bool,
}

impl Accelerometer {
    fn new() -> Accelerometer {
        Accelerometer {
            active: false,
        }
    }
}

impl Gyroscope {
    fn new() -> Gyroscope {
        Gyroscope {
            active: false,
        }
    }
}

impl DirectionalSensor for Accelerometer {

    fn get_x(&self, _: &mut rppal::i2c::Ipub2c) {
        todo!()
    }
    fn get_y(&self, _: &mut rppal::i2c::I2c) { todo!() }
    fn get_z(&self, _: &mut rppal::i2c::I2c) { todo!() }
}

impl GenericSensor for Accelerometer {

    /// Gets the activity status of the Accelerometer
    fn get_status(&self) -> bool { 
        self.active
    }

    fn start(&mut self, rate: u8, scale: u8, filter: u8, bus: &mut rppal::i2c::I2c) -> Result<(), Box<dyn Error>> { 
        info!(target: "LSM6DS33", "Starting Accelerometer");

        start_sensor(xl::ControlRegister::Reg1 as u8, rate, scale, filter, bus)?;

        info!(target: "LSM6DS33", "Accelerometer Started");

        Ok(())
    }
    fn stop(&mut self, bus: &mut rppal::i2c::I2c) -> Result<(), Box<dyn Error>> { 
        info!(target: "LSM6DS33", "Stopping Accelerometer");

        stop_sensor(gyro::ControlRegister::Reg1 as u8, bus)?;

        info!(target: "LSM6DS33", "Accelerometer Stopped");

        Ok(())
    }
}

impl DirectionalSensor for Gyroscope {

    fn get_x(&self, _: &mut rppal::i2c::I2c) { todo!() }
    fn get_y(&self, _: &mut rppal::i2c::I2c) { todo!() }
    fn get_z(&self, _: &mut rppal::i2c::I2c) { todo!() }
}

impl GenericSensor for Gyroscope {
    
    /// Gets the activity status of the Gyroscope
    fn get_status(&self) -> bool { 
        self.active
    }

    fn start(&mut self, rate: u8, scale: u8, filter: u8, bus: &mut rppal::i2c::I2c) -> Result<(), Box<dyn Error>> { 
        info!(target: "LSM6DS33", "Starting Gyroscope");

        start_sensor(gyro::ControlRegister::Reg1 as u8, rate, scale, filter, bus)?;

        info!(target: "LSM6DS33", "Gyroscope Started");

        Ok(())
    }
    fn stop(&mut self, bus: &mut rppal::i2c::I2c) -> Result<(), Box<dyn Error>> {
        info!(target: "LSM6DS33", "Stopping Gyroscope");

        stop_sensor(gyro::ControlRegister::Reg1 as u8, bus)?;

        info!(target: "LSM6DS33", "Gyroscope Stopped");

        Ok(())
    }
}

pub trait GenericSensor {
    fn get_status(&self) -> bool;
    fn start(&mut self, rate: u8, scale: u8, filter: u8, bus: &mut rppal::i2c::I2c) -> Result<(), Box<dyn Error>>;
    fn stop(&mut self, bus: &mut rppal::i2c::I2c) -> Result<(), Box<dyn Error>>;
}

pub trait DirectionalSensor:GenericSensor {
    fn get_x( bus: &mut rppal::i2c::I2c) -> f32;
    fn get_y( bus: &mut rppal::i2c::I2c) -> f32;
    fn get_z( bus: &mut rppal::i2c::I2c) -> f32;
}

struct DirectionalSensorImpl {}
impl DirectionalSensor for DirectionalSensorImpl {
    fn get_x( bus: &mut rppal::i2c::I2c) -> f32 {todo!()}
    fn get_y( bus: &mut rppal::i2c::I2c) -> f32 {todo!()}
    fn get_z( bus: &mut rppal::i2c::I2c) -> f32 {todo!()}
}

impl GenericSensor for DirectionalSensor {

}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
