//! This is a minimal implementation of the [System Management Bus (SMBus)][smbus]
//! [Packet Error Code][smbus-pec] calculation algorithm intended for use in `no_std`.
//!
//! SMBus 1.1 and later defines an optional Packet Error Checking mode. When used, an extra byte
//! is appended to all transmissions containing a Packet Error Code (PEC).
//!
//! The PEC is calculated over the whole transmission including address and read/write bit.
//! The polynomial used is `x^8 + x^2 + x + 1`, which corresponds to [CRC-8-ATM HEC][crc8]
//! initialized to zero.
//!
//! [crc8]: https://en.wikipedia.org/wiki/CRC-8
//! [smbus]: https://en.wikipedia.org/wiki/System_Management_Bus
//! [smbus-pec]: https://en.wikipedia.org/wiki/System_Management_Bus#Packet_Error_Checking
//!
//! # Usage
//!
//! ## Send command with a value to an address
//!
//! A typical use case for this would be writing the value to a register
//!
//! ```rust
//! # struct Smbus;
//! # impl Smbus {
//! #   fn write(&mut self, address: u8, data: &[u8]) {}
//! # }
//! #
//! # let mut smbus = Smbus {};
//! #
//! use smbus_pec::pec;
//! const ADDR: u8 = 0x5A;
//! let command = 0x06;
//! let value = 0xAB;
//!
//! let checksum = pec(&[ADDR << 1, command, value]);
//! smbus.write(ADDR, &[command, value, checksum]);
//! ```
//!
//! ## Perform request-response
//!
//! A typical use case for this would be reading the value of a register.
//! To do this, a write with the register address is sent, followed by a read.
//!
//! ```rust
//! # struct Smbus;
//! # impl Smbus {
//! #   fn write_read(&mut self, address: u8, data: &[u8], buffer: &mut [u8]) {}
//! # }
//! #
//! # let mut smbus = Smbus {};
//! #
//! use smbus_pec::pec;
//! const ADDR: u8 = 0x5A;
//! let register = 0x06;
//! let mut data = [0; 2];
//!
//! smbus.write_read(ADDR, &[register], &mut data);
//! let checksum = pec(&[ADDR << 1, register, (ADDR << 1) | 1, data[0]]);
//! if checksum != data[1] {
//!   println!("Packet Error Code mismatch.");
//! }
//! let value = data[0];
//! ```
//!

#![doc(html_root_url = "https://docs.rs/smbus-pec/0.1.0")]
#![deny(unsafe_code, missing_docs)]
#![no_std]

/// Calculate SMBus Packet Error Code over transmitted data.
///
/// The input data array must contain the complete message including address and read/write bit.
pub fn pec(data: &[u8]) -> u8 {
    const POLY: u8 = 7; // x^8 + x^2 + x + 1
    let mut crc = 0;
    for v in data {
        crc ^= v;
        for _ in 0..8 {
            crc = if (crc & (1 << 7)) != 0 {
                (crc << 1) ^ POLY
            } else {
                crc << 1
            };
        }
    }
    crc
}
