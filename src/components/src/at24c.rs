/*
* Code fore interacting with the AT24C256C and AT24C128C flash memory chips
*/
use embedded_hal::blocking::i2c::{Write, Read};
use rp_pico::hal;

const DEVICE_ADDRESS: u8 = 0b1010000;

pub enum I2cError {
    ReadError,
    WriteError,
}

pub enum At24cError {
    I2cError(I2cError),
    // Raised when a a buffer larger than 64 bytes is written
    InvalidSize,
    AddressOutOfRange,
}

impl Into<At24cError> for I2cError {
    fn into(self) -> At24cError {
        At24cError::I2cError(self)
    }
}

#[repr(u16)]
pub enum At24cMemSize {
    Kb128 = 128,
    Kb256 = 256,
}

pub struct At24c<'t, T> {
    size: At24cMemSize,
    i2c: T,
    tx_buffer: [u8; 66],
    last_write: u32,
    timer: &'t hal::Timer,
}

impl<'t, T: Read + Write> At24c<'t, T> {
    pub fn new(i2c: T, size: At24cMemSize, timer: &'t hal::Timer) -> At24c<'t, T> {
        At24c {
            size,
            i2c,
            timer,
            tx_buffer: [0; 66],
            last_write: 0
        }
    }


    // The AT24C* supports sequential reads of up to 64 bytes during at once
    pub fn read(&mut self, address: u16, buf: &mut [u8]) -> Result<(), At24cError> {
        // Check address and page boundries
        let _ = self.is_address_valid(address)?;
        if buf.len() > 64 {
            return Err(At24cError::InvalidSize);
        }

        let addrs_hi = ((address & 0xFF00) >> 8) as u8;
        let addrs_lo = (address & 0x00FF) as u8;
        self.wait_write_cycle();
        match self.i2c.write(DEVICE_ADDRESS, &[addrs_hi, addrs_lo]) {
            Ok(..) => {},
            Err(_) => return Err(I2cError::WriteError.into()), 
        }
        match self.i2c.read(DEVICE_ADDRESS, buf) {
            Ok(..) => {},
            Err(_) => return Err(I2cError::ReadError.into()), 
        }
        Ok(())
    }

    // The AT24C* support page writes of up to 64 bytes during
    // a single write cycle
    pub fn write(&mut self, address: u16, buf: &[u8]) -> Result<(), At24cError> {
        // Check address and page boundries
        let _ = self.is_address_valid(address)?;
        if buf.len() > 64 {
            return Err(At24cError::InvalidSize);
        }

        buf.iter().enumerate().for_each(|(i, b)| self.tx_buffer[2 + i] = *b);

        let addrs_hi = ((address & 0xFF00) >> 8) as u8;
        let addrs_lo = (address & 0x00FF) as u8;
        self.tx_buffer[0] = addrs_hi;
        self.tx_buffer[1] = addrs_lo;

        self.wait_write_cycle();
        match self.i2c.write(DEVICE_ADDRESS, &self.tx_buffer) {
            Ok(..) => {},
            Err(_) => return Err(I2cError::WriteError.into()), 
        }
        self.last_write = self.timer.get_counter_low();
        self.clear_tx_buffer();
        Ok(())
    }

    // The AT24C256C and AT24C128C have a maximum read cycle time of 5ms
    // after a write the internal write cycle time starts and you are not
    // allowed to read/write to the device during this time
    // If the last write happened later than 5ms ago this function blocks
    // until 5ms have passed
    pub fn wait_write_cycle(&self) {
        while self.timer.get_counter_low() - self.last_write < 5000 {
            cortex_m::asm::nop(); 
        }
    }

    // Checks that the requrested address is within the address range
    // of the device
    fn is_address_valid(&self, address: u16) -> Result<(), At24cError> {
        match self.size {
            At24cMemSize::Kb128 => {
                if address >= 16_384 {
                    return Err(At24cError::AddressOutOfRange);
                }
            },
            At24cMemSize::Kb256 => {
                if address >= 32_768 {
                    return Err(At24cError::AddressOutOfRange);
                }
            },
        }
        Ok(())
    }

    fn clear_tx_buffer(&mut self) {
        self.tx_buffer.iter_mut().for_each(|b| *b = 0);
    }
}
