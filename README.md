# nippy

[![crates.io version](https://img.shields.io/crates/v/nippy.svg)](https://crates.io/crates/nippy) [![Documentation](https://docs.rs/nippy/badge.svg)](https://docs.rs/nippy/) [![Build Status](https://travis-ci.org/ExpHP/nippy.svg?branch=master)](https://travis-ci.org/ExpHP/nippy)


>  **Note: 2021/07/01.** The time is now.  Once again I've needed this for my own projects, and now I've been working hard to prepare it for release.  Hopefully, I will have succeeded and removed this message long before you ever have the chance to read it.
>
> So.  If you *are* reading this message, and 2021/07/01 was a while ago, then.... well, that sucks.  If you desparately need one of the above features, you can use this as a git dependency for now, and please drop me a message reminding me to come back and finish what I started.
>
> ```toml
> [dependencies.nippy]
> git = "https://github.com/ExpHP/nippy"
> rev = "bd608d41f"  # replace with the latest commit
> ```

Numpy format (*.npy) serialization and deserialization.

[**NPY**](https://docs.scipy.org/doc/numpy-dev/neps/npy-format.html) is a simple binary data format.
It stores the type, shape and endianness information in a header,
which is followed by a flat binary data field. This crate offers a simple, mostly type-safe way to
read and write *.npy files. Files are handled using iterators, so they don't need to fit in memory.

`nippy` is a fork and successor of the seemingly-dead [`npy`](https://github.com/potocpav/npy-rs).

## Usage

To use **nippy**, two dependencies must be specified in `Cargo.toml`:

```toml
nippy = "0.5"
```

You may be interested in enabling some features:

```toml
npy = {version = "0.5", features = ["derive", "complex"]}
```

Data can now be read from a `*.npy` file:

```rust
use nippy::NpyReader;

fn main() -> std::io::Result<()> {
    let bytes = std::fs::read("examples/plain.npy")?;

    // Note: In addition to byte slices, this accepts any io::Read
    let data: NpyReader<f64, _> = NpyReader::new(&bytes[..])?;
    for number in data {
        let number = number?;
        eprintln!("{}", number);
    }
    Ok(())
}
```

For further examples and information on:
* Reading `npy` files,
* Writing `npy` files,
* Working with structured arrays,
* Interop with the `ndarray` crate,

please see the [documentation on the root module](https://docs.rs/nippy).

## Relation to the `npy` crate

`nippy` is a fork of Pavel Potoček's [`npy` crate](https://github.com/potocpav/npy-rs).  The original `npy` supported structured arrays with derives, but had many, many limitations:

* 1D arrays only.
* Little endian only.
* No `Complex`, no bytestrings.
* Reading API based on `&[u8]` instead of `Read`, with the expectation that a user can use a memmap for files too large to fit in memory.
* No `io::Write`, only one writing API that writes directly to the filesystem.

Originally, `nippy` was a place for me to protype new features with reckless abandon before finally making a PR to `npy`, but even my first few foundational PRs have yet to be merged upstream.  I believe Pavel has a good head on their shoulders and a great attention to detail, and I appreciated their initial response on my PRs, but nearly two years have passed since the last time I have heard from them. Therefore, I've decided to go forward and publish the fork.
