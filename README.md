# Rust OpenCV bindings

[![Build Status](https://travis-ci.org/twistedfall/opencv-rust.svg?branch=master)](https://travis-ci.org/twistedfall/opencv-rust)
[![docs.rs](https://docs.rs/opencv/badge.svg)](https://docs.rs/opencv)

Experimental Rust bindings for OpenCV 3.

The API is usable but unstable and not very battle-tested; use at your own risk.

## API

[API Documentation](https://docs.rs/opencv) is, to varying success,
translated from OpenCV's doxygen docs. Most likely you'll want to
refer to the official [OpenCV C++
documentation](https://docs.opencv.org/3.4.6/) as well.

### Functionality

Generally the crate tries to only wrap OpenCV API and provide some convenience functions
to be able to use it in Rust easier. We try to avoid adding any functionality besides
that.

### Callbacks

Some API functions accept callbacks, e.g. set_mouse_callback. While currently
it's possible to successfully use those functions there are some limitations to
keep in mind. Current implementation of callback handling keeps hold of the
passed callback argument forever. That means that the closure used as a callback
will never be freed during the lifetime of a program and moreover Drop will
not be called for it (they are stored in global static [`Slab`](https://crates.io/crates/slab)).
There is a plan to implement possibility to be able to free at least some of the
closures.

### Unsafety

Although crate tries to provide ergonomic Rust interface for OpenCV, don't expect
Rust safety guarantees at this stage. It's especially true for borrow checking and
shared mutable ownership. Notable example would be `Mat` which is a reference counted
object in its essence. You can own a seemingly separate `Mat` in Rust terms, but
it's going to be a mutable reference to the other `Mat` under the hood. Treat safety
of the crate's API as you would treat one of C++, use `clone()` when needed.

## Getting Started

The following external dependencies are required:
- python2.7
- OpenCV 3.4.6 (other 3.4 versions may work, but development is done
  against this version)

OpenCV is a complicated dependency with a lot of different
configurations that can break this crate since it relies on fragile
header parsing. Try these strategies in order, continuing to the next
if builds with this crate fail.

### See if you already have the right version of OpenCV installed

if OpenCV is installed, its version can be checked with:

```sh
opencv_version
```

### Install OpenCV

Install OpenCV 3.4.6. Check your platform's package manager or see the
upstream OpenCV
[installation guides](https://docs.opencv.org/3.4/df/d65/tutorial_table_of_content_introduction.html).

### Compiling OpenCV

See the [upstream
guides](https://docs.opencv.org/3.4/df/d65/tutorial_table_of_content_introduction.html)
for compiling OpenCV for your platform. Make sure to compile from the
correct release tag! We recommend including opencv_contrib and
configuring your build with the same flags the travis build uses:

```sh
cmake \
    -DCMAKE_BUILD_TYPE=Release \
    -DBUILD_PERF_TESTS=OFF \
    -DBUILD_TESTS=OFF \
    -DINSTALL_TESTS=OFF \
    -DBUILD_DOCS=OFF \
    -DBUILD_EXAMPLES=OFF \
    -DBUILD_opencv_apps=ALL \
    -DWITH_IPP=OFF \
    -DPYTHON_EXECUTABLE=OFF \
    -DINSTALL_PYTHON_EXAMPLES=OFF \
    -DWITH_LAPACK=ON \
    -DWITH_EIGEN=ON \
    -DBUILD_SHARED_LIBS=ON \
    -DWITH_TBB=ON \
    -DOPENCV_ENABLE_NONFREE=ON \
    -DCMAKE_INSTALL_PREFIX=/usr \
    -DOPENCV_EXTRA_MODULES_PATH=$PATH_TO_OPENCV_CONTRIB_MODULES \
    $PATH_TO_OPENCV_SRC
```

## Contrib modules
The following modules require contrib installed:
 * bioinspired
 * ccalib
 * dpm
 * freetype
 * fuzzy
 * img_hash
 * phase_unwrapping
 * plot

## OpenCV 2 support

If you can't use OpenCV 3.4.6, the (no longer maintained) `0.2.4`
version of this crate is known to work with OpenCV `2.4.7.13` (and probably other 2.4 versions).

## The binding strategy

This crate works following the model of python and java's OpenCV
wrappers - it parses the OpenCV C++ headers, generates a C interface
to the C++ api, and wraps the C interface in Rust.

All the major modules in the C++ API are merged together in a huge
`cv::` namespace. We instead made one rust module for each major
OpenCV module. So, for example, C++ `cv::Mat` is `opencv::core::Mat`
in this crate.

The methods and field names have been snake_cased. Methods arguments with
default value lose these default values, but they are reported in the
API documentation.

Overloaded methods have been — manually — given different names.

All methods return a Result to hack around C++ exception handling.

Most of the API is covered, but for various reasons several modules
are not yet implemented. If a missing module is near and dear to you,
file an issue (or better, open a pull request!)

## Contributor's Guide

The crate itself, as imported by users, consists of generated rust
code in [src](src) committed to the repo. This way, users don't have
to handle the code generation overhead in their builds. When
developing this crate, you can test changes to the binding generation
using `cargo build -vv --features buildtime_bindgen`. When changing
the codegen, be sure to push changes to the generated code!

`hdr_parser.py` comes from OpenCV python/java generator. We've tried
not to mess too much with this file, but had to make a few changes.

`gen_rust.py` is initially a copy of gen_java, also from the OpenCV
generators, but it has drifted significantly from the original.

The license for the original work is [MIT](https://opensource.org/licenses/MIT).

Special thanks to [ttacon](https://github.com/ttacon) for yielding the crate name.
