use csum_lib::Checksum;
use csum_lib::bsd_sum::BsdSum;


#[test]
fn test_basic() {
    assert!(true);
}

#[test]
fn test_corpus() {
    let check_data: Vec<(&str, &str, &str, &str)> = vec![
        (
            "",
            "00000     0",
            "0 0",
            "4294967295 0"
        )
    ];

    for (input, _, _, _) in check_data {
        let mut sum: BsdSum = BsdSum{checksum: 0};
        sum.update(input);
        assert_eq!(sum.checksum, 0);
    }
}

// static const CheckData check_data[] = {
//     {
//         "Hello World!",
//         "02760     1",
//         "1085 1",
//         "2817829334 12"
//     },
//     // Zero length string.
//     {
//         "",
//         "00000     0",
//         "0 0",
//         "4294967295 0"
//     },
//     // Another zero length string.
//     {
//         "\0",
//         "00000     0",
//         "0 0",
//         "4294967295 0"
//     },
//     {
//         "\xff",
//         "00255     1",
//         "255 1",
//         "3045181057 1"
//     },
//     {
//         "\xff\xfe\xfd\xfc",
//         "57817     1",
//         "1014 1",
//         "115125973 4"
//     }
// };
