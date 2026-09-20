# Copyright 2026 Mahmoud Harmouch.
#
# Licensed under the MIT license
# <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
# option. This file may not be copied, modified, or distributed
# except according to those terms.
# pylint: disable=missing-function-docstring,redefined-outer-name

import pytest

# pyrefly: ignore [missing-import]
from crc32_rs import (
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

HELLO_WORLD = b"Hello, world!"
CHECK_VECTOR = b"123456789"
CHECK_CRC = 0xCBF43926
HELLO_CRC = 0xEBE6C6E6


def test_version_format():
    parts = __version__.split(".")
    assert len(parts) == 3
    assert all(p.isdigit() for p in parts)


def test_crc32_empty():
    assert crc32(b"") == 0


def test_crc32_known_hello():
    assert crc32(HELLO_WORLD) == HELLO_CRC


def test_crc32_check_vector():
    assert crc32(CHECK_VECTOR) == CHECK_CRC


def test_crc32_chaining_equals_single_shot():
    assert crc32(b"world!", crc32(b"Hello, ")) == crc32(HELLO_WORLD)


def test_crc32_bytes_known():
    result = crc32_bytes(HELLO_WORLD)
    assert isinstance(result, bytes)
    assert len(result) == 4
    assert int.from_bytes(result, "big") == HELLO_CRC


def test_crc32_bytes_empty():
    result = crc32_bytes(b"")
    assert result == b"\x00\x00\x00\x00"


def test_crc32_hex_known():
    assert crc32_hex(HELLO_WORLD) == "ebe6c6e6"


def test_crc32_hex_check_vector():
    assert crc32_hex(CHECK_VECTOR) == "cbf43926"


def test_crc32_hex_empty():
    assert crc32_hex(b"") == "00000000"


def test_crc32_hex_is_lowercase():
    h = crc32_hex(HELLO_WORLD)
    assert h == h.lower()
    assert len(h) == 8


def test_crc32_little_known():
    assert crc32_little(HELLO_WORLD) == HELLO_CRC


def test_crc32_little_empty():
    assert crc32_little(b"") == 0


def test_crc32_little_8_known():
    assert crc32_little_8(HELLO_WORLD) == HELLO_CRC


def test_crc32_little_8_empty():
    assert crc32_little_8(b"") == 0


def test_crc32_little_16_known():
    assert crc32_little_16(HELLO_WORLD) == HELLO_CRC


def test_crc32_little_16_empty():
    assert crc32_little_16(b"") == 0


def test_crc32_little_16_check_vector():
    assert crc32_little_16(CHECK_VECTOR) == CHECK_CRC


def test_crc32_big_empty():
    assert crc32_big(b"") == 0


def test_crc32_big_chaining():
    full = crc32_big(HELLO_WORLD)
    chained = crc32_big(b"world!", crc32_big(b"Hello, "))
    assert full == chained


def test_crc32_combine_basic():
    c1 = crc32(b"Hello, ")
    c2 = crc32(b"world!")
    combined = crc32_combine(c1, c2, len(b"world!"))
    assert combined == crc32(HELLO_WORLD)


def test_crc32_combine_zero_length():
    c = crc32(b"data")
    assert crc32_combine(c, 0, 0) == c


def test_digest_empty():
    d = Digest()
    assert d.finalize() == 0


def test_digest_known():
    d = Digest()
    d.update(HELLO_WORLD)
    assert d.finalize() == HELLO_CRC


def test_digest_incremental():
    d = Digest()
    d.update(b"Hello, ")
    d.update(b"world!")
    assert d.finalize() == HELLO_CRC


def test_digest_reset():
    d = Digest()
    d.update(HELLO_WORLD)
    d.reset()
    assert d.finalize() == 0


def test_digest_bytes():
    d = Digest()
    d.update(HELLO_WORLD)
    result = d.digest()
    assert isinstance(result, bytes)
    assert len(result) == 4
    assert int.from_bytes(result, "big") == HELLO_CRC


def test_digest_repr():
    d = Digest()
    d.update(HELLO_WORLD)
    r = repr(d)
    assert r.startswith("Digest(crc=0x")
    assert "EBE6C6E6" in r.upper()


def test_digest_with_initial():
    existing = crc32(b"Hello, ")
    d = Digest.with_initial(existing)
    d.update(b"world!")
    assert d.finalize() == crc32(HELLO_WORLD)
