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
