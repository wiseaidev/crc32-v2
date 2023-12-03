use crate::crc32tables::CRC_TABLE;

/// This function updates the CRC32 checksum with the next 4 bytes from the buffer.
///
/// # Arguments
/// * `c` -  a mutable reference to the current CRC32 checksum
/// * `buf4` - a slice containing the next 4 bytes from the input buffer
/// * `buf4pos` - a mutable reference to the index into the `buf4` slice
///
/// # Examples
/// ```
/// use crc32_v2::byfour::dolit4;
///
/// let mut crc = 0u32;
/// let buf = [0u8, 1u8, 2u8, 3u8];
/// let buf4 = unsafe { std::slice::from_raw_parts(buf.as_ptr() as *const u32, 1) };
/// let mut buf4pos = 0;
/// dolit4(&mut crc, buf4, &mut buf4pos);
/// assert_eq!(crc, 0xAAFD590F);
/// ```
pub fn dolit4(c: &mut u32, buf4: &[u32], buf4pos: &mut usize) {
    let c1 = *c ^ buf4[*buf4pos];
    *buf4pos += 1;
    *c = CRC_TABLE[3][(c1 & 0xff) as usize]
        ^ CRC_TABLE[2][((c1 >> 8) & 0xff) as usize]
        ^ CRC_TABLE[1][((c1 >> 16) & 0xff) as usize]
        ^ CRC_TABLE[0][(c1 >> 24) as usize];
}

/// This function updates the CRC32 checksum with the next 32 bytes from the buffer.
///
/// # Arguments
/// * `c` -  a mutable reference to the current CRC32 checksum
/// * `buf4` - a slice containing the next 32 bytes from the input buffer
/// * `buf4pos` - a mutable reference to the index into the `buf4` slice
///
/// # Examples
/// ```
/// use crc32_v2::byfour::dolit32;
///
/// let mut crc = 0u32;
/// let buf = [0u8; 32];
/// let buf4 = unsafe { std::slice::from_raw_parts(buf.as_ptr() as *const u32, 8) };
/// let mut buf4pos = 0;
/// dolit32(&mut crc, buf4, &mut buf4pos);
/// assert_eq!(crc, 0);
/// ```
pub fn dolit32(c: &mut u32, buf4: &[u32], buf4pos: &mut usize) {
    dolit4(c, buf4, buf4pos);
    dolit4(c, buf4, buf4pos);
    dolit4(c, buf4, buf4pos);
    dolit4(c, buf4, buf4pos);
    dolit4(c, buf4, buf4pos);
    dolit4(c, buf4, buf4pos);
    dolit4(c, buf4, buf4pos);
    dolit4(c, buf4, buf4pos);
}

/// This function converts a slice of u8 into a slice of u32.
///
/// # Arguments
/// * `s8` -  a slice of u8 bytes
///
/// # Returns
/// (`&[u32]`): a slice of u32 corresponding to the input slice of u8
///
/// # Safety
/// The function uses unsafe code to reinterpret the memory layout of the u8 slice as u32.
///
/// # Examples
/// ```
/// use crc32_v2::byfour::slice_u8_as_u32;
///
/// let bytes = [0u8, 1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8];
/// let u32_slice = slice_u8_as_u32(&bytes);
/// assert_eq!(u32_slice, &[50462976u32, 117835012u32]);
/// ```
pub fn slice_u8_as_u32(s8: &[u8]) -> &[u32] {
    let len_u32 = s8.len() / 4;
    let ptr: *const u32 = s8.as_ptr() as *const u32;

    unsafe { std::slice::from_raw_parts(ptr, len_u32) }
}

/// This function calculates the CRC32 checksum of a byte buffer in little-endian format.
///
/// # Arguments
/// * `crc` -  the initial CRC32 value (usually 0)
/// * `buf` -  a slice containing the input bytes
///
/// # Returns
/// (`u32`): the CRC32 checksum of the input buffer
///
/// # Examples
/// ```
/// use crc32_v2::byfour::crc32_little;
///
/// let crc = crc32_little(0, &[0u8, 1u8, 2u8, 3u8]);
/// assert_eq!(crc, 0x55BC80E2);
/// ```
pub fn crc32_little(crc: u32, buf: &[u8]) -> u32 {
    let mut len = buf.len();
    let mut bufpos = 0; // index into buf

    let mut c: u32 = crc;
    c = !c;

    let mut buf_align_bits = (buf.as_ptr() as usize) & 3;
    while len != 0 && (buf_align_bits & 3) != 0 {
        let b = buf[bufpos];
        let bi = (c & 0xff) as u8 ^ b;
        c = CRC_TABLE[0][bi as usize] ^ (c >> 8);
        buf_align_bits += 1;
        bufpos += 1;
        len -= 1;
    }

    let buf4 = slice_u8_as_u32(&buf[bufpos..]);
    let mut buf4pos: usize = 0;
    while len >= 32 {
        dolit32(&mut c, buf4, &mut buf4pos);
        len -= 32;
    }
    while len >= 4 {
        dolit4(&mut c, buf4, &mut buf4pos);
        len -= 4;
    }

    // now handle trailing bytes

    bufpos += buf4pos * 4;

    if len != 0 {
        loop {
            let b = buf[bufpos];
            let bi = (c & 0xff) as u8 ^ b;
            c = CRC_TABLE[0][bi as usize] ^ (c >> 8);
            bufpos += 1;
            len -= 1;
            if len == 0 {
                break;
            }
        }
    }
    c = !c;
    return c;
}
