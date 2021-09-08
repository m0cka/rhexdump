![Version](https://img.shields.io/github/v/tag/m0cka/rhexdump?color=1982c4&style=flat-square) ![Dependencies](https://img.shields.io/badge/dependencies-none-ffca3a?style=flat-square) ![License Badge](https://img.shields.io/github/license/m0cka/rhexdump?color=8ac926&style=flat-square)


# rhexdump

`rhexdump` is a hexdump library written in Rust to format byte slices and files.

## Features

rhexdump provides the following features:

* Hexdump of files and byte slices (with an optional base offset)
* Iterators over files and byte slices (with an optional base offset)
* Customizable settings:
    - numeral base 
    - number of bytes per group
    - number of bytes per line
    - whether or not duplicate lines should be displayed
    - output format

## Usage

### Default Hexdump Without Any Customization.

You can use rhexdump with a default configuration, feed it a byte slice and simply print the result.

```rust
use rhexdump;

let v = (0..0x80).collect::<Vec<u8>>();
println!("{}", rhexdump::hexdump(&v));
```

```
00000000: 00 01 02 03 04 05 06 07 08 09 0a 0b 0c 0d 0e 0f | ................
00000010: 10 11 12 13 14 15 16 17 18 19 1a 1b 1c 1d 1e 1f | ................
00000020: 20 21 22 23 24 25 26 27 28 29 2a 2b 2c 2d 2e 2f | .!"#$%&'()*+,-./
00000030: 30 31 32 33 34 35 36 37 38 39 3a 3b 3c 3d 3e 3f | 0123456789:;<=>?
00000040: 40 41 42 43 44 45 46 47 48 49 4a 4b 4c 4d 4e 4f | @ABCDEFGHIJKLMNO
00000050: 50 51 52 53 54 55 56 57 58 59 5a 5b 5c 5d 5e 5f | PQRSTUVWXYZ[\]^_
00000060: 60 61 62 63 64 65 66 67 68 69 6a 6b 6c 6d 6e 6f | `abcdefghijklmno
00000070: 70 71 72 73 74 75 76 77 78 79 7a 7b 7c 7d 7e 7f | pqrstuvwxyz{|}~.
```


### Default Hexdump With a Base Offset.

If you're working with memory dumps, you can start the output from a base offset.

```rust
use rhexdump;

let v = (0..0x80).collect::<Vec<u8>>();
println!("{}", rhexdump::hexdump_offset(&v, 0xdeadbeef));
```

```
deadbeef: 00 01 02 03 04 05 06 07 08 09 0a 0b 0c 0d 0e 0f | ................
deadbeff: 10 11 12 13 14 15 16 17 18 19 1a 1b 1c 1d 1e 1f | ................
deadbf0f: 20 21 22 23 24 25 26 27 28 29 2a 2b 2c 2d 2e 2f | .!"#$%&'()*+,-./
deadbf1f: 30 31 32 33 34 35 36 37 38 39 3a 3b 3c 3d 3e 3f | 0123456789:;<=>?
deadbf2f: 40 41 42 43 44 45 46 47 48 49 4a 4b 4c 4d 4e 4f | @ABCDEFGHIJKLMNO
deadbf3f: 50 51 52 53 54 55 56 57 58 59 5a 5b 5c 5d 5e 5f | PQRSTUVWXYZ[\]^_
deadbf4f: 60 61 62 63 64 65 66 67 68 69 6a 6b 6c 6d 6e 6f | `abcdefghijklmno
deadbf5f: 70 71 72 73 74 75 76 77 78 79 7a 7b 7c 7d 7e 7f | pqrstuvwxyz{|}~.
```


### Customized Hexdump - Binary Base and 4-Byte Lines

If the default configuration does not fit your need, you can specify your own.

```rust
use rhexdump;

let v = (0..0x40).collect::<Vec<u8>>();

let rhx = rhexdump::Rhexdump::new(
    rhexdump::Base::Bin,                      // Raw bytes will be displayed in binary format
    rhexdump::Endianess::LittleEndian,        // Byte groups will be interpreted as LE
    1,                                        // Byte groups are 1-byte long
    4,                                        // There are 4 bytes per line
    true,                                     // Duplicate lines will be displayed
    "#[OFFSET]: #[RAW] | #[ASCII]"            // Output format string
).unwrap();

println!("{}", rhx.hexdump(&v));
```

```
00000004: 00000100 00000101 00000110 00000111 | ....
00000008: 00001000 00001001 00001010 00001011 | ....
0000000c: 00001100 00001101 00001110 00001111 | ....
00000010: 00010000 00010001 00010010 00010011 | ....
00000014: 00010100 00010101 00010110 00010111 | ....
00000018: 00011000 00011001 00011010 00011011 | ....
0000001c: 00011100 00011101 00011110 00011111 | ....
00000020: 00100000 00100001 00100010 00100011 | .!"#
00000024: 00100100 00100101 00100110 00100111 | $%&'
00000028: 00101000 00101001 00101010 00101011 | ()*+
0000002c: 00101100 00101101 00101110 00101111 | ,-./
00000030: 00110000 00110001 00110010 00110011 | 0123
00000034: 00110100 00110101 00110110 00110111 | 4567
00000038: 00111000 00111001 00111010 00111011 | 89:;
0000003c: 00111100 00111101 00111110 00111111 | <=>?
```


### Customized Hexdump - Custom Format

The output format can be customized by providing a string that specifies the format of a line.

For example, if you want to display the ascii representation, the hexadecimal bytes and then the offsets, while having everything separated by dots, you could specify the following string: 

```
#[ASCII] .. #[RAW] .. #[OFFSET]
```

```rust
use rhexdump;

let v = (0..0x40).collect::<Vec<u8>>();

let rhx = rhexdump::Rhexdump::new(
    rhexdump::Base::Hex,                      // Raw bytes will be displayed in hex format
    rhexdump::Endianess::LittleEndian,        // Byte groups will be interpreted as LE
    2,                                        // Byte groups are 2-byte long
    16,                                       // There are 16 bytes per line
    true,                                     // Duplicate lines will be displayed
    "#[ASCII] | #[RAW] :: [OFFSET]"           // Output format string
).unwrap();

println!("{}", rhx.hexdump(&v));
```

```
................ | 0100 0302 0504 0706 0908 0b0a 0d0c 0f0e :: 00000000
................ | 1110 1312 1514 1716 1918 1b1a 1d1c 1f1e :: 00000010
.!"#$%&'()*+,-./ | 2120 2322 2524 2726 2928 2b2a 2d2c 2f2e :: 00000020
0123456789:;<=>? | 3130 3332 3534 3736 3938 3b3a 3d3c 3f3e :: 00000030
```


### Customized Hexdump - Masking Duplicate Lines

You can group duplicate lines together to get a more readable output.

```rust
use rhexdump;

let a = [0u8; 0x10000];

let mut rhx = rhexdump::Rhexdump::default();
rhx.display_duplicate_lines(false);

println!("{}", rhx.hexdump(&a));
```

```
00000000: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 | ................
*
0000fff0: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 | ................
```


### Iterators Over a File (or a Byte Slice)

You can iterate over a file or a byte slice.

```rust
use rhexdump;

use std::fs::OpenOptions;

let mut f = OpenOptions::new()
    .read(true)
    .open("/dev/random")
    .expect("Cannot open /dev/random");
let rhx = rhexdump::Rhexdump::default();

for line in rhx.iter_file(&mut f, Some(0x80)) {
    println!("{}", line);
}
```

```
00000000: bc ab de a1 07 db 95 8a 1c 57 24 ba 71 34 c5 5e | .........W$.q4.^
00000010: 6f 58 6b c3 27 4d ef ae 0b 5d 5d 39 80 c6 42 2b | oXk.'M...]]9..B+
00000020: 8b e2 3a 02 c1 6a 77 9f 69 99 65 4d 3e ac f4 e5 | ..:..jw.i.eM>...
00000030: 7f ff 98 77 31 78 7b 65 32 15 7e 46 68 03 10 69 | ...w1x{e2.~Fh..i
00000040: ad cd df fa e7 d8 21 79 7c c4 4b 11 48 c1 49 c3 | ......!y|.K.H.I.
00000050: 56 df e0 eb de cc 9b fa 29 34 63 d1 d8 0e 63 5f | V.......)4c...c_
00000060: e5 6c 10 5e 16 30 8b ed 45 41 ed 79 6c 3f b9 5a | .l.^.0..EA.yl?.Z
00000070: 5e 50 eb 49 45 66 38 e5 8d 92 1b 8c 9e fe c6 c8 | ^P.IEf8.........
```
