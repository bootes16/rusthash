use crate::Checksum;

pub struct Adler32 {
    a: u32,
    b: u32,
}

impl Checksum for Adler32 {
    fn update(&mut self, data: &str) {
        for d in data.bytes() {
            self.a = (self.a + d as u32) % 65521;
            self.b = (self.b + self.a) % 65521;
        }
    }

    fn checksum(&self) -> u32 {
        (self.b << 16) | self.a
    }

    fn hex_digest(&self) -> String {
        return format!("{:08x}", self.checksum());
    }

    fn checksum_from_string(data:&str) -> u32 {
        let mut adlr = Adler32{a: 1, b: 0};
        adlr.update(data);
        return adlr.checksum();
    }
}

impl Adler32 {
    pub fn new() -> Adler32 {
        Adler32 {a: 1, b: 0}
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_adler32() {
        let test_data: Vec<(u32, &str)> = vec![
            (0x00000001, ""),
            (0x00620062, "a"),
            (0x024d0127, "abc"),
            (0x29750586, "message digest"),
            (0x90860b20, "abcdefghijklmnopqrstuvwxyz"),
            (0x8adb150c, "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789"),
            (0x97b61069, "12345678901234567890123456789012345678901234567890123456789012345678901234567890")
        ];

        for (checksum, text) in test_data {
            assert_eq!(Adler32::checksum_from_string(text), checksum);
        }
    }

    #[test]
    fn test_two() {
        let mut adlr = Adler32::new();
        adlr.update("abc");
        assert_eq!(adlr.hex_digest(), "024d0127")

    }

    #[test]
    fn test_digest() {
        let adlr = Adler32{a: 0xbeef, b: 0xdead};
        assert_eq!(adlr.hex_digest(), "deadbeef");
    }
}
