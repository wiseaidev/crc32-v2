// Copyright 2026 Mahmoud Harmouch.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crc32_v2::byfour::{crc32_big, crc32_little, dolit4, dolit32, slice_u8_as_u32};
use crc32_v2::{Digest, crc32, crc32_combine};

const HELLO_WORLD: &[u8] = b"Hello, world!";

fn crc32fast_ref(data: &[u8]) -> u32 {
    crc32fast::hash(data)
}

#[test]
fn test_crc32_known_values() {
    assert_eq!(crc32(0, &[0u8, 1u8, 2u8, 3u8]), 0x8BB9_8613);
    assert_eq!(crc32(0, b"Hello"), 0xF7D1_8982);
    assert_eq!(crc32(0, HELLO_WORLD), 0xEBE6_C6E6);
    assert_eq!(crc32(0, b"123456789"), 0xCBF4_3926);
}

#[test]
fn test_crc32_empty_buffer() {
    assert_eq!(crc32(0, &[]), 0);
}

#[test]
fn test_crc32_single_zero_byte() {
    let expected = crc32fast_ref(&[0x00]);
    assert_eq!(crc32(0, &[0x00]), expected);
}

#[test]
fn test_crc32_single_ff_byte() {
    let expected = crc32fast_ref(&[0xFF]);
    assert_eq!(crc32(0, &[0xFF]), expected);
}

#[test]
fn test_crc32_all_zeros_1k() {
    let data = vec![0u8; 1024];
    let expected = crc32fast_ref(&data);
    assert_eq!(crc32(0, &data), expected);
}

#[test]
fn test_crc32_all_ff_1k() {
    let data = vec![0xFFu8; 1024];
    let expected = crc32fast_ref(&data);
    assert_eq!(crc32(0, &data), expected);
}

#[test]
fn test_crc32_chaining_buffers() {
    let full = crc32(0, HELLO_WORLD);
    let chained = crc32(crc32(0, b"Hello, "), b"world!");
    assert_eq!(full, chained);
}

#[test]
fn test_crc32_cross_validates_crc32fast() {
    for chunk in [
        b"".as_ref(),
        b"a".as_ref(),
        b"\x00\xFF\xAA\x55".as_ref(),
        HELLO_WORLD,
        b"The quick brown fox jumps over the lazy dog".as_ref(),
    ] {
        assert_eq!(
            crc32(0, chunk),
            crc32fast_ref(chunk),
            "mismatch for {:?}",
            chunk
        );
    }
}

#[test]
fn test_crc32_arbitrary_large_buffer() {
    let data: Vec<u8> = (0u8..=255).cycle().take(65536).collect();
    let expected = crc32fast_ref(&data);
    assert_eq!(crc32(0, &data), expected);
}

#[test]
fn test_crc32_little_known_values() {
    assert_eq!(crc32_little(0, &[0u8, 1u8, 2u8, 3u8]), 0x8BB9_8613);
    assert_eq!(crc32_little(0, HELLO_WORLD), crc32fast::hash(HELLO_WORLD));
}

#[test]
fn test_crc32_little_empty_buffer() {
    assert_eq!(crc32_little(0, &[]), 0);
}

#[test]
fn test_crc32_little_all_zeros_1k() {
    let data = vec![0u8; 1024];
    assert_eq!(crc32_little(0, &data), crc32fast::hash(&data));
}

#[test]
fn test_crc32_little_all_ff_64() {
    let data = vec![0xFFu8; 64];
    let via_scalar = crc32(0, &data);
    let via_byfour = crc32_little(0, &data);
    assert_ne!(via_scalar, 0);
    assert_eq!(via_scalar, via_byfour);
}

#[test]
fn test_crc32_little_matches_crc32_for_aligned_data() {
    let data: Vec<u8> = (0u8..=255).collect();
    assert_eq!(crc32(0, &data), crc32_little(0, &data));
}

#[test]
fn test_crc32_big_empty_buffer() {
    assert_eq!(crc32_big(0, &[]), 0);
}

#[test]
fn test_crc32_big_all_zeros_1k() {
    let data = vec![0u8; 1024];
    let v = crc32_big(0, &data);
    assert_eq!(v, crc32_big(0, &data));
}

#[test]
fn test_crc32_big_non_zero_data() {
    let result = crc32_big(0, HELLO_WORLD);
    assert_ne!(result, 0);
}

#[test]
fn test_crc32_big_chaining() {
    let full = crc32_big(0, HELLO_WORLD);
    let p1 = crc32_big(0, b"Hello, ");
    let chained = crc32_big(p1, b"world!");
    assert_eq!(full, chained);
}

#[test]
fn test_crc32_combine_basic() {
    let crc1 = crc32(0, b"Hello, ");
    let crc2 = crc32(0, b"world!");
    let combined = crc32_combine(crc1, crc2, b"world!".len() as u64);
    assert_eq!(combined, crc32(0, HELLO_WORLD));
}

#[test]
fn test_crc32_combine_zero_length_second() {
    let crc1 = crc32(0, b"Hello, world!");
    let combined = crc32_combine(crc1, crc32(0, &[]), 0);
    assert_eq!(combined, crc1);
}

#[test]
fn test_crc32_combine_large_second() {
    let part1 = b"Part one of the message.".as_ref();
    let part2 = (0u8..=255).cycle().take(10_000).collect::<Vec<_>>();

    let crc1 = crc32(0, part1);
    let crc2 = crc32(0, &part2);
    let combined = crc32_combine(crc1, crc2, part2.len() as u64);

    let mut full = part1.to_vec();
    full.extend_from_slice(&part2);
    assert_eq!(combined, crc32(0, &full));
}

#[test]
fn test_digest_equals_single_shot() {
    let mut digest = Digest::new();
    digest.update(b"Hello, ");
    digest.update(b"world!");
    assert_eq!(digest.finalize(), crc32(0, HELLO_WORLD));
}

#[test]
fn test_digest_empty_equals_zero() {
    let digest = Digest::new();
    assert_eq!(digest.finalize(), 0);
}

#[test]
fn test_digest_with_initial() {
    let existing = crc32(0, b"prefix:");
    let mut digest = Digest::with_initial(existing);
    digest.update(b" suffix");
    assert_eq!(digest.finalize(), crc32(existing, b" suffix"));
}

#[test]
fn test_digest_reset() {
    let mut digest = Digest::new();
    digest.update(HELLO_WORLD);
    assert_ne!(digest.finalize(), 0);
    digest.reset();
    assert_eq!(digest.finalize(), 0);
}

#[test]
fn test_digest_incremental_many_chunks() {
    let data: Vec<u8> = (0u8..=255).cycle().take(4096).collect();
    let mut digest = Digest::new();
    for chunk in data.chunks(17) {
        digest.update(chunk);
    }
    assert_eq!(digest.finalize(), crc32(0, &data));
}

#[test]
fn test_dolit4() {
    let mut crc = 0u32;
    let buf = [0u8, 1u8, 2u8, 3u8];
    let buf4 = unsafe { std::slice::from_raw_parts(buf.as_ptr() as *const u32, 1) };
    let mut pos = 0;
    dolit4(&mut crc, buf4, &mut pos);
    assert_eq!(crc, 0xAAFD_590F);
    assert_eq!(pos, 1);
}

#[test]
fn test_dolit32() {
    let mut crc = 0u32;
    let buf = [0u8; 32];
    let buf4 = unsafe { std::slice::from_raw_parts(buf.as_ptr() as *const u32, 8) };
    let mut pos = 0;
    dolit32(&mut crc, buf4, &mut pos);
    assert_eq!(crc, 0);
    assert_eq!(pos, 8);
}

#[test]
fn test_slice_u8_as_u32_little_endian() {
    let bytes = [0u8, 1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8];
    let words = slice_u8_as_u32(&bytes);
    assert_eq!(words, &[0x0302_0100u32, 0x0706_0504u32]);
}

#[test]
fn test_slice_u8_as_u32_truncates_trailing() {
    let bytes = [0u8; 7];
    let words = slice_u8_as_u32(&bytes);
    assert_eq!(words.len(), 1);
}

#[test]
fn test_crc32_standard_check_value() {
    assert_eq!(crc32(0, b"123456789"), 0xCBF4_3926);
}
