# realsense-sys

Generate and use RealSense C library bindings as a Rust crate. This crate is used as a base layer in the more
user-friendly [realsense-rust](https://gitlab.com/tangram-vision-oss/realsense-rust) crate; we recommend use of
realsense-rust if possible in order to better maintain Rust memory safety.

Compatible with RealSense SDK v2.0 and up.

**Default bindings are for librealsense version: 2.50.0**

## Usage

This crate finds and links the RealSense SDK. Though one can use the generated bindings directly, this crate is meant as
a base layer for [realsense-rust](https://gitlab.com/tangram-vision-oss/realsense-rust).

To use this crate, add this line in your `Cargo.toml`.

```toml
realsense-sys = "<current version number>"
```

## Regenerating the API Bindings

Bindgen relies on clang to generate new FFI bindings. See the OS Use Notes below for more.

_Non-Linux users_: The current bindings are formatted for Linux. Users on systems other than Linux must run with the
`buildtime-bindgen` feature to reformat the bindings. See more notes for your platform below.

_Backwards compatibility_: If you're using an older librealsense version, you may enable the `buildtime-bindgen` feature
to re-generate the bindings. We make no claims of backwards compatibility; good luck.

With all of that said: Run the following to regenerate the realsense2 SDK bindings:

`cargo build --features buildtime-bindgen`

# OS Use Notes

## Linux

You can install Clang using the following command:

`sudo apt install libclang-dev clang`

If the realsense2 SDK is installed, pkg-config will detect the [realsense2.pc](./realsense2.pc) config file automatically. This will load
the necessary headers and libraries.

## Windows

**NOTE**: The current bindings are formatted for Linux. Users must run with the `buildtime-bindgen` feature active to
reformat the bindings for Windows platforms.

This installation process assumes that the RealSense SDK was installed through the .exe wizard downloadable from [the
librealsense asset page](https://github.com/IntelRealSense/librealsense/releases/tag/v2.47.0). This process will install
the SDK in `C:/Program Files (x86)/Intel RealSense SDK 2.0`. If your installation is in another place, modify the
`prefix` line in [realsense2.pc](./realsense2.pc) to the right path.

### Install Pkg-config and Clang

Install pkg-config via Chocolatey:

1. https://chocolatey.org/install (if not already on the system)
2. `choco install pkgconfiglite`
3. `choco install llvm` for bindgen (if not already installed)

### Guide Pkg-config to realsense2.pc

Set the pkg-config path in Powershell to the realsense-sys directory. One can do this in two ways:

**First Option: Modify pkg-config's environment variables**

To do this, run

`$Env:PKG_CONFIG_PATH="C:\Users\< path_to_repo >\realsense-rust\realsense-sys\"`

This will help pkg-config find the [realsense2.pc](./realsense2.pc) file located in this directory. This file tells pkg-config where to
locate the headers and libraries necessary for RealSense operation. The Windows wizard does not provide this file, so we
provide it ourselves.

It's a good idea to set the `PKG_CONFIG_PATH` Environment Variable globally as well via the System Properties. _BUT
NOTE_: Environment Variables set through the Windows System Properties will not apply until the host machine is power
cycled. Yep. That's a thing.

**Second Option: Add [realsense2.pc](./realsense2.pc) to pkg-config's search directory**

Run the following command...

`pkg-config --variable pc_path pkg-config`

...to identify the directory (or directories) that pkg-config uses to find \*.pc files. Copy [realsense2.pc](./realsense2.pc) to that
directory. Boom, done.

---

# Architecture Notes

This crate provides a bindgen mapping to the low-level C-API of librealsense2.

In that respect, it is fairly straightforward in how realsense-sys maps types from the C-API to
Rust, as nothing particularly unique is done other than running `bindgen` to generate the
bindings.

The `sys` alias is used extensively throughout the realsense-rust crate, so you'll often see
code of the form `sys::rs2_XXX`.

## Understanding lifetimes

This library is generated by using `bindgen` on a set of C headers, usually located at
`/usr/include/librealsense2/rs.h` (or wherever you installed librealsense2). Inherently, this
makes most of the code in this module unsafe, since it is relying on the underlying C library
to define what the lifetimes are for every data type.

librealsense2 does not always do the best job at documenting what the lifetime of an object is.
Understand that the library is primarily a C++ library, with a C-compatible API built on top of
it. This means that while some guarantees about lifetimes can be made by some types, these
guarantees are not always explicit. By that, I mean that many quantities in the C++ library are
managed via C++ `shared_ptr` or `unique_ptr`. However, the C API on top of this cannot expose
these types, and it is often unclear if a pointer you get in the C API is a result of calling
`shared_ptr::get()` or `unique_ptr::get()` under the hood. A good example of this is
`rs2_get_frame_sensor`, which will give you a pointer to a sensor type from a `shared_ptr`
under the hood. As a result, you do not need to manually manage this pointer and can just drop
it whenever as the `shared_ptr` under the hood will delete the resource when it is no longer
held. However, if you get the sensor from a sensor list in the low-level API by calling
`rs2_create_sensor` then you will notice that this pointer is allocated with `new`, and if you
were using this then it needs to be deleted by a call to `rs2_delete_sensor`. In both cases you
get a `*mut rs2_sensor` from the wrapper, but the lifetime and ownership information is dealt
with very differently. This makes the API fairly difficult to navigate.

- [`rs2_get_frame_sensor`](https://github.com/IntelRealSense/librealsense/blob/master/src/rs.cpp#L922)
- [`rs2_create_sensor`](https://github.com/IntelRealSense/librealsense/blob/master/src/rs.cpp#L308)

In general, reading through `bindings.rs` in the repo will be useful in describing the
documentation that Intel provides around every function in the C-API. However, you may find
that such documentation is insufficient to understand the lifetimes since not every function
documents the proper ownership. As a result you end up needing to understand librealsense2 in
C, C++, and Rust in order to utilize the realsense-sys library safely and effectively. If you
do find yourself looking for an entry point into the librealsense2 C-API, we highly suggest
starting at [this file](https://github.com/IntelRealSense/librealsense/blob/master/src/rs.cpp)
and working your way out via each type.

If this seems like a lot of effor to you (it truly is!), we highly suggest using the
realsense-rust wrapper, which attempts to abstract over these and provide a high-level,
Rust-native API that avoids unsafe code.

# License

Apache 2.0. See [LICENSE](LICENSE) file.
