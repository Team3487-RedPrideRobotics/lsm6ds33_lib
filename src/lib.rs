
//FIFO Config Registers

/// FIFO contol register (r/w).
/// FIFO threshold level setting.
/// Default value: 0000 0000.
/// Watermark falg rises when th enumber of bytes written
/// to FIFO after the next write is greater than or equal
/// to the threshold level.
/// Minimum resolution fo the FIFO is 1 LSB = 2 bytes (1 word) in FIFO
pub const FIFO_CTRL1:u8 = 0x06;
pub const FIFO_CTRL2:u8 = 0x07;
pub const FIFO_CTRL3:u8 = 0x08;
pub const FIFO_CTRL4:u8 = 0x09;
pub const FIFO_CTRL5:u8 = 0x0A;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
