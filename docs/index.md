# `xmpkit-py`

A Python package providing bindings to [`xmpkit`](https://github.com/cavivie/xmpkit), a pure-Rust crate dedicated to XMP metadata manipulation.

> The Extensible Metadata Platform (XMP) is an ISO standard, originally created by Adobe Systems Inc., for the creation, processing and interchange of standardized and custom metadata for digital documents and data sets.

## Why?

- Very few Python packages exist for manipulating XMP metadata
- The only options often require to install a dependency along the Python package itself, which is not very convenient
- Cross-platform support is not always assured

This package has no dependencies and can be installed in one step only through `pip` or your favorite package manager. It also benefits from the high performance of Rust.

## Features

- Compatible with Adobe XMP standard
- Support for common file formats (see list [here](https://github.com/cavivie/xmpkit#file-format-support))
- Memory safe and high performance
- API almost identical to the Rust crate
