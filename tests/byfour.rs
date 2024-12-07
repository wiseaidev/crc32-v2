use crc32_v2::byfour::{crc32_little, dolit32, dolit4, slice_u8_as_u32};
use crc32_v2::crc32;

#[test]
fn test_dolit4() {
    let mut crc = 0u32;
    let buf = [0u8, 1u8, 2u8, 3u8];
    let buf4 = unsafe { std::slice::from_raw_parts(buf.as_ptr() as *const u32, 1) };
    let mut buf4pos = 0;
    dolit4(&mut crc, buf4, &mut buf4pos);
    assert_eq!(crc, 0xAAFD590F);
}

#[test]
fn test_dolit32() {
    let mut crc = 0u32;
    let buf = [0u8; 32];
    let buf4 = unsafe { std::slice::from_raw_parts(buf.as_ptr() as *const u32, 8) };
    let mut buf4pos = 0;
    dolit32(&mut crc, buf4, &mut buf4pos);
    assert_eq!(crc, 0);
}

#[test]
fn test_crc32_little() {
    let crc = crc32_little(0, &[0u8, 1u8, 2u8, 3u8]);
    assert_eq!(crc, 0x55bc80e2);
}

#[test]
fn test_crc32() {
    // Known data: [0, 1, 2, 3]
    let crc = crc32(0, &[0u8, 1u8, 2u8, 3u8]);
    assert_eq!(crc, 0x8BB98613);

    // Test with an empty byte array
    let empty_data: Vec<u8> = vec![];
    let computed_crc = crc32(0, &empty_data);
    assert_eq!(computed_crc, 0);

    // Test with the string "Hello"
    let hello_data = b"Hello";
    let computed_crc_hello = crc32(0, hello_data);
    assert_eq!(computed_crc_hello, 0xf7d18982);
}

#[test]
fn test_slice_u8_as_u32() {
    let bytes = [0u8, 1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8];
    let u32_slice = slice_u8_as_u32(&bytes);
    assert_eq!(u32_slice, &[50462976u32, 117835012u32]);
}
