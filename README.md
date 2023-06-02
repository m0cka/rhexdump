![Version](https://img.shields.io/github/v/tag/m0cka/rhexdump?color=1982c4&style=flat-square) ![License Badge](https://img.shields.io/github/license/m0cka/rhexdump?color=8ac926&style=flat-square) ![Dependencies](https://img.shields.io/badge/dependencies-none-ffca3a?style=flat-square) <!-- [![Documentation](https://img.shields.io/badge/documentation-doc.rs-ff595e?style=flat-square)](https://docs.rs/rhexdump/0.2.0/rhexdump/) -->

# Rhexdump

Rhexdump is a hexdump library written in Rust to format data in a fast, convenient and customizable manner.

## Add as a Dependency

Add the following line under `[dependencies]` in your `Cargo.toml` file.

```toml
rhexdump = { git = "https://github.com/m0cka/rhexdump.git" }
```

<!--
```toml
rhexdump = "0.2.0"
```
-->


## Features

Rhexdump provides the following features:

* [Macros](#macros) to format data and configure Rhexdump efficiently;
* [Hexdump utilities](#hexdump-utilities) ready to be used or that can be adapted to your needs;
* [Iterators](#iterators)
* [Customizable settings](#customizable-settings) for Rhexdump instances.

### Macros

Macros work on byte slices and can be used to format data to stdout or a string.

```rust
use rhexdump::prelude::*;

fn main() {
    let v = (0..0x80).collect::<Vec<u8>>();

    // Hexdumps `v` to stdout using the current global configuration.
    rhexdump!(&v);

    // 00000000: 00 01 02 03 04 05 06 07 08 09 0a 0b 0c 0d 0e 0f | ................
    // 00000010: 10 11 12 13 14 15 16 17 18 19 1a 1b 1c 1d 1e 1f | ................
    // 00000020: 20 21 22 23 24 25 26 27 28 29 2a 2b 2c 2d 2e 2f | .!"#$%&'()*+,-./
    // 00000030: 30 31 32 33 34 35 36 37 38 39 3a 3b 3c 3d 3e 3f | 0123456789:;<=>?
    // 00000040: 40 41 42 43 44 45 46 47 48 49 4a 4b 4c 4d 4e 4f | @ABCDEFGHIJKLMNO
    // 00000050: 50 51 52 53 54 55 56 57 58 59 5a 5b 5c 5d 5e 5f | PQRSTUVWXYZ[\]^_
    // 00000060: 60 61 62 63 64 65 66 67 68 69 6a 6b 6c 6d 6e 6f | `abcdefghijklmno
    // 00000070: 70 71 72 73 74 75 76 77 78 79 7a 7b 7c 7d 7e 7f | pqrstuvwxyz{|}~.


    // Hexdumps `v` to stdout with an offest using the current global configuration.
    rhexdump!(&v, 0x12340000);

    // 12340000: 00 01 02 03 04 05 06 07 08 09 0a 0b 0c 0d 0e 0f | ................
    // 12340010: 10 11 12 13 14 15 16 17 18 19 1a 1b 1c 1d 1e 1f | ................
    // 12340020: 20 21 22 23 24 25 26 27 28 29 2a 2b 2c 2d 2e 2f | .!"#$%&'()*+,-./
    // 12340030: 30 31 32 33 34 35 36 37 38 39 3a 3b 3c 3d 3e 3f | 0123456789:;<=>?
    // 12340040: 40 41 42 43 44 45 46 47 48 49 4a 4b 4c 4d 4e 4f | @ABCDEFGHIJKLMNO
    // 12340050: 50 51 52 53 54 55 56 57 58 59 5a 5b 5c 5d 5e 5f | PQRSTUVWXYZ[\]^_
    // 12340060: 60 61 62 63 64 65 66 67 68 69 6a 6b 6c 6d 6e 6f | `abcdefghijklmno
    // 12340070: 70 71 72 73 74 75 76 77 78 79 7a 7b 7c 7d 7e 7f | pqrstuvwxyz{|}~.


    // Returns the hexdump of `v` as a string using the current global configuration.
    let output = rhexdumps!(&v);


    // Returns the hexdump of `v`, with an offset, as a string using the current global
    // configuration.
    let output = rhexdumps!(&v, 0x12340000);
}
```

By default, a global configuration is provided to control the macros' output format. This configuration can be changed using the `rhexdump_install` macro. You can find more information about configurations in the [documentation](https://docs.rs/rhexdump/)

```rust
use rhexdump::prelude::*;

fn main() {
    // Data to format.
    let v = (0..0x14).collect::<Vec<u8>>();
    // Creating a new rhexdump configuration.
    let config = RhexdumpBuilder::new()
        .base(Base::Oct)
        .bit_width(BitWidth::BW64)
        .group_size(GroupSize::Word)
        .groups_per_line(4)
        .config();
    // Installing the configuration globally.
    rhexdump_install!(config);
    // Hexdump of the same data with the new configuration.
    let output = rhexdumps!(&v);
    assert_eq!(
        &output,
        "0000000000000000: 000400 001402 002404 003406  ........\n\
        0000000000000008: 004410 005412 006414 007416  ........\n\
        0000000000000010: 010420 011422                ....\n"
    );
}
```

### Hexdump Utilities

* Generic hexdump with an offset from a `std::io::Read` object to a `std::io::Write` one.

```rust
use rhexdump::prelude::*;
use std::io::prelude::*;
use std::io::*;
use std::env::temp_dir;
use std::fs::OpenOptions;

fn main() {
    // Create a Rhexdump instance.
    let rhx = Rhexdump::new();

    // Create a temporary output file.
    let filename = "rhx_rhexdump_hexdump_offset.test";
    let mut test_file = temp_dir();
    test_file.push(filename);
    let mut f = OpenOptions::new()
        .write(true)
        .read(true)
        .create(true)
        .truncate(true)
        .open(test_file)
        .expect(&format!("Cannot create {}", filename));

    // String that will be formatted.
    let input = String::from("Lorem ipsum dolor sit amet, consectetur adipiscing elit");
    let mut cur = Cursor::new(&input);

    // Formatting data to the temp file.
    rhx.hexdump_offset(&mut f, &mut cur, 0x12340000);

    // Reading the temp file content and making sure it corresponds to the expected output.
    f.seek(SeekFrom::Start(0))
        .expect(&format!("Could not seek to start of {}", filename));
    let mut output = Vec::new();
    f.read_to_end(&mut output)
        .expect(&format!("Cannot read from {}", filename));
    assert_eq!(
        &String::from_utf8_lossy(&output),
        "12340000: 4c 6f 72 65 6d 20 69 70 73 75 6d 20 64 6f 6c 6f  Lorem.ipsum.dolo\n\
         12340010: 72 20 73 69 74 20 61 6d 65 74 2c 20 63 6f 6e 73  r.sit.amet,.cons\n\
         12340020: 65 63 74 65 74 75 72 20 61 64 69 70 69 73 63 69  ectetur.adipisci\n\
         12340030: 6e 67 20 65 6c 69 74                             ng.elit\n"
    );
}
```

* Hexdump a byte slice to a [`String`].

```rust
use rhexdump::prelude::*;

fn main() {
    let v = (0..0x14).collect::<Vec<u8>>();
    let rh = RhexdumpString::new();
    let out = rh.hexdump_bytes(&v);
    assert_eq!(
        &out,
        "00000000: 00 01 02 03 04 05 06 07 08 09 0a 0b 0c 0d 0e 0f  ................\n\
         00000010: 10 11 12 13                                      ....\n"
    );
}
```

* Hexdump with an offset a [`std::io::Read`] object to [`std::io::Stdout`].

```rust
use rhexdump::prelude::*;

fn main() {
    // Create a Rhexdump instance.
    let rhx = RhexdumpStdout::new();

    // String that will be formatted.
    let input = String::from("Lorem ipsum dolor sit amet, consectetur adipiscing elit");
    let mut cur = std::io::Cursor::new(&input);

    // Formatting data to the temp file.
    rhx.hexdump_offset(&mut cur, 0x12340000);
}
```

Refer to the [documentation](https://docs.rs/rhexdump/) for additional hexdump methods.

### Iterators

This crate also provides iterators over hexdump-formatted data.

```rust
use rhexdump::prelude::*;

fn main() {
    // Create a Rhexdump instance.
    let rhx = Rhexdump::new();

    // String that will be formatted.
    let input = String::from("Lorem ipsum dolor sit amet, consectetur adipiscing elit");
    let mut cur = std::io::Cursor::new(&input);

    // Creating an iterator.
    let mut iter = RhexdumpStringIter::new(rhx, &mut cur);

    // Taking two lines of output.
    let _ = iter.next().unwrap();
    let output = iter.next().unwrap();

    assert_eq!(
        &output,
        "00000010: 72 20 73 69 74 20 61 6d 65 74 2c 20 63 6f 6e 73  r.sit.amet,.cons"
    );
}
```

### Customizable Settings

Use the `RhexdumpBuilder` object to construct a customized configuration for your data.

```rust
use rhexdump::prelude::*;

fn main() {
    let v = (0..0x10).collect::<Vec<u8>>();
    let rh = RhexdumpBuilder::new()
        .group_size(GroupSize::Dword)
        .groups_per_line(4)
        .endianness(Endianness::BigEndian)
        .build_string();
    let out = rh.hexdump_bytes(&v);
    assert_eq!(
        &out,
        "00000000: 00010203 04050607 08090a0b 0c0d0e0f  ................\n"
    );
}
```