# Rust Portable SMBus Packet Error Code Algorithm

[![crates.io](https://img.shields.io/crates/v/smbus-pec.svg)](https://crates.io/crates/smbus-pec)
[![Docs](https://docs.rs/smbus-pec/badge.svg)](https://docs.rs/smbus-pec)
[![Build Status](https://travis-ci.com/eldruin/smbus-pec-rs.svg?branch=master)](https://travis-ci.com/eldruin/smbus-pec-rs)
[![Coverage Status](https://coveralls.io/repos/github/eldruin/smbus-pec-rs/badge.svg?branch=master)](https://coveralls.io/github/eldruin/smbus-pec-rs?branch=master)

This is a portable minimal implementation of the [System Management Bus (SMBus)][smbus]
[Packet Error Code][smbus-pec] calculation algorithm intended for use in `no_std`.

SMBus 1.1 and later defines an optional Packet Error Checking mode. When used, an extra byte
is appended to all transmissions containing a Packet Error Code (PEC).

The PEC is calculated over the whole transmission including address and read/write bit.

The polynomial used is `x^8 + x^2 + x + 1`, which corresponds to [CRC-8-ATM HEC][crc8]
initialized to zero.

# How this crate compares to others

There is a number of crates implementing CRC algorithms but their intention is to
be configurable, generic, use acceleration via SIMD instructions, etc.

This crate provides a portable and non-configurable implementation of exactly one
algorithm: The one used for SMBus PEC (optionally using a pre-calculated lookup table).

This should allow the compiler to make good optimizations and allows for use of the
algorithm in any target architecture with minimal code bloat.

This makes this crate specially well suited for use in `no_std` environments.

# Pre-calculated lookup table

A faster version of the algorithm is provided through the use of a pre-calculated
lookup table. This can be enabled through the `lookup-table` feature.

With this feature enabled a table of 256 pre-calculated `u8` values will be included
which avoids bit-by-bit calculation at the cost of the space needed to store it.

## Usage

```rust
use smbus_pec::pec;

const ADDRESS: u8 = 0x5A;
const REGISTER: u8 = 0x06;

fn main() {
    let pec_write = pec(&[ADDRESS << 1, REGISTER, 0xAB, 0xCD]);
    println!("PEC: {}", pec_write); // prints 95

    let data = [ADDRESS << 1, REGISTER, (ADDRESS << 1) + 1, 38, 58];
    let pec_write_read = pec(&data);
    println!("PEC: {}", pec_write_read); // prints 102
}
```

## Support

For questions, issues, feature requests, other changes, or just feedback, please file an
[issue in the github project](https://github.com/eldruin/smbus-pec-rs/issues).

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

### Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.

[crc8]: https://en.wikipedia.org/wiki/CRC-8
[smbus]: https://en.wikipedia.org/wiki/System_Management_Bus
[smbus-pec]: https://en.wikipedia.org/wiki/System_Management_Bus#Packet_Error_Checking
