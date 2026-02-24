# Packaging Notes

This document is intended for package maintainers.

## Arch Linux (AUR)

### Manual build

```bash
cd pkg
makepkg -si
```

### Clean chroot build (recommended for verification)

```bash
# Install devtools if not present
sudo pacman -S devtools

# Build in a clean chroot
cd pkg
extra-x86_64-build
```

### Verify with namcap

```bash
cd pkg
namcap PKGBUILD
namcap stardial-*.pkg.tar.zst
```

### Runtime dependencies

- `gcc-libs` (glibc/libgcc — dynamically linked)

### Build dependencies

- `cargo` (Rust toolchain)
