# crc32-v2 Packaging Guide (Debian / RPM)

This document explains how to build and install native `.deb` and `.rpm`
packages for the `crc32-v2` Rust library.

> [!NOTE]
> Debian and RPM packages bundle the `crc32-v2` Rust library only. The
> Python (`crc32-rs`) and Node.js (`crc32-rs`) packages are distributed
> separately via PyPI and npm.

## 🏗 Debian / Ubuntu

### Prerequisites

```sh
sudo apt-get install -y build-essential debhelper devscripts pkg-config libssl-dev
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Build the `.deb`

```sh
git clone https://github.com/wiseaidev/crc32-v2.git
cd crc32-v2
debuild -d --preserve-envvar PATH -us -uc -b
ls ../*.deb
```

### Install

```sh
sudo dpkg -i ../crc32-v2_0.1.2_amd64.deb
```

### Verify

```sh
dpkg -s crc32-v2
```

## 🏗 RHEL / Fedora

### Prerequisites

```sh
sudo dnf install -y rpm-build gcc openssl-devel pkg-config
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Prepare the RPM environment

```sh
mkdir -p ~/rpmbuild/{BUILD,BUILDROOT,RPMS,SOURCES,SPECS,SRPMS}
cp rpm/crc32-v2.spec ~/rpmbuild/SPECS/

VERSION=$(grep -m1 '^version =' Cargo.toml | sed 's/.*"\(.*\)".*/\1/')

tar -czvf ~/rpmbuild/SOURCES/crc32-v2-${VERSION}.tar.gz \
  --transform "s,^\.,crc32-v2-${VERSION}," \
  --exclude=.git .
```

### Build the `.rpm`

```sh
rpmbuild -bb ~/rpmbuild/SPECS/crc32-v2.spec
ls ~/rpmbuild/RPMS/x86_64/
```

### Install

```sh
sudo rpm -ivh ~/rpmbuild/RPMS/x86_64/crc32-v2-0.1.2-1.x86_64.rpm
```

### Verify

```sh
rpm -qi crc32-v2
```

## 📦 Pre-built Packages (GitHub Releases)

Pre-built `.deb` and `.rpm` packages are attached to every
[GitHub Release](https://github.com/wiseaidev/crc32-v2/releases).

Download with `gh`:

```sh
gh release download v0.1.2 --pattern '*.deb'
gh release download v0.1.2 --pattern '*.rpm'
```

## 🔗 See Also

- [Debian Policy Manual](https://www.debian.org/doc/debian-policy/)
- [RPM Packaging Guide](https://rpm-packaging-guide.github.io/)
- [linux-publish.yml](https://github.com/wiseaidev/crc32-v2/blob/main/.github/workflows/linux-publish.yml): CI workflow
