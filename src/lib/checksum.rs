pub trait Checksum {
    fn update(&mut self, data: &str);
    fn checksum(&self) -> u32;
    fn hex_digest(&self) -> String;
    fn checksum_from_string(data:&str) -> u32;
    // fn new() -> Adler32;
}
