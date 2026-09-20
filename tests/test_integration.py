# Copyright 2026 Mahmoud Harmouch.
#
# Licensed under the MIT license
# <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
# option. This file may not be copied, modified, or distributed
# except according to those terms.
# pylint: disable=missing-function-docstring,redefined-outer-name

import sys

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

HELLO = b"Hello, "
WORLD = b"world!"
HELLO_WORLD = b"Hello, world!"
CHECK_VECTOR = b"123456789"
CHECK_CRC = 0xCBF43926
HELLO_CRC = 0xEBE6C6E6


@pytest.fixture
def empty_digest():
    return Digest()


@pytest.fixture
def hello_world_digest():
    d = Digest()
    d.update(HELLO_WORLD)
    return d


def test_version_is_string():
    assert isinstance(__version__, str)


def test_version_has_three_parts():
    parts = __version__.split(".")
    assert len(parts) == 3
    assert all(p.isdigit() for p in parts)


def test_crc32_empty_is_zero():
    assert crc32(b"") == 0


def test_crc32_single_zero_byte():
    assert crc32(b"\x00") != 0


def test_crc32_all_zeros_1k():
    assert isinstance(crc32(b"\x00" * 1024), int)


def test_crc32_check_vector():
    assert crc32(CHECK_VECTOR) == CHECK_CRC


def test_crc32_hello_world():
    assert crc32(HELLO_WORLD) == HELLO_CRC


def test_crc32_chaining():
    assert crc32(WORLD, crc32(HELLO)) == crc32(HELLO_WORLD)


def test_crc32_chaining_three_parts():
    a, b, c = b"abc", b"def", b"ghi"
    assert crc32(c, crc32(b, crc32(a))) == crc32(a + b + c)


def test_crc32_returns_u32():
    result = crc32(HELLO_WORLD)
    assert 0 <= result <= 0xFFFFFFFF


def test_crc32_bytes_type():
    assert isinstance(crc32_bytes(HELLO_WORLD), bytes)


def test_crc32_bytes_length():
    assert len(crc32_bytes(HELLO_WORLD)) == 4


def test_crc32_bytes_big_endian():
    result = crc32_bytes(HELLO_WORLD)
    assert int.from_bytes(result, "big") == HELLO_CRC


def test_crc32_bytes_matches_crc32():
    data = b"some random data for testing"
    assert int.from_bytes(crc32_bytes(data), "big") == crc32(data)


def test_crc32_bytes_empty():
    assert crc32_bytes(b"") == b"\x00\x00\x00\x00"


def test_crc32_bytes_chaining():
    result = crc32_bytes(WORLD, int.from_bytes(crc32_bytes(HELLO), "big"))
    assert int.from_bytes(result, "big") == HELLO_CRC


def test_crc32_hex_type():
    assert isinstance(crc32_hex(HELLO_WORLD), str)


def test_crc32_hex_length():
    assert len(crc32_hex(HELLO_WORLD)) == 8


def test_crc32_hex_lowercase():
    h = crc32_hex(HELLO_WORLD)
    assert h == h.lower()


def test_crc32_hex_known():
    assert crc32_hex(HELLO_WORLD) == "ebe6c6e6"


def test_crc32_hex_check_vector():
    assert crc32_hex(CHECK_VECTOR) == "cbf43926"


def test_crc32_hex_empty():
    assert crc32_hex(b"") == "00000000"


def test_crc32_hex_matches_crc32():
    data = b"integration test data"
    assert crc32_hex(data) == f"{crc32(data):08x}"


def test_all_little_variants_agree_hello():
    assert crc32(HELLO_WORLD) == crc32_little(HELLO_WORLD)
    assert crc32(HELLO_WORLD) == crc32_little_8(HELLO_WORLD)
    assert crc32(HELLO_WORLD) == crc32_little_16(HELLO_WORLD)


@pytest.mark.parametrize("size", [1, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 65536])
def test_all_little_variants_agree_varied_sizes(size):
    data = bytes(range(256)) * (size // 256 + 1)
    data = data[:size]
    ref = crc32(data)
    assert crc32_little(data) == ref
    assert crc32_little_8(data) == ref
    assert crc32_little_16(data) == ref


def test_crc32_big_empty():
    assert crc32_big(b"") == 0


def test_crc32_big_chaining():
    assert crc32_big(WORLD, crc32_big(HELLO)) == crc32_big(HELLO_WORLD)


def test_crc32_combine_adjacent():
    c1 = crc32(HELLO)
    c2 = crc32(WORLD)
    assert crc32_combine(c1, c2, len(WORLD)) == crc32(HELLO_WORLD)


def test_crc32_combine_zero_len2():
    c = crc32(b"data")
    assert crc32_combine(c, 0, 0) == c


@pytest.mark.parametrize("split", [1, 2, 5, 7, 10, 12])
def test_crc32_combine_multiple_splits(split):
    data = HELLO_WORLD
    c1 = crc32(data[:split])
    c2 = crc32(data[split:])
    assert crc32_combine(c1, c2, len(data) - split) == crc32(data)


def test_digest_starts_at_zero(empty_digest):
    assert empty_digest.finalize() == 0


def test_digest_update_single(empty_digest):
    empty_digest.update(HELLO_WORLD)
    assert empty_digest.finalize() == HELLO_CRC


def test_digest_update_incremental():
    d = Digest()
    d.update(HELLO)
    d.update(WORLD)
    assert d.finalize() == HELLO_CRC


def test_digest_finalize_does_not_reset(hello_world_digest):
    v1 = hello_world_digest.finalize()
    v2 = hello_world_digest.finalize()
    assert v1 == v2


def test_digest_reset(hello_world_digest):
    hello_world_digest.reset()
    assert hello_world_digest.finalize() == 0


def test_digest_reset_and_reuse(hello_world_digest):
    hello_world_digest.reset()
    hello_world_digest.update(HELLO_WORLD)
    assert hello_world_digest.finalize() == HELLO_CRC


def test_digest_bytes_type(hello_world_digest):
    assert isinstance(hello_world_digest.digest(), bytes)


def test_digest_bytes_length(hello_world_digest):
    assert len(hello_world_digest.digest()) == 4


def test_digest_bytes_matches_finalize(hello_world_digest):
    result = hello_world_digest.digest()
    assert int.from_bytes(result, "big") == hello_world_digest.finalize()


def test_digest_repr(hello_world_digest):
    r = repr(hello_world_digest)
    assert r.startswith("Digest(crc=0x")
    assert r.endswith(")")


def test_digest_with_initial():
    existing = crc32(HELLO)
    d = Digest.with_initial(existing)
    d.update(WORLD)
    assert d.finalize() == crc32(HELLO_WORLD)


def test_digest_matches_single_shot():
    d = Digest()
    for chunk in [b"a", b"bc", b"def", b"ghij"]:
        d.update(chunk)
    assert d.finalize() == crc32(b"abcdefghij")


@pytest.mark.parametrize("size", [1, 64, 1024, 65536])
def test_digest_large_payloads(size):
    data = bytes(i % 256 for i in range(size))
    d = Digest()
    d.update(data)
    assert d.finalize() == crc32(data)


def test_crc32_bytes_and_hex_consistent():
    data = b"consistency check"
    b_result = int.from_bytes(crc32_bytes(data), "big")
    h_result = int(crc32_hex(data), 16)
    assert b_result == h_result == crc32(data)
