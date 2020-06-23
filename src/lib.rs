use std::error::Error;
use log::{debug, info};
use byteorder::{BigEndian, ByteOrder};

/// The only two possible addresses for the sensor suite.
#[derive(Copy, Clone)]
pub enum ChipAddress {
    /// Address = 0x6A
    A = 0x6a,
    /// Address = 0x6B
    B = 0x6b,
}

impl std::fmt::Display for ChipAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChipAddress::A => write!(f, "0x6A"),
            ChipAddress::B => write!(f, "0x6B")
        }
    }
}
/// A scale for unit per least significant bit.
pub type Scale = (u8, f32);

/// An address to a register.
pub type Address = u8;

/// Data registers for the accelerometer.
pub mod xl {
    /// The control registers for the accelerometer.
    pub mod control_register {
        /// The primary control register for controlling the accelerometer. 
        /// This controls the data rate, scale, and anti-alias filter. 
        /// **Note**: This contains an anti-alias filter, unlike the gyroscope.
        pub const REG1: crate::Address = 0x10;
    }
    /// The data registers for the axes.
    pub mod data_register {
        pub const XL: crate::Address = 0x28;
        pub const XH: crate::Address = 0x29;
        pub const YL: crate::Address = 0x2A;
        pub const YH: crate::Address = 0x2B;
        pub const ZL: crate::Address = 0x2C;
        pub const ZH: crate::Address = 0x2D;
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
    /// The scales for converting bits to milli-gs.
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

    /// Anti-Aliasing filter modss
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

/// Data on all the registers for the gyroscope.
pub mod gyro {
    /// The only control register being implemented (for now).
    pub mod control_register {
        /// The primary data register. Controls data rate and scale
        /// **Note**: This does not contain an anti-alias filter, like the accelerometer.
        pub const REG1: crate::Address = 0x11;
    }

    /// These data rates are XL_HM_MODE = 1.
    /// If that register is 0, then high-performance mode is selected.
    pub enum DataRate {
        /// Gyro turned off.
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
    /// The data registers for the axes.
    pub mod data_register {
        pub const XL: crate::Address = 0x22;
        pub const XH: crate::Address = 0x23;
        pub const YL: crate::Address = 0x24;
        pub const YH: crate::Address = 0x25;
        pub const ZL: crate::Address = 0x26;
        pub const ZH: crate::Address = 0x27;
    }
    /// The scales for converting bits to milli-degrees.
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

/// The suite of sensors available to the chip.
pub enum SensorType {
    Accelerometer,
    Gyroscope,
    Thermometer,
}
impl std::fmt::Display for SensorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SensorType::Accelerometer => write!(f, "Accelerometer"),
            SensorType::Gyroscope => write!(f, "Gyroscope"),
            SensorType::Thermometer => write!(f, "Thermometer")
        }
    }
}

/*------------------*/
/*DirectionalSensor*/
/*----------------*/

/// A representation of one of the three types of sensors.
pub struct Sensor {
    active: bool,
    control_register: Option<u8>,
    sensor: SensorType,
    scale: Option<Scale>
}

impl Sensor {
    /// Constructs a new sensor of type SensorType.
    pub fn new(kind: SensorType) -> Sensor {
        let register = match kind {
            SensorType::Accelerometer => Some(xl::control_register::REG1),    
            SensorType::Gyroscope => Some(gyro::control_register::REG1),
            SensorType::Thermometer => None,
        };
        Sensor {
            active: false,
            control_register: register,
            scale: None,
            sensor: kind
        }
    }
    
    /// Change the sensor type (also changes the register).
    pub fn set_sensor(&mut self, kind: SensorType) {
        match kind {
            SensorType::Accelerometer => self.control_register = Some(xl::control_register::REG1 as u8),
            SensorType::Gyroscope => self.control_register = Some(gyro::control_register::REG1 as u8), 
            _ => self.control_register = None
        }
        self.sensor = kind;
    }

    /// Get the x axis of the sensor.
    pub fn get_x(&mut self, bus: &mut rppal::i2c::I2c) -> Result<Option<f32>, Box<dyn Error>> {
        info!(target: "LSM6DS33", "Getting {} X", self.sensor);

        match &self.sensor {
            SensorType::Accelerometer => {
                self.get_axis(xl::data_register::XH, xl::data_register::XL, bus)
            },
            SensorType::Gyroscope => {
                self.get_axis(gyro::data_register::XH, gyro::data_register::XL, bus)
            },
            _ => panic!("Not  Implemented")
        }

    }

    /// Get the y axis of the sensor.
    pub fn get_y(&mut self, bus: &mut rppal::i2c::I2c) -> Result<Option<f32>, Box<dyn Error>>  {
        info!(target: "LSM6DS33", "Getting {} Y", self.sensor);

        match &self.sensor {
            SensorType::Accelerometer => {
                self.get_axis(xl::data_register::YH, xl::data_register::YL, bus)
            },
            SensorType::Gyroscope => {
                self.get_axis(gyro::data_register::YH, gyro::data_register::YL, bus)
            },
            _ => panic!("Not  Implemented")
        }
    }

    /// Get the z axis of the sensor.
    pub fn get_z(&mut self, bus: &mut rppal::i2c::I2c) -> Result<Option<f32>, Box<dyn Error>>  {
        info!(target: "LSM6DS33", "Getting {} Z", self.sensor);

        match &self.sensor {
            SensorType::Accelerometer => {
                self.get_axis(xl::data_register::ZH, xl::data_register::ZL, bus)
            },
            SensorType::Gyroscope => {
                self.get_axis(gyro::data_register::ZH, gyro::data_register::ZL, bus)
            },
            _ => panic!("Not  Implemented")
        }
    }

    fn get_axis(&mut self, reg_high: u8, reg_low: u8, bus: &mut rppal::i2c::I2c) -> Result<Option<f32>, Box<dyn Error>> {
        if self.active == false {
            return Ok(None);
        }

        let a_high = self.get_register(reg_high, bus)?.unwrap();
        let a_low = self.get_register(reg_low, bus)?.unwrap();

        let raw_data = BigEndian::read_i16(&vec![a_high, a_low]);

        return Ok(Some((raw_data as f32 * self.scale.unwrap().1) / 1000.0))
    }
}

impl GenericSensorTrait for Sensor {

    /// Get the status of the sensor.
    fn get_status(&self) -> bool {
        self.active
     }

    /// Start refreshing the sensors data buffer.
    fn start(&mut self, rate: u8, scale: Scale, filter: u8, bus: &mut rppal::i2c::I2c) -> Result<(), Box<(dyn std::error::Error + 'static)>> {
        info!(target: "LSM6DS33", "Starting {}", self.sensor);

        if let Some(control_reg) = self.control_register {
            let starter = vec![control_reg, rate | scale.0 | filter];
            debug!(target: "LSM6DS33", "Writing to register: {:#b}", starter.get(1).unwrap());
            bus.write(&starter)?;
            self.scale = Some(scale);
        }
        self.active = true;
        info!(target: "LSM6DS33", "{} Started", self.sensor);
        Ok(())
    }

    ///Stop the sensor from collecting data.
    fn stop(&mut self, bus: &mut rppal::i2c::I2c) -> Result<(), Box<(dyn std::error::Error + 'static)>> {
        info!(target: "LSM6DS33", "Stopping {}", self.sensor);
        if let Some(control_register) = self.control_register {
            // Read the current mode.
            let mut mode =  vec![0];
            bus.write_read(&vec![control_register], &mut mode)?;
            debug!(target: "LSM6DS33", "Pre-Stop Current Mode: {:b}", mode.get(0).unwrap());

            // Set the ODR to 0
            let mode = mode.get(0).unwrap() & 0b0000_1111;
            debug!(target: "LSM6DS33", "Stop Mode: {:b}", mode);
        
            // Send the new ODR
            let stopper = vec![control_register, mode];
            bus.write(&stopper)?;
        }

        self.active = false;
        self.scale = None;
        info!(target: "LSM6DS33", "{} Stopped", self.sensor);
        Ok(())
    }

    /// Get a single register from the sensor.
    fn get_register(&mut self, addr: crate::Address, bus: &mut rppal::i2c::I2c) -> Result<Option<u8>, Box<(dyn std::error::Error + 'static)>> { 
        if self.active == false {
            return Ok(None);
        }

        let mut buf = vec![0];
        bus.write_read(&vec![addr], &mut buf)?;
        Ok(Some(*buf.get(0).unwrap()))
     }
}

/*-----------*/
/* Generics */
/*---------*/

/// In case anyone decides to extend this package.
pub trait GenericSensorTrait {
    fn get_status(&self) -> bool;
    fn start(&mut self, rate: u8, scale: Scale, filter: u8, bus: &mut rppal::i2c::I2c) -> Result<(), Box<dyn Error>>;
    fn stop(&mut self, bus: &mut rppal::i2c::I2c) -> Result<(), Box<dyn Error>>;
    fn get_register(&mut self, register: crate::Address, bus: &mut rppal::i2c::I2c) -> Result<Option<u8>, Box<dyn Error>>;
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
