Name:           crc32-v2
Version:        0.1.2
Release:        1%{?dist}
Summary:        Blazingly fast CRC-32 Rust library with slicing-by-16
License:        MIT
URL:            https://github.com/wiseaidev/crc32-v2
Source0:        %{name}-%{version}.tar.gz

BuildRequires:  rust cargo

%description
crc32-v2 provides a pure-software, 100%-safe-Rust CRC-32 implementation
using slicing-by-16 tables for maximum throughput (~1.3 GiB/s), with zero
heap allocation in the hot path. Native Python and Node.js bindings included.

%prep
%autosetup

%build
cargo build --release

%install
mkdir -p %{buildroot}%{_libdir}
cp target/release/libcrc32_v2.so %{buildroot}%{_libdir}/ 2>/dev/null || true

%files
%license LICENSE
%doc README.md
%{_libdir}/libcrc32_v2.so

%changelog
* Sat Sep 20 2026 Mahmoud Harmouch <oss@wiseai.dev> - 0.1.2-1
- Initial RPM release.
- Slicing-by-16 for ~1.3 GiB/s throughput.
- Zero-alloc hot path with fat LTO.
- crc32-codegen build-time table generation subcrate.
- Python (crc32-rs) and Node.js (crc32-rs) native bindings.
