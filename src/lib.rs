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

pub mod xl {
    
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

pub mod gyro {
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

/*----------------*/
/* Accelerometer */
/*--------------*/
pub struct Accelerometer {
    sensor: DirectionalSensorImpl,
}

impl Accelerometer {
    fn new() -> Accelerometer {
        Accelerometer {
            sensor: DirectionalSensorImpl {
                active: false,
                control_register: xl::ControlRegister::Reg1 as u8,
            }
        }
    }
}

impl GenericSensor for Accelerometer {
    fn get_status(&self) -> bool {
        self.sensor.get_status()
    }
    fn start(&mut self, rate: u8, scale: u8, filter: u8, bus: &mut rppal::i2c::I2c) -> std::result::Result<(), std::boxed::Box<(dyn std::error::Error + 'static)>> {
        info!(target:"LSM6DS33", "Starting Accelerometer");
        let result = self.sensor.start(rate, scale, filter, bus);
        if let Ok(()) = result {
            info!(target: "LSM6DS33", "Accelerometer Started");
        }
        result
    }
    fn stop(&mut self, bus: &mut rppal::i2c::I2c) -> std::result::Result<(), std::boxed::Box<(dyn std::error::Error + 'static)>> {
        self.sensor.stop(bus)
    }
}

impl DirectionalSensor for Accelerometer {
    fn get_x(&self, bus: &mut rppal::i2c::I2c) -> f32 {
        todo!()
    }
    fn get_y(&self, bus: &mut rppal::i2c::I2c) -> f32 {
        todo!()
    }
    fn get_z(&self, bus: &mut rppal::i2c::I2c) -> f32 {
        todo!()
    }
}
/*------------*/
/* Gyroscope */
/*----------*/
pub struct Gyroscope {
    sensor: DirectionalSensorImpl,
}

impl Gyroscope {
    fn new() -> Gyroscope {
        Gyroscope {
            sensor: DirectionalSensorImpl {
                active: false,
                control_register: gyro::ControlRegister::Reg1 as u8,
            }
        }
    }
}

impl GenericSensor for Gyroscope {
    fn get_status(&self) -> bool {
        self.sensor.get_status()
     }
    fn start(&mut self, rate: u8, scale: u8, filter: u8, bus: &mut rppal::i2c::I2c) -> std::result::Result<(), std::boxed::Box<(dyn std::error::Error + 'static)>> {
        info!(target:"LSM6DS33", "Starting Gyroscope");
        let result = self.sensor.start(rate, scale, filter, bus);
        if let Ok(()) = result {
            info!(target: "LSM6DS33", "Gyroscope Started");
        }
        result
    }
    fn stop(&mut self, bus: &mut rppal::i2c::I2c) -> std::result::Result<(), std::boxed::Box<(dyn std::error::Error + 'static)>> { 
        info!(target:"LSM6DS33", "Stopping Gyroscope");
        let result = self.sensor.stop(bus);
        if let Ok(()) = result {
            info!(target: "LSM6DS33", "Gyroscope Stopped");
        }
        result
    }
}

impl DirectionalSensor for Gyroscope {
fn get_x(&self, _: &mut rppal::i2c::I2c) -> f32 { todo!() }
fn get_y(&self, _: &mut rppal::i2c::I2c) -> f32 { todo!() }
fn get_z(&self, _: &mut rppal::i2c::I2c) -> f32 { todo!() }
}

/*------------------*/
/*DirectionalSensor*/
/*----------------*/

struct DirectionalSensorImpl {
    active: bool,
    control_register: u8,
}

impl DirectionalSensor for DirectionalSensorImpl {
    fn get_x(&self, bus: &mut rppal::i2c::I2c) -> f32 {todo!()}
    fn get_y(&self, bus: &mut rppal::i2c::I2c) -> f32 {todo!()}
    fn get_z(&self, bus: &mut rppal::i2c::I2c) -> f32 {todo!()}
}

impl GenericSensor for DirectionalSensorImpl {

    fn get_status(&self) -> bool { 
        self.active
     }

    fn start(&mut self, rate: u8, scale: u8, filter: u8, bus: &mut rppal::i2c::I2c) -> std::result::Result<(), std::boxed::Box<(dyn std::error::Error + 'static)>> {

        let starter = vec![self.control_register, rate | scale | filter];
        debug!(target: "LSM6DS33", "Writing to register: {:#b}", starter.get(1).unwrap());

        bus.write(&starter)?;
        self.active = true;
        Ok(())
    }
    fn stop(&mut self, bus: &mut rppal::i2c::I2c) -> std::result::Result<(), std::boxed::Box<(dyn std::error::Error + 'static)>> {
        /// Read the current mode
        let mut mode =  vec![0];
        bus.write_read(&vec![self.control_register], &mut mode)?;
        debug!(target: "LSM6DS33", "Pre-Stop Current Mode: {:b}", mode.get(0).unwrap());
        
        /// Set the ODR to 0
        let mode = mode.get(0).unwrap() & 0b0000_1111;
        debug!(target: "LSM6DS33", "Stop Mode: {:b}", mode);
    
        /// Send the new ODR
        let stopper = vec![self.control_register, mode];
        bus.write(&stopper);

        self.active = false;
        Ok(())
    }
}


/*-----------*/
/* Generics */
/*---------*/

pub trait GenericSensor {
    fn get_status(&self) -> bool;
    fn start(&mut self, rate: u8, scale: u8, filter: u8, bus: &mut rppal::i2c::I2c) -> Result<(), Box<dyn Error>>;
    fn stop(&mut self, bus: &mut rppal::i2c::I2c) -> Result<(), Box<dyn Error>>;
}

pub trait DirectionalSensor:GenericSensor {
    fn get_x(&self, bus: &mut rppal::i2c::I2c) -> f32;
    fn get_y(&self, bus: &mut rppal::i2c::I2c) -> f32;
    fn get_z(&self, bus: &mut rppal::i2c::I2c) -> f32;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
