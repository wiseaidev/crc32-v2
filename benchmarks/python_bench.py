#!/usr/bin/env python3
"""
CRC-32 Python vs Rust comparison benchmark.

Measures zlib.crc32 and crcmod at multiple payload sizes using timeit,
then compares against the crc32_rs (crc32-v2) Python bindings if installed.

Usage:
    python3 benchmarks/python_bench.py

Requirements:
    pip install crcmod           # always needed
    pip install crc32-rs         # optional: installs crc32-v2 Python bindings
"""

import timeit
import zlib
import sys


SIZES = [1, 64, 1_024, 65_536, 1_048_576]
REPEAT = 5
NUMBER_SMALL = 100_000
NUMBER_LARGE = 1_000


def make_data(size: int) -> bytes:
    return bytes(range(256)) * (size // 256 + 1)


def ns_per_op(elapsed_seconds: float, number: int) -> float:
    return (elapsed_seconds / number) * 1e9


def mib_per_s(size: int, elapsed_seconds: float, number: int) -> float:
    total_bytes = size * number
    return total_bytes / elapsed_seconds / (1024 * 1024)


def bench(fn, size: int) -> tuple[float, float]:
    number = NUMBER_SMALL if size <= 1024 else NUMBER_LARGE
    times = timeit.repeat(fn, repeat=REPEAT, number=number)
    best = min(times)
    return ns_per_op(best, number), mib_per_s(size, best, number)


def header(title: str) -> None:
    print(f"\n{'=' * 72}")
    print(f"  {title}")
    print(f"{'=' * 72}")
    print(f"  {'Size':>10}  {'ns/op':>12}  {'MiB/s':>10}  {'Library'}")
    print(f"  {'-' * 10}  {'-' * 12}  {'-' * 10}  {'-' * 20}")


def row(size: int, ns: float, mbs: float, label: str) -> None:
    size_str = (
        f"{size} B" if size < 1024
        else f"{size // 1024} KiB" if size < 1_048_576
        else f"{size // 1_048_576} MiB"
    )
    print(f"  {size_str:>10}  {ns:>12.1f}  {mbs:>10.1f}  {label}")


def bench_zlib() -> dict[int, tuple[float, float]]:
    header("Python zlib.crc32")
    results = {}
    for size in SIZES:
        data = make_data(size)
        ns, mbs = bench(lambda d=data: zlib.crc32(d), size)
        row(size, ns, mbs, "zlib.crc32")
        results[size] = (ns, mbs)
    return results


def bench_crcmod() -> dict[int, tuple[float, float]] | None:
    try:
        import crcmod
        crc32_fn = crcmod.predefined.mkCrcFun("crc-32")
    except ImportError:
        print("\n  crcmod not installed, skipping (pip install crcmod)")
        return None

    header("Python crcmod (crc-32 preset)")
    results = {}
    for size in SIZES:
        data = make_data(size)
        ns, mbs = bench(lambda d=data: crc32_fn(d), size)
        row(size, ns, mbs, "crcmod")
        results[size] = (ns, mbs)
    return results


def bench_crc32_rs() -> dict[int, tuple[float, float]] | None:
    try:
        import crc32_rs
    except ImportError:
        print("\n  crc32-rs not installed, skipping (pip install crc32-rs)")
        print("  Or build locally: maturin develop --features python")
        return None

    header("crc32-rs (crc32_little_16, Rust slicing-by-16, ~1.3 GiB/s)")
    results = {}
    for size in SIZES:
        data = make_data(size)
        ns, mbs = bench(lambda d=data: crc32_rs.crc32_little_16(d), size)
        row(size, ns, mbs, "crc32_rs.crc32_little_16")
        results[size] = (ns, mbs)
    return results


def print_comparison(
    zlib_r: dict,
    crcmod_r: dict | None,
    rs_r: dict | None,
) -> None:
    header("Speedup: crc32-rs (slicing-by-16) vs Python alternatives")
    print(f"  {'Size':>10}  {'vs zlib':>12}  {'vs crcmod':>12}")
    print(f"  {'-' * 10}  {'-' * 12}  {'-' * 12}")

    if rs_r is None:
        print("  (crc32-rs not available, install with: pip install crc32-rs)")
        return

    for size in SIZES:
        size_str = (
            f"{size} B" if size < 1024
            else f"{size // 1024} KiB" if size < 1_048_576
            else f"{size // 1_048_576} MiB"
        )
        rs_ns = rs_r[size][0]
        zlib_speedup = zlib_r[size][0] / rs_ns
        crc_speedup = (crcmod_r[size][0] / rs_ns) if crcmod_r else 0.0
        crc_str = f"{crc_speedup:>10.1f}×" if crcmod_r else "         N/A"
        print(f"  {size_str:>10}  {zlib_speedup:>10.1f}×  {crc_str}")


def main() -> None:
    print("CRC-32 Python vs Rust Benchmark")
    print(f"Python {sys.version.split()[0]}\n")
    print("Methodology: timeit, best of 5 runs.")
    print("  Sizes ≤ 1 KiB: 100 000 iterations per run.")
    print("  Sizes > 1 KiB:   1 000 iterations per run.")

    zlib_results = bench_zlib()
    crcmod_results = bench_crcmod()
    rs_results = bench_crc32_rs()

    print_comparison(zlib_results, crcmod_results, rs_results)

if __name__ == "__main__":
    main()
