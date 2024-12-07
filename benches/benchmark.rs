use crc32_v2::crc32tables::CRC_TABLE;
use crc32fast::Hasher;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

#[inline]
pub fn crc32_while_loop(start_crc: u32, buf: &[u8]) -> u32 {
    // Initialize variables
    let len = buf.len();
    let mut crc = start_crc;
    let mut bufpos: usize = 0;
    let mut remaining_bytes = len;

    // XOR with 0xffffffff as specified in CRC32 algorithm
    crc = crc ^ 0xffffffff;

    // Reference to the first CRC table for faster access
    let t0 = &CRC_TABLE[0];

    // Process each byte in the buffer
    while remaining_bytes > 0 {
        let b = buf[bufpos];
        let b32 = b as u32;
        let b_index = (crc ^ b32) & 0xff;
        let t = t0[b_index as usize];
        crc = t ^ (crc >> 8);
        bufpos += 1;
        remaining_bytes -= 1;
    }

    // XOR again with 0xffffffff as specified in CRC32 algorithm
    crc ^ 0xffffffff
}

#[inline]
pub fn crc32_for_loop(start_crc: u32, buf: &[u8]) -> u32 {
    // XOR with 0xffffffff as specified in the CRC32 algorithm
    let mut crc = start_crc ^ 0xffffffff;

    // Reference to the first CRC table for faster access
    let t0 = &CRC_TABLE[0];

    // Process each byte in the buffer
    for &byte in buf {
        let b_index = (crc ^ (byte as u32)) & 0xff;
        crc = t0[b_index as usize] ^ (crc >> 8);
    }

    crc ^ 0xffffffff
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function(
        "`crc32-v2` crc32_while_loop(0, b\"hello\") performance:",
        |b| b.iter(|| crc32_while_loop(black_box(0), black_box(b"hello"))),
    );
    c.bench_function(
        "`crc32-v2` crc32_for_loop(0, b\"hello\") performance:",
        |b| b.iter(|| crc32_for_loop(black_box(0), black_box(b"hello"))),
    );
    c.bench_function(
        "`crc32fast` crc32fast::hash(b\"hello\") performance:",
        |b| b.iter(|| crc32fast::hash(black_box(b"hello"))),
    );
    c.bench_function(
        "`crc32fast` crc32fast::Hasher::new().update(b\"hello\").finalize() performance:",
        |b| {
            b.iter(|| {
                let mut hasher = black_box(Hasher::new());
                hasher.update(black_box(b"foo bar baz"));
                let _checksum = hasher.finalize();
            })
        },
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
