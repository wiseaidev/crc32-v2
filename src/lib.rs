/* crc32.c -- compute the CRC-32 of a data stream
 * Copyright (C) 1995-2006, 2010, 2011, 2012 Mark Adler
 * For conditions of distribution and use, see copyright notice in zlib.h
 *
 * Thanks to Rodney Brown <rbrown64@csc.com.au> for his contribution of faster
 * CRC methods: exclusive-oring 32 bits of data at a time, and pre-computing
 * tables for updating the shift register in one step with three exclusive-ors
 * instead of four steps with four exclusive-ors.  This results in about a
 * factor of two increase in speed on a Power PC G4 (PPC7455) using gcc -O3.
 */

mod crc32tables;

use crate::crc32tables::CRC_TABLE;

/* ========================================================================= */
// #define DO1 crc = CRC_TABLE[0][(crc ^ (*buf++)) & 0xff] ^ (crc >> 8)
// #define DO8 DO1; DO1; DO1; DO1; DO1; DO1; DO1; DO1

/// Computes the CRC-32 function over a block of data.  The caller provides
/// the previous CRC value as `start_crc` and the input data as `buf`.  The
/// function computes the CRC and returns it.
pub fn crc32(start_crc: u32, buf: &[u8]) -> u32 {
    let len = buf.len();
    let mut crc = start_crc;
    let mut bufpos: usize = 0; // index into buf
    let mut remaining_bytes = len;

    /*// #ifdef BYFOUR
        if (sizeof(void *) == sizeof(ptrdiff_t)) {
            z_crc_t endian;

            endian = 1;
            if (*((unsigned char *)(&endian)))
                return crc32_little(crc, buf, len);
            else
                return crc32_big(crc, buf, len);
        }
    // #endif /* BYFOUR */
    */
    crc = crc ^ 0xffffffff;
    // while (len >= 8) {
    //     DO8;
    //     len -= 8;
    // }

    let t0 = &CRC_TABLE[0];
    while remaining_bytes > 0 {
        let b = buf[bufpos];
        let b32 = b as u32;
        let b_index = (crc ^ b32) & 0xff;
        let t = t0[b_index as usize];
        crc = t ^ (crc >> 8);
        bufpos += 1;
        remaining_bytes -= 1;
    }

    crc ^ 0xffffffff
}

pub mod byfour {
    // use std::slice;
    use crate::crc32tables::CRC_TABLE;

    /* ========================================================================= */
    fn dolit4(c: &mut u32, buf4: &[u32], buf4pos: &mut usize) {
        let c1 = *c ^ buf4[*buf4pos];
        *buf4pos += 1;
        *c = CRC_TABLE[3][(c1 & 0xff) as usize]
            ^ CRC_TABLE[2][((c1 >> 8) & 0xff) as usize]
            ^ CRC_TABLE[1][((c1 >> 16) & 0xff) as usize]
            ^ CRC_TABLE[0][(c1 >> 24) as usize];
    }

    fn dolit32(c: &mut u32, buf4: &[u32], buf4pos: &mut usize) {
        dolit4(c, buf4, buf4pos);
        dolit4(c, buf4, buf4pos);
        dolit4(c, buf4, buf4pos);
        dolit4(c, buf4, buf4pos);
        dolit4(c, buf4, buf4pos);
        dolit4(c, buf4, buf4pos);
        dolit4(c, buf4, buf4pos);
        dolit4(c, buf4, buf4pos);
    }

    fn slice_u8_as_u32(s8: &[u8]) -> &[u32] {
        let len_u32 = s8.len() / 4;
        let ptr: *const u32 = s8.as_ptr() as *const u32;

        unsafe { std::slice::from_raw_parts(ptr, len_u32) }
    }

    // rusti: fn slice_u8_as_u32(s8: &[u8]) -> &[u32] { let new_len = s8.len() / 4;     unsafe { let ptr: *const u32 = s8.as_ptr() as *const u32; slice::from_raw_buf(ptr, new_len) } }

    /* ========================================================================= */
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

    /*
    /* ========================================================================= */
    #define DOBIG4 c ^= *++buf4; \
            c = CRC_TABLE[4][c & 0xff] ^ CRC_TABLE[5][(c >> 8) & 0xff] ^ \
                CRC_TABLE[6][(c >> 16) & 0xff] ^ CRC_TABLE[7][c >> 24]
    #define DOBIG32 DOBIG4; DOBIG4; DOBIG4; DOBIG4; DOBIG4; DOBIG4; DOBIG4; DOBIG4

    /* ========================================================================= */
    local unsigned long crc32_big(crc, buf, len)
        unsigned long crc;
        const unsigned char FAR *buf;
        unsigned len;
    {
        z_crc_t c;
        const z_crc_t FAR *buf4;

        c = ZSWAP32((z_crc_t)crc);
        c = ~c;
        while (len && ((ptrdiff_t)buf & 3)) {
            c = CRC_TABLE[4][(c >> 24) ^ *buf++] ^ (c << 8);
            len--;
        }

        buf4 = (const z_crc_t FAR *)(const void FAR *)buf;
        buf4--;
        while (len >= 32) {
            DOBIG32;
            len -= 32;
        }
        while (len >= 4) {
            DOBIG4;
            len -= 4;
        }
        buf4++;
        buf = (const unsigned char FAR *)buf4;

        if (len) do {
            c = CRC_TABLE[4][(c >> 24) ^ *buf++] ^ (c << 8);
        } while (--len);
        c = ~c;
        return (unsigned long)(ZSWAP32(c));
    }

    #endif /* BYFOUR */

    #define GF2_DIM 32      /* dimension of GF(2) vectors (length of CRC) */

    /* ========================================================================= */
    local unsigned long gf2_matrix_times(mat, vec)
        unsigned long *mat;
        unsigned long vec;
    {
        unsigned long sum;

        sum = 0;
        while (vec) {
            if (vec & 1)
                sum ^= *mat;
            vec >>= 1;
            mat++;
        }
        return sum;
    }

    /* ========================================================================= */
    local void gf2_matrix_square(square, mat)
        unsigned long *square;
        unsigned long *mat;
    {
        int n;

        for (n = 0; n < GF2_DIM; n++)
            square[n] = gf2_matrix_times(mat, mat[n]);
    }

    /* ========================================================================= */
    local uLong crc32_combine_(crc1, crc2, len2)
        uLong crc1;
        uLong crc2;
        z_off64_t len2;
    {
        int n;
        unsigned long row;
        unsigned long even[GF2_DIM];    /* even-power-of-two zeros operator */
        unsigned long odd[GF2_DIM];     /* odd-power-of-two zeros operator */

        /* degenerate case (also disallow negative lengths) */
        if (len2 <= 0)
            return crc1;

        /* put operator for one zero bit in odd */
        odd[0] = 0xedb88320UL;          /* CRC-32 polynomial */
        row = 1;
        for (n = 1; n < GF2_DIM; n++) {
            odd[n] = row;
            row <<= 1;
        }

        /* put operator for two zero bits in even */
        gf2_matrix_square(even, odd);

        /* put operator for four zero bits in odd */
        gf2_matrix_square(odd, even);

        /* apply len2 zeros to crc1 (first square will put the operator for one
           zero byte, eight zero bits, in even) */
        do {
            /* apply zeros operator for this bit of len2 */
            gf2_matrix_square(even, odd);
            if (len2 & 1)
                crc1 = gf2_matrix_times(even, crc1);
            len2 >>= 1;

            /* if no more bits set, then done */
            if (len2 == 0)
                break;

            /* another iteration of the loop with odd and even swapped */
            gf2_matrix_square(odd, even);
            if (len2 & 1)
                crc1 = gf2_matrix_times(odd, crc1);
            len2 >>= 1;

            /* if no more bits set, then done */
        } while (len2 != 0);

        /* return combined crc */
        crc1 ^= crc2;
        return crc1;
    }

    /* ========================================================================= */
    uLong ZEXPORT crc32_combine(crc1, crc2, len2)
        uLong crc1;
        uLong crc2;
        z_off_t len2;
    {
        return crc32_combine_(crc1, crc2, len2);
    }

    uLong ZEXPORT crc32_combine64(crc1, crc2, len2)
        uLong crc1;
        uLong crc2;
        z_off64_t len2;
    {
        return crc32_combine_(crc1, crc2, len2);
    }

    */
}
