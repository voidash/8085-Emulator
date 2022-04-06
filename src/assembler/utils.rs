use std::str::FromStr;
pub fn convert_8085_hex_to_i32(data: &str) -> i32 {
    i32::from_str_radix(&data[0..data.len() - 1], 16).unwrap()
}

pub fn convert_8085_oct_to_i32(data: &str) -> i32 {
    i32::from_str_radix(&data[0..data.len() - 1], 8).unwrap()
}

pub fn convert_8085_dec_to_i32(data: &str) -> i32 {
    if data.contains("D") {
        i32::from_str_radix(&data[0..data.len() - 1], 10).unwrap()
    } else {
        i32::from_str(data).unwrap()
    }
}

#[test]
fn test_hex_conversion() {
    assert_eq!(convert_8085_hex_to_i32("85H"), 133);
}

#[test]
fn test_octal_conversion() {
    assert_eq!(convert_8085_oct_to_i32("14Q"), 12);
}

#[test]
fn test_dec_conversion() {
    assert_eq!(convert_8085_dec_to_i32("14D"), 14);
    assert_eq!(convert_8085_dec_to_i32("14"), 14);
}
