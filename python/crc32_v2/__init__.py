"""crc32-rs Python package.

A fast CRC-32 library for Python, powered by Rust.

Functions
---------
crc32(data, initial_crc=0) -> int
    CRC-32 checksum compatible with zlib, PKZIP, Ethernet, and FDDI.

crc32_little(data, initial_crc=0) -> int
    Four-bytes-at-a-time little-endian CRC-32.

crc32_big(data, initial_crc=0) -> int
    Four-bytes-at-a-time big-endian CRC-32.

crc32_combine(crc1, crc2, len2) -> int
    Combine two CRC-32 values without holding the original data.

Classes
-------
Digest
    Streaming CRC-32 digest object.
"""

from ._crc32_v2 import (
    Digest,
    crc32,
    crc32_big,
    crc32_combine,
    crc32_little,
)

__all__ = [
    "Digest",
    "crc32",
    "crc32_big",
    "crc32_combine",
    "crc32_little",
]
