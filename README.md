
# libspnav-sys

[![crates.io](https://img.shields.io/crates/v/libspnav-sys.svg)](https://crates.io/crates/libspnav-sys)

The libspnav-sys crate provides declarations and linkage for the libspnav C
library. Following the *-sys package conventions, the libspnav-sys crate does
not define higher-level abstractions over the native libspnav library functions.

## Versions

This crate currently builds and links against [`libspnav v0.2.3`](https://github.com/FreeSpacenav/libspnav.git).

[`bindgen 0.48.1`](https://github.com/rust-lang/rust-bindgen) was used to generate the bindings.

## Dependencies

This crate makes use of [`cc`](https://github.com/alexcrichton/cc-rs) to compile `spnav.c`, therfore a working C compiler useable by `cc` is required.

## Limitations

Bindings are currently only provided for `spnav.h` with `USE_X11` disabled. X11
support (including `spnav_magellon.h`) will be considered in the future.
