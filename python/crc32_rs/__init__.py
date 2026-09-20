# Copyright 2026 Mahmoud Harmouch.
#
# Licensed under the MIT license
# <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
# option. This file may not be copied, modified, or distributed
# except according to those terms.

from crc32_rs._crc32_v2 import (
    Digest,
    __version__,
    crc32,
    crc32_big,
    crc32_bytes,
    crc32_combine,
    crc32_hex,
    crc32_little,
    crc32_little_16,
    crc32_little_8,
)

FrozenCrc = Digest

__all__ = [
    "__version__",
    "crc32",
    "crc32_bytes",
    "crc32_hex",
    "crc32_little",
    "crc32_little_8",
    "crc32_little_16",
    "crc32_big",
    "crc32_combine",
    "Digest",
    "FrozenCrc",
]
