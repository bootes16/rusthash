// use csum_lib;
use csum_lib::Checksum;
use csum_lib::adler32::Adler32;

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
