const TBLS: usize = 8;

/*
  Generate tables for a byte-wise 32-bit CRC calculation on the polynomial:
  x^32+x^26+x^23+x^22+x^16+x^12+x^11+x^10+x^8+x^7+x^5+x^4+x^2+x+1.

  Polynomials over GF(2) are represented in binary, one bit per coefficient,
  with the lowest powers in the most significant bit.  Then adding polynomials
  is just exclusive-or, and multiplying a polynomial by x is a right shift by
  one.  If we call the above polynomial p, and represent a byte as the
  polynomial q, also with the lowest power in the most significant bit (so the
  byte 0xb1 is the polynomial x^7+x^3+x+1), then the CRC is (q*x^32) mod p,
  where a mod b means the remainder after dividing a by b.

  This calculation is done using the shift-register method of multiplying and
  taking the remainder.  The register is initialized to zero, and for each
  incoming bit, x^32 is added mod p to the register if the bit is a one (where
  x^32 mod p is p+x^32 = x^26+...+1), and the register is multiplied mod p by
  x (which is shifting right by one and adding x^32 mod p if the bit shifted
  out is a one).  We start with the highest power (least significant bit) of
  q and repeat for all eight bits of q.

  The first table is simply the CRC of all possible eight bit values.  This is
  all the information needed to generate CRCs on data a byte at a time for all
  combinations of CRC register values and incoming bytes.  The remaining tables
  allow for word-at-a-time CRC calculation for both big-endian and little-
  endian machines, where a word is four bytes.
*/

pub type CrcTable = [[u32; 0x100]; TBLS];

pub fn make_crc_table() -> CrcTable {
    /* terms of polynomial defining this crc (except x^32): */
    let p: [u8; 14] = [0, 1, 2, 4, 5, 7, 8, 10, 11, 12, 16, 22, 23, 26]; // 14 terms

    /* make exclusive-or pattern from polynomial (0xedb88320UL) */
    let mut poly: u32 = 0; /* polynomial exclusive-or pattern */
    for term in p.iter() {
        poly |= 1u32 << (31 - *term as usize);
    }

    // local z_crc_t FAR crc_table[TBLS][256];
    let mut crc_table: [[u32; 0x100]; TBLS] = [[0; 0x100]; TBLS];

    /* generate a crc for every 8-bit value */
    for n in 0..0x100 {
        let mut c = n as u32;
        for _ in 0..8 {
            c = if (c & 1) != 0 {
                poly ^ (c >> 1)
            } else {
                c >> 1
            }
        }
        crc_table[0][n] = c;
    }

    // #ifdef BYFOUR
    /* generate crc for each value followed by one, two, and three zeros,
    and then the byte reversal of those as well as the first table */
    for n in 0..0x100 {
        let mut c: u32 = crc_table[0][n];
        crc_table[4][n] = zswap32(c);
        for k in 1..3 {
            c = crc_table[0][c as usize & 0xff] ^ (c >> 8);
            crc_table[k][n] = c;
            crc_table[k + 4][n] = zswap32(c);
        }
    }
    // #endif /* BYFOUR */
    crc_table
}

pub fn zswap32(n: u32) -> u32 {
    (n << 24)                       // dd
    | ((n << 8) & 0x00ff0000u32)    // cc
    | ((n >> 8) & 0x0000ff00u32)    // bb
    | ((n >> 24) & 0x000000ffu32) // aa
}

pub fn write_tables(crc_table: &CrcTable) -> String {
    /* write out CRC tables to crc32.h */
    let mut s: String = String::new();

    let tt = crc_table;
    // let &CrcTable(ref tt) = &crc_table; // match crc_table { &CrcTable(ref t) => t };

    s.push_str("/* crc32tables.rs -- tables for rapid CRC calculation\n");
    s.push_str(" * Generated automatically by crc32gen.rs\n */\n\n");
    s.push_str("pub static CRC_TABLE:[[u32; 0x100]; 8] = [\n  [\n");
    write_table(&mut s, &tt[0]);
    // #  ifdef BYFOUR
    s.push_str("// #ifdef BYFOUR\n");
    for item in tt.iter().take(8).skip(1) {
        s.push_str("  ],\n [\n");
        write_table(&mut s, item);
    }
    s.push_str("// #endif\n");
    // #  endif /* BYFOUR */
    s.push_str("  ]\n];\n");
    s
    // #endif /* MAKECRCH */
}

fn write_table(s: &mut String, table: &[u32; 0x100]) {
    for (n, item) in table.iter().enumerate().take(0x100) {
        let line = format!(
            "{}0x{:08x}{}",
            if n % 5 != 0 { "" } else { "    " },
            item,
            if n == 255 {
                "\n"
            } else if n % 5 == 4 {
                ",\n"
            } else {
                ", "
            }
        );
        s.push_str(line.as_str());
    }
}
