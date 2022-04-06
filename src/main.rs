mod lib;
use lib::Checksum;

fn main() {
    let mut sum = lib::adler32::Adler32::new();
    sum.update("Hello World!");
    println!("{}", sum.hex_digest());
    println!("{:?} 0x{0:08x}", lib::adler32::Adler32::checksum_from_string("Hello World!"));
}
