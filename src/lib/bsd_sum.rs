use crate::Checksum;

pub struct BsdSum {
    pub checksum: u32,
}

#[allow(dead_code, unused)]
impl Checksum for BsdSum {
    fn update(&mut self, data: &str) {}
    fn checksum(&self) -> u32 {0}
    fn hex_digest(&self) -> String { return String::from("");}
    fn checksum_from_string(data:&str) -> u32 {0}
}

#[allow(dead_code, unused)]
impl BsdSum {
    pub fn new() -> BsdSum {
        BsdSum{checksum: 0}
    }
}
// void BsdSum::append(std::istream& istr) {
//     char c;
//     while (istr.get(c)) {
//         checksum = (checksum >> 1) + ((checksum & 1) << 15);
//         checksum += static_cast<unsigned char>(c);
//         checksum &= 0xffff;       /* Keep it within bounds. */

//         byte_count++;
//     }
// }

// std::string BsdSum::hexdigest(void) const {
//     char buf[8];
//     sprintf(buf, "%05x", checksum);
//     return std::string(buf);
// }

// //
// // System V checksum output: <checksum> <blocks>
// // separated by spaces where:
// //  <checksum> - non-zero padded, non-aligned decimal 16bit checksum value.
// //  <blocks> - number of 512byte blocks
// //
// std::string BsdSum::to_string(void) const {
//     std::ostringstream oss;
//     oss << std::setw(5) << std::setfill('0') << checksum <<
//         " " << std::setfill(' ') << std::setw(5) << num_blocks();
//     return oss.str();
// }

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_new() {
        let mut sum = BsdSum::new();
        assert_eq!(sum.checksum(), 0);
        sum.update("");
        assert_eq!(sum.checksum(), 0);
    }
}