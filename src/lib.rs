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
//! ## How this crate compares to others
//!
//! There is a number of crates implementing CRC algorithms but their intention is to
//! implement several of them like CRC-8, CRC-16, CRC-32, be configurable, use
//! acceleration via SIMD instructions, etc.
//!
//! This crate provides a portable and non-configurable implementation of exactly one
//! algorithm: the one used for SMBus PEC and no more than that.
//! This should allow the compiler to make good optimizations and allows for use of the
//! algorithm in any target architecture with minimal code bloat.
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

#[cfg(not(feature = "lookup-table"))]
mod default_impl {
    use embedded_crc_macros::{crc8, crc8_hasher};
    crc8!(
        pec,
        7,
        0,
        "Calculate SMBus Packet Error Code over transmitted data.\
\n\nThe input data array must contain the complete message including address and read/write bit."
    );

    crc8_hasher!(
        Pec,
        7,
        0,
        "Calculate SMBus Packet Error Code over transmitted data. `core::hash::Hasher` implementation.\
\n\nThe input data array must contain the complete message including address and read/write bit."
    );
}

#[cfg(not(feature = "lookup-table"))]
pub use crate::default_impl::{pec, Pec};

#[cfg(feature = "lookup-table")]
mod lookup_table_impl {
    use crate::LOOKUP_TABLE;
    use embedded_crc_macros::{crc8_hasher_lookup_table, crc8_lookup_table};

    crc8_lookup_table!(
        pec,
        0,
        "Calculate SMBus Packet Error Code over transmitted data.\
\n\nThe input data array must contain the complete message including address and read/write bit.
\n\nThis implementation uses a lookup table and is much faster at the cost of some space."
    );

    crc8_hasher_lookup_table!(
        Pec,
        0,
        "Calculate SMBus Packet Error Code over transmitted data. `core::hash::Hasher` implementation.\
\n\nThe input data array must contain the complete message including address and read/write bit."
    );
}

#[cfg(feature = "lookup-table")]
pub use crate::lookup_table_impl::{pec, Pec};

#[cfg(feature = "lookup-table")]
include!(concat!(env!("OUT_DIR"), "/lookup_table.rs"));
