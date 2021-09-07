//! This crate provides a hexdump library to format byte arrays and files.
//!
//! # Examples
//!
//! ### Default Hexdump Without Any Customization.
//!
//! ```
//! use rhexdump;
//!
//! let v = (0..0x80).collect::<Vec<u8>>();
//! println!("{}", rhexdump::hexdump(&v));
//! ```
//!
//! ```text, no_run
//! 00000000: 00 01 02 03 04 05 06 07 08 09 0a 0b 0c 0d 0e 0f | ................
//! 00000010: 10 11 12 13 14 15 16 17 18 19 1a 1b 1c 1d 1e 1f | ................
//! 00000020: 20 21 22 23 24 25 26 27 28 29 2a 2b 2c 2d 2e 2f | .!"#$%&'()*+,-./
//! 00000030: 30 31 32 33 34 35 36 37 38 39 3a 3b 3c 3d 3e 3f | 0123456789:;<=>?
//! 00000040: 40 41 42 43 44 45 46 47 48 49 4a 4b 4c 4d 4e 4f | @ABCDEFGHIJKLMNO
//! 00000050: 50 51 52 53 54 55 56 57 58 59 5a 5b 5c 5d 5e 5f | PQRSTUVWXYZ[\]^_
//! 00000060: 60 61 62 63 64 65 66 67 68 69 6a 6b 6c 6d 6e 6f | `abcdefghijklmno
//! 00000070: 70 71 72 73 74 75 76 77 78 79 7a 7b 7c 7d 7e 7f | pqrstuvwxyz{|}~.
//! ```
//!
//!
//! ### Default Hexdump With a Base Offset.
//!
//! ```
//! use rhexdump;
//!
//! let v = (0..0x80).collect::<Vec<u8>>();
//! println!("{}", rhexdump::hexdump_offset(&v, 0xdeadbeef));
//! ```
//!
//! ```text, no_run
//! deadbeef: 00 01 02 03 04 05 06 07 08 09 0a 0b 0c 0d 0e 0f | ................
//! deadbeff: 10 11 12 13 14 15 16 17 18 19 1a 1b 1c 1d 1e 1f | ................
//! deadbf0f: 20 21 22 23 24 25 26 27 28 29 2a 2b 2c 2d 2e 2f | .!"#$%&'()*+,-./
//! deadbf1f: 30 31 32 33 34 35 36 37 38 39 3a 3b 3c 3d 3e 3f | 0123456789:;<=>?
//! deadbf2f: 40 41 42 43 44 45 46 47 48 49 4a 4b 4c 4d 4e 4f | @ABCDEFGHIJKLMNO
//! deadbf3f: 50 51 52 53 54 55 56 57 58 59 5a 5b 5c 5d 5e 5f | PQRSTUVWXYZ[\]^_
//! deadbf4f: 60 61 62 63 64 65 66 67 68 69 6a 6b 6c 6d 6e 6f | `abcdefghijklmno
//! deadbf5f: 70 71 72 73 74 75 76 77 78 79 7a 7b 7c 7d 7e 7f | pqrstuvwxyz{|}~.
//! ```
//!
//!
//! ### Customized Hexdump - Binary Base and 4-Byte Lines
//!
//! ```
//! use rhexdump;
//!
//! let v = (0..0x40).collect::<Vec<u8>>();
//!
//! let rhx = rhexdump::Rhexdump::new(
//!     rhexdump::Base::Bin,                      // Raw bytes will be displayed in binary format
//!     rhexdump::Endianess::LittleEndian,        // Byte groups will be interpreted as LE
//!     1,                                        // Byte groups are 1-byte long
//!     4,                                        // There are 4 bytes per line
//!     true,                                     // Duplicate lines will be displayed
//!     "#[OFFSET]: #[RAW] | #[ASCII]"            // Output format string
//! ).unwrap();
//!
//! println!("{}", rhx.hexdump(&v));
//! ```
//!
//! ```text, no_run
//! 00000004: 00000100 00000101 00000110 00000111 | ....
//! 00000008: 00001000 00001001 00001010 00001011 | ....
//! 0000000c: 00001100 00001101 00001110 00001111 | ....
//! 00000010: 00010000 00010001 00010010 00010011 | ....
//! 00000014: 00010100 00010101 00010110 00010111 | ....
//! 00000018: 00011000 00011001 00011010 00011011 | ....
//! 0000001c: 00011100 00011101 00011110 00011111 | ....
//! 00000020: 00100000 00100001 00100010 00100011 | .!"#
//! 00000024: 00100100 00100101 00100110 00100111 | $%&'
//! 00000028: 00101000 00101001 00101010 00101011 | ()*+
//! 0000002c: 00101100 00101101 00101110 00101111 | ,-./
//! 00000030: 00110000 00110001 00110010 00110011 | 0123
//! 00000034: 00110100 00110101 00110110 00110111 | 4567
//! 00000038: 00111000 00111001 00111010 00111011 | 89:;
//! 0000003c: 00111100 00111101 00111110 00111111 | <=>?
//! ```
//!
//!
//! ### Customized Hexdump - Custom Format
//!
//! ```
//! use rhexdump;
//!
//! let v = (0..0x40).collect::<Vec<u8>>();
//!
//! let rhx = rhexdump::Rhexdump::new(
//!     rhexdump::Base::Hex,                      // Raw bytes will be displayed in hex format
//!     rhexdump::Endianess::LittleEndian,        // Byte groups will be interpreted as LE
//!     2,                                        // Byte groups are 2-byte long
//!     16,                                       // There are 16 bytes per line
//!     true,                                     // Duplicate lines will be displayed
//!     "#[ASCII] | #[RAW] :: [OFFSET]"           // Output format string
//! ).unwrap();
//!
//! println!("{}", rhx.hexdump(&v));
//! ```
//!
//! ```text, no_run
//! ................ | 0100 0302 0504 0706 0908 0b0a 0d0c 0f0e :: 00000000
//! ................ | 1110 1312 1514 1716 1918 1b1a 1d1c 1f1e :: 00000010
//! .!"#$%&'()*+,-./ | 2120 2322 2524 2726 2928 2b2a 2d2c 2f2e :: 00000020
//! 0123456789:;<=>? | 3130 3332 3534 3736 3938 3b3a 3d3c 3f3e :: 00000030
//! ```
//!
//!
//! ### Customized Hexdump - Masking Duplicate Lines
//!
//! ```
//! use rhexdump;
//!
//! let a = [0u8; 0x10000];
//!
//! let mut rhx = rhexdump::Rhexdump::default();
//! rhx.display_duplicate_lines(false);
//!
//! println!("{}", rhx.hexdump(&a));
//! ```
//!
//! ```text, no_run
//! 00000000: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 | ................
//! *
//! 0000fff0: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 | ................
//! ```
//!
//!
//! ### Iterators Over a File (or a Byte Array)
//!
//! ```
//! use rhexdump;
//!
//! use std::fs::OpenOptions;
//!
//! let mut f = OpenOptions::new()
//!     .read(true)
//!     .open("/dev/random")
//!     .expect("Cannot open /dev/random");
//! let rhx = rhexdump::Rhexdump::default();
//!
//! for line in rhx.iter_file(&mut f, Some(0x80)) {
//!     println!("{}", line);
//! }
//! ```
//!
//! ```text, no_run
//! 00000000: bc ab de a1 07 db 95 8a 1c 57 24 ba 71 34 c5 5e | .........W$.q4.^
//! 00000010: 6f 58 6b c3 27 4d ef ae 0b 5d 5d 39 80 c6 42 2b | oXk.'M...]]9..B+
//! 00000020: 8b e2 3a 02 c1 6a 77 9f 69 99 65 4d 3e ac f4 e5 | ..:..jw.i.eM>...
//! 00000030: 7f ff 98 77 31 78 7b 65 32 15 7e 46 68 03 10 69 | ...w1x{e2.~Fh..i
//! 00000040: ad cd df fa e7 d8 21 79 7c c4 4b 11 48 c1 49 c3 | ......!y|.K.H.I.
//! 00000050: 56 df e0 eb de cc 9b fa 29 34 63 d1 d8 0e 63 5f | V.......)4c...c_
//! 00000060: e5 6c 10 5e 16 30 8b ed 45 41 ed 79 6c 3f b9 5a | .l.^.0..EA.yl?.Z
//! 00000070: 5e 50 eb 49 45 66 38 e5 8d 92 1b 8c 9e fe c6 c8 | ^P.IEf8.........
//! ```
//!

use std::convert::TryInto;
use std::default::Default;
use std::io::prelude::*;

/// Numeral bases supported by rhexdump.
///
///  - `Bin` formats the output in base 2
///  - `Oct` formats the output in base 8
///  - `Dec` formats the output in base 10
///  - `Hex` formats the output in base 16
#[derive(Debug, PartialEq)]
pub enum Base {
    Bin = 2,
    Oct = 8,
    Dec = 10,
    Hex = 16,
}

/// Endianess supported by rhexdump.
pub enum Endianess {
    BigEndian,
    LittleEndian,
}

/// Format information starting marker
const FMT_START_MARK: &str = "#[";
/// Format information end marker
const FMT_END_MARK: &str = "]";

/// Types of information that can be configured to be displayed (offsets, ascii output, etc.).
#[derive(Debug, PartialEq)]
pub enum FormatType {
    /// Displays the line offset.
    OFFSET,
    /// Displays the formatted raw bytes in the base selected by the user
    RAW,
    /// Displays an ASCII representation of the dumped bytes
    ASCII,
}

/// Represents the hexdump output format.
#[derive(Debug, PartialEq)]
struct Format {
    /// `info` is a list of `FormatType` that contains the type of information we want to display
    /// and the order in which we want them to appear.
    info: Vec<FormatType>,
    /// `separators` is a list of `String` that contains, as its suggests, the separators between
    /// the information we want to display. The first and last one, whether they are empty are not,
    /// are always, respectively, the prefix and suffix.
    separators: Vec<String>,
}

/// Maximum number of bytes per group
const MAX_BYTES_PER_GROUP: usize = 8;

/// Main object used to configure the output format.
pub struct Rhexdump {
    /// Base offset from which the line offsets start (not an actual data offset).
    base: Base,
    /// The endianess output format.
    endianess: Endianess,
    /// Format object that determines the format of a single line of output.
    format: Format,
    /// Formatted bytes can be grouped together. If the actual data is `de ad be ef`, grouping them
    /// by two with a little endian output format would result in `adde efbe`.
    /// `bytes_per_group` is the number of bytes in such a group.
    bytes_per_group: u8,
    /// Number of bytes we want to format per line.
    bytes_per_line: u8,
    /// Specifies if we want to omit duplicate lines and replace them by a single '*'.
    display_duplicate_lines: bool,
}

/// Error types.
#[derive(Debug, PartialEq)]
pub enum RhexdumpError {
    InvalidArgument,
    UnknownFormatType(String),
}

/// Default implementation for Rhexdump
///
/// This is equivalent to calling Rhexdump as follows:
///
/// ```
/// use rhexdump::*;
///
/// let rhx = Rhexdump::new(
///     Base::Hex,
///     Endianess::LittleEndian,
///     1,
///     16,
///     true,
///     "#[OFFSET]: #[RAW] | #[ASCII]"
/// );
/// ```
impl Default for Rhexdump {
    fn default() -> Self {
        Rhexdump {
            base: Base::Hex,
            endianess: Endianess::LittleEndian,
            bytes_per_group: 1,
            bytes_per_line: 16,
            display_duplicate_lines: true,
            format: Rhexdump::format_parse("#[OFFSET]: #[RAW] | #[ASCII]").unwrap(),
        }
    }
}

impl<'r, 'd, 'f> Rhexdump {
    /// Creates a customized instance of rhexdump.
    ///
    /// Supported output formats are determined by [FormatType].
    ///
    /// The expected format of the information provided in the `format` string is simply the
    /// corresponding `FormatType` name enclosed in `#[...]`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rhexdump::*;
    ///
    /// let rhx = Rhexdump::new(
    ///     Base::Hex,                             // Raw bytes will be displayed in hex format
    ///     Endianess::LittleEndian,               // Byte groups will be interpreted as LE
    ///     2,                                     // Byte groups are 2-byte long
    ///     16,                                    // There are 16 bytes per line
    ///     true,                                  // Duplicate lines will be displayed
    ///     "# #[OFFSET] -- #[RAW] || #[ASCII] #"  // Output format string
    /// );
    /// ```
    ///
    /// Using this instance of hexdump on an array filled with zeros would display something
    /// similar to this:
    ///
    /// ```text, no_run
    /// ## 00000000 -- 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 || ................ #
    /// ## 00000010 -- 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 || ................ #
    /// ## 00000020 -- 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 || ................ #
    /// ## 00000030 -- 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 || ................ #
    /// [...]
    /// ```
    ///
    /// If you don't need any customization, a default implementation also exists
    /// ```
    /// use rhexdump::*;
    ///
    /// let rhx = Rhexdump::default();
    ///
    /// // Equivalent to:
    /// let rhx = Rhexdump::new(
    ///     Base::Hex,
    ///     Endianess::LittleEndian,
    ///     1,
    ///     16,
    ///     true,
    ///     "#[OFFSET]: #[RAW] | #[ASCII]"
    /// );
    /// ```
    pub fn new(
        base: Base,
        endianess: Endianess,
        bytes_per_group: u8,
        bytes_per_line: u8,
        display_duplicate_lines: bool,
        format: &str,
    ) -> Result<Self, RhexdumpError> {
        Ok(Self {
            base,
            endianess,
            bytes_per_group,
            bytes_per_line,
            display_duplicate_lines,
            format: Self::format_parse(format)?,
        })
    }

    /// Scans the format string provided by the user to determine the format in which data should
    /// be displayed.
    fn format_parse(fmt: &str) -> Result<Format, RhexdumpError> {
        let mut offset = 0;
        let mut info = vec![];
        let mut separators = vec![];

        // Scans the format string for the two marks signaling the beginning and the end of the
        // info we want to display. These marks are respectively `FMT_START_MARK` and
        // `FMT_END_MARK`.
        // What's inside those two marks is then parsed as a `FormatType` and then put into
        // the info vector. The rest are treated as separators.
        loop {
            // Looks for the starting mark `FMT_START_MARK`.
            match fmt[offset..].find(&FMT_START_MARK) {
                None => break,
                Some(info_start) => {
                    let info_start = offset + info_start;
                    // Looks for the end mark `FMT_END_MARK`.
                    match fmt[info_start..].find(&FMT_END_MARK) {
                        None => break,
                        Some(info_end) => {
                            let info_end = info_start + info_end + 1;
                            // The characters from the beginning offset (of this loop iteration) to
                            // the start of the info mark form a separator.
                            // The first separator is our prefix.
                            separators.push(fmt[offset..info_start].to_string());
                            // The string located between `FMT_START_MARK` and `FMT_END_MARK`
                            // is parsed as `FormatType`.
                            match &fmt
                                [info_start + FMT_START_MARK.len()..info_end - FMT_END_MARK.len()]
                            {
                                "OFFSET" => info.push(FormatType::OFFSET),
                                "RAW" => info.push(FormatType::RAW),
                                "ASCII" => info.push(FormatType::ASCII),
                                x => return Err(RhexdumpError::UnknownFormatType(x.to_string())),
                            }
                            // We then restart the process from the end of the latest info we
                            // parsed
                            offset = info_end;
                        }
                    }
                }
            }
        }
        // Push the rest of the string, whether it's empty or not as a separator
        // This will be our suffix.
        separators.push(fmt[offset..].to_string());

        // The `info` and `separators` must differ in length by only one element.
        assert_eq!(info.len() + 1, separators.len());

        Ok(Format { info, separators })
    }

    /// Returns the formatted string for the ascii and byte outputs based on the configuration
    /// associated to the current instance.
    fn format_line_raw_ascii(&self, data: &[u8]) -> (String, String) {
        // Computes the maximum value that can be formatted if we group bytes by `bytes_per_group`.
        let max_value = 2u128.pow(8 * self.bytes_per_group as u32) - 1;
        // Uses this maximum value to compute the length of a given group.
        // A given group will be padded with zeros until the size reaches `fill`.
        let fill = match self.base {
            Base::Bin => format!("{:b}", max_value).len(),
            Base::Oct => format!("{:o}", max_value).len(),
            Base::Dec => format!("{:}", max_value).len(),
            Base::Hex => format!("{:x}", max_value).len(),
        };
        let mut ascii = String::new();
        // Iterates over chunks of size `bytes_per_group`, format each group and concatenates them.
        // We also take advantage of this iterator to compute the associated ascii output.
        let raw = data
            .chunks(self.bytes_per_group as usize)
            .map(|b| {
                // Formats the current bytes and adds them to the ascii string.
                for &c in b.iter() {
                    ascii.push(if c.is_ascii_graphic() { c as char } else { '.' });
                }
                // Hackish way to convert the current chunk of bytes into a u64.
                // The chunk is first converted into a vector.
                let mut value_vec = b.to_vec();
                // Then it gets resized to `MAX_BYTES_PER_GROUP` so that we can be sure that
                // we have 8 bytes for the u64 convertion coming next.
                value_vec.resize(MAX_BYTES_PER_GROUP, 0);
                // We call `from_le_bytes` or `from_be_bytes` based on the endianess value in the
                // configuration.
                let value = match self.endianess {
                    Endianess::LittleEndian => {
                        u64::from_le_bytes(value_vec.as_slice().try_into().unwrap())
                    }
                    Endianess::BigEndian => {
                        // In the `BigEndian` case, we rotate the bytes in the array.
                        // Here's an example that illustrates why we need to do this.
                        // Suppose we have:
                        //      b = [0xde, 0xad];
                        // Once it gets resized, we get:
                        //      value_vec = [0xde, 0xad, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
                        // The little endian conversion will return 0xadde, as expected.
                        // However, the big endian conversion will interpret all these zeros as
                        // part of the value and not just paddind.
                        // What we want is to have:
                        //      value_vec = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xde, 0xad];
                        // which is why bytes get rotated inside the vector.
                        value_vec.rotate_right((MAX_BYTES_PER_GROUP - b.len()) as usize);
                        u64::from_be_bytes(value_vec.as_slice().try_into().unwrap())
                    }
                };
                // We finally format the current group dependinf on the base associated to the
                // instance.
                match self.base {
                    Base::Bin => format!("{:0f$b}", value, f = fill),
                    Base::Oct => format!("{:0f$o}", value, f = fill),
                    Base::Dec => format!("{:0f$}", value, f = fill),
                    Base::Hex => format!("{:0f$x}", value, f = fill),
                }
            })
            .collect::<Vec<String>>()
            .join(" ");
        (raw, ascii)
    }

    /// Formats a single line of output based on the format associated to the current instance.
    fn format_line(&self, offset: u32, raw: String, ascii: String) -> String {
        let mut output = String::new();

        // Iterates over the information type and the separators to format the line
        for (info, sep) in self.format.info.iter().zip(self.format.separators.iter()) {
            output = match info {
                FormatType::ASCII => format!("{}{}{}", output, sep, ascii),
                FormatType::OFFSET => format!("{}{}{:08x}", output, sep, offset),
                FormatType::RAW => format!("{}{}{}", output, sep, raw),
            };
        }

        // We can unwrap here because we know we have a suffix in `separators`
        format!("{}{}", output, self.format.separators.last().unwrap())
    }

    /// Sets the numeral base of the current instance.
    pub fn set_base(&mut self, base: Base) {
        self.base = base;
    }

    /// Sets the number of bytes per group of the current instance.
    pub fn set_bytes_per_group(&mut self, size: u8) -> Result<(), RhexdumpError> {
        if size as usize > MAX_BYTES_PER_GROUP {
            return Err(RhexdumpError::InvalidArgument);
        }
        self.bytes_per_group = size;
        Ok(())
    }

    /// Sets the number of bytes per line of the current instance.
    pub fn set_bytes_per_line(&mut self, count: u8) {
        self.bytes_per_line = count;
    }

    /// Sets whether or not duplicate lines should be displayed.
    pub fn display_duplicate_lines(&mut self, display: bool) {
        self.display_duplicate_lines = display;
    }

    /// Sets the format of the current instance. See [Rhexdump::new] for examples of accepted
    /// formats.
    pub fn set_format(&mut self, format: &str) -> Result<(), RhexdumpError> {
        self.format = Rhexdump::format_parse(format)?;
        Ok(())
    }

    /// Returns an iterator over a byte array
    ///
    /// # Examples
    ///
    /// ```
    /// use rhexdump::*;
    ///
    /// let v = (0..0x10).collect::<Vec<u8>>();
    /// let rhx = Rhexdump::default();
    ///
    /// for line in rhx.iter(&v) {
    ///     println!("{}", line);
    /// }
    /// ```
    pub fn iter(&'r self, data: &'d [u8]) -> RhexdumpIter<'r, 'd> {
        RhexdumpIter {
            rhx: self,
            base_offset: 0,
            data,
            offset: 0,
            raw_size: 0,
            ascii_size: 0,
            prev_line: None,
            duplicate_line_displayed: false,
        }
    }

    /// Returns an iterator over a byte array and starts the offset from `base_offset`
    ///
    /// # Examples
    ///
    /// ```
    /// use rhexdump::*;
    ///
    /// let v = (0..0x10).collect::<Vec<u8>>();
    /// let rhx = Rhexdump::default();
    ///
    /// for line in rhx.iter_offset(&v, 0x1000) {
    ///     println!("{}", line);
    /// }
    /// ```
    pub fn iter_offset(&'r self, data: &'d [u8], base_offset: u32) -> RhexdumpIter<'r, 'd> {
        RhexdumpIter {
            rhx: self,
            base_offset,
            data,
            offset: 0,
            raw_size: 0,
            ascii_size: 0,
            prev_line: None,
            duplicate_line_displayed: false,
        }
    }

    /// Returns an iterator over a file
    ///
    /// # Examples
    ///
    /// ```
    /// use rhexdump::*;
    /// use std::fs::OpenOptions;
    ///
    /// let mut f = OpenOptions::new()
    ///     .read(true)
    ///     .open("/dev/random")
    ///     .expect("Cannot open /dev/random");
    /// let rhx = Rhexdump::default();
    ///
    /// for line in rhx.iter_file(&mut f, Some(0x1000)) {
    ///     println!("{}", line);
    /// }
    /// ```
    pub fn iter_file<F: Read>(
        &'r self,
        file: &'f mut F,
        size: Option<usize>,
    ) -> RhexdumpFileIter<'r, 'f, F> {
        RhexdumpFileIter {
            rhx: self,
            base_offset: 0,
            file,
            size,
            offset: 0,
            raw_size: 0,
            ascii_size: 0,
            prev_line: None,
            duplicate_line_displayed: false,
        }
    }

    /// Returns an iterator over a file and starts the offset from `base_offset`
    ///
    /// # Examples
    ///
    /// ```
    /// use rhexdump::*;
    /// use std::fs::OpenOptions;
    ///
    /// let mut f = OpenOptions::new()
    ///     .read(true)
    ///     .open("/dev/random")
    ///     .expect("Cannot open /dev/random");
    /// let rhx = Rhexdump::default();
    ///
    /// for line in rhx.iter_file_offset(&mut f, Some(0x1000), 0x1000) {
    ///     println!("{}", line);
    /// }
    /// ```
    pub fn iter_file_offset<F: Read>(
        &'r self,
        file: &'f mut F,
        size: Option<usize>,
        base_offset: u32,
    ) -> RhexdumpFileIter<'r, 'f, F> {
        RhexdumpFileIter {
            rhx: self,
            base_offset,
            file,
            size,
            offset: 0,
            raw_size: 0,
            ascii_size: 0,
            prev_line: None,
            duplicate_line_displayed: false,
        }
    }

    /// Hexdumps `data` according to the configuration of the current instance.
    pub fn hexdump(&self, data: &[u8]) -> String {
        self.iter(data).collect::<Vec<String>>().join("\n")
    }

    /// Hexdumps `data` starting from the offset `offset` according to the configuration of the
    /// current instance.
    pub fn hexdump_offset(&self, data: &[u8], offset: u32) -> String {
        self.iter_offset(data, offset)
            .collect::<Vec<String>>()
            .join("\n")
    }

    /// Hexdumps a file according to the configuration of the current instance.
    pub fn hexdump_file<F: Read>(&self, file: &'f mut F, size: Option<usize>) -> String {
        self.iter_file(file, size)
            .collect::<Vec<String>>()
            .join("\n")
    }

    /// Hexdumps a file starting from the offset `offset` according to the configuration of the
    /// current instance.
    pub fn hexdump_file_offset<F: Read>(
        &self,
        file: &'f mut F,
        size: Option<usize>,
        offset: u32,
    ) -> String {
        self.iter_file_offset(file, size, offset)
            .collect::<Vec<String>>()
            .join("\n")
    }
}

/// Hexdumps `data` using a default configuration.
pub fn hexdump(data: &[u8]) -> String {
    Rhexdump::default().hexdump(data)
}

/// Hexdumps `data` starting from the offset `offset` using a default configuration.
pub fn hexdump_offset(data: &[u8], offset: u32) -> String {
    Rhexdump::default().hexdump_offset(data, offset)
}

/// Hexdumps a file using a default configuration.
pub fn hexdump_file<F: Read>(file: &mut F, size: Option<usize>) -> String {
    Rhexdump::default().hexdump_file(file, size)
}

/// Hexdumps a file starting from the offset `offset` using a default configuration.
pub fn hexdump_file_offset<F: Read>(file: &mut F, size: Option<usize>, offset: u32) -> String {
    Rhexdump::default().hexdump_file_offset(file, size, offset)
}

/// Iterator over a slice of bytes that returns one formatted line at a time.
pub struct RhexdumpIter<'r, 'd> {
    /// The original Rhexdump object.
    rhx: &'r Rhexdump,
    /// The base offset from which we want to start displaying data.
    base_offset: u32,
    /// The byte array we want to format.
    data: &'d [u8],
    /// The current offset into `data`. Gets incremented after each iterator's step.
    offset: usize,
    /// The number of ascii characters in a line.
    /// Computed dynamically and used to pad the last line if it's not big enough.
    ascii_size: usize,
    /// The number of formatted raw bytes in a line.
    /// Computed dynamically and used to pad the last line if it's not big enough.
    raw_size: usize,
    /// The raw bytes of the previous line that was returned by the iterator.
    /// Used to identify duplicate lines.
    prev_line: Option<Vec<u8>>,
    /// State value to know whether or not we've already displayed the duplicate line characters '*'
    duplicate_line_displayed: bool,
}

impl<'r, 'd> Iterator for RhexdumpIter<'r, 'd> {
    type Item = String;

    /// Returns one line of formatted bytes from the byte array according to the configuration of
    /// the associated Rhexdump object.
    fn next(&mut self) -> Option<Self::Item> {
        // Stops iterating when we are outside the buffer
        if self.offset >= self.data.len() {
            return None;
        }

        let mut start;
        let mut end;

        // Duplicate detection loop
        loop {
            // Computes the start and end of the slice we want to read data from
            start = self.offset as usize;
            end = self.offset + self.rhx.bytes_per_line as usize;
            self.offset += self.rhx.bytes_per_line as usize;

            // Truncates the end offset if it goes out of bound. It also resets the previous line
            // displayed to always show the last line, whether it's a duplicate or not.
            if end >= self.data.len() {
                self.prev_line = None;
                end = self.data.len();
                break;
            }

            // If we don't want to display duplicate lines...
            if !self.rhx.display_duplicate_lines && self.prev_line.is_some() {
                let is_duplicate = self.data[start..end]
                    .iter()
                    .zip(self.prev_line.as_ref().unwrap().iter())
                    .all(|(&a, &b)| a == b);
                // ... and the current one is a duplicate of the previous one...
                if is_duplicate {
                    // ... then ignore the current line and restart the process with the next
                    // one if we have already displayed the '*' character...
                    if self.duplicate_line_displayed {
                        continue;
                    }
                    // ... otherwise, display '*' and store the fact that it was shown.
                    self.duplicate_line_displayed = true;
                    return Some(String::from("*"));
                }
            }
            break;
        }

        // Formats data between `start` and `end` and retrieves the raw bytes as well as the
        // ascii outputs.
        let (mut raw, mut ascii) = self.rhx.format_line_raw_ascii(&self.data[start..end]);

        // Fill out the line to the right if the raw or ascii output is not large enough.
        // This is normally the case for the last line of the hexdump.
        if raw.len() < self.raw_size {
            raw = format!("{:<fill$}", raw, fill = self.raw_size);
        }
        if ascii.len() < self.ascii_size {
            ascii = format!("{:<fill$}", ascii, fill = self.ascii_size);
        }
        self.raw_size = raw.len();
        self.ascii_size = ascii.len();

        // If we reached this point, we can update the current previous line if we don't want
        // to display duplicates.
        if !self.rhx.display_duplicate_lines {
            self.prev_line = Some(self.data[start..end].to_vec());
            self.duplicate_line_displayed = false;
        }

        // Returns the formatted current line.
        Some(
            self.rhx
                .format_line(self.base_offset + start as u32, raw, ascii),
        )
    }
}

/// Iterator over a file that returns one formatted line at a time.
pub struct RhexdumpFileIter<'r, 'f, F: Read> {
    /// The original Rhexdump object.
    rhx: &'r Rhexdump,
    /// The base offset from which we want to start displaying data.
    base_offset: u32,
    /// File we want to read data from.
    file: &'f mut F,
    /// Size we want to read. A `None` value means we read the entire file.
    size: Option<usize>,
    /// The current offset into the file. Gets incremented after each iterator's step.
    offset: usize,
    /// The number of ascii characters in a line.
    /// Computed dynamically and used to pad the last line if it's not big enough.
    ascii_size: usize,
    /// The number of formatted raw bytes in a line.
    /// Computed dynamically and used to pad the last line if it's not big enough.
    raw_size: usize,
    /// The raw bytes of the previous line that was returned by the iterator.
    /// Used to identify duplicate lines.
    prev_line: Option<Vec<u8>>,
    /// State value to know whether or not we've already displayed the duplicate line characters '*'
    duplicate_line_displayed: bool,
}

impl<'r, 'f, F: Read> Iterator for RhexdumpFileIter<'r, 'f, F> {
    type Item = String;

    /// Returns one line of formatted bytes from the file according to the configuration of the
    /// associated Rhexdump object.
    fn next(&mut self) -> Option<Self::Item> {
        if self.size.is_some() && self.offset >= self.size.unwrap() {
            return None;
        }

        let start = self.offset as u32;
        let mut size_read;
        // Buffer of size `bytes_per_line` to read from the file
        let mut buffer = vec![0u8; self.rhx.bytes_per_line as usize];

        // Duplicate detection loop
        loop {
            // Reading data from the input file
            size_read = self.file.read(&mut buffer).unwrap();
            self.offset += size_read;
            // If there is no more data to read, returns None
            if size_read == 0 {
                return None;
            }
            // If we don't want to display duplicate lines...
            if !self.rhx.display_duplicate_lines && self.prev_line.is_some() {
                let is_duplicate = buffer
                    .iter()
                    .zip(self.prev_line.as_ref().unwrap().iter())
                    .all(|(&a, &b)| a == b);
                // ... and the current one is a duplicate of the previous one...
                if is_duplicate {
                    // ... then ignore the current line and restart the process with the next
                    // one if we have already displayed the '*' character...
                    if self.duplicate_line_displayed {
                        continue;
                    }
                    // ... otherwise, display '*' and store the fact that it was shown.
                    self.duplicate_line_displayed = true;
                    return Some(String::from("*"));
                }
            }
            break;
        }

        let end = if let Some(size) = self.size {
            std::cmp::min(size_read, size - start as usize)
        } else {
            size_read
        };
        // Formats data in `buffer` and retrieves the raw bytes as well as the ascii outputs.
        let (mut raw, mut ascii) = self.rhx.format_line_raw_ascii(&buffer[..end]);

        // Fill out the line to the right if the raw or ascii output is not large enough.
        // This is normally the case for the last line of the hexdump.
        if raw.len() < self.raw_size {
            raw = format!("{:<fill$}", raw, fill = self.raw_size);
        }
        if ascii.len() < self.ascii_size {
            ascii = format!("{:<fill$}", ascii, fill = self.ascii_size);
        }
        self.raw_size = raw.len();
        self.ascii_size = ascii.len();

        // If we reached this point, we can update the current previous line if we don't want
        // to display duplicates.
        if !self.rhx.display_duplicate_lines {
            self.prev_line = Some(buffer);
            self.duplicate_line_displayed = false;
        }

        // Returns the formatted current line.
        Some(self.rhx.format_line(self.base_offset + start, raw, ascii))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::{File, OpenOptions};
    use std::io::SeekFrom;

    #[test]
    fn rhx_default() {
        let rhx = Rhexdump::default();
        let a = [0u8; 0x38];
        let mut rhx_iter = rhx.iter(&a);

        assert_eq!(
            rhx_iter.next().unwrap(),
            String::from(
                "00000000: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 | ................"
            )
        );
        assert_eq!(
            rhx_iter.next().unwrap(),
            String::from(
                "00000010: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 | ................"
            )
        );
        assert_eq!(
            rhx_iter.next().unwrap(),
            String::from(
                "00000020: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 | ................"
            )
        );
        assert_eq!(
            rhx_iter.next().unwrap(),
            String::from(
                "00000030: 00 00 00 00 00 00 00 00                         | ........        "
            )
        );
        assert_eq!(rhx_iter.next().is_none(), true);
    }

    #[test]
    fn rhx_custom_bin() {
        let v = (0..0x10).collect::<Vec<u8>>();
        let rhx = Rhexdump::new(
            Base::Bin,
            Endianess::LittleEndian,
            2,
            8,
            true,
            "#[OFFSET] | #[RAW] | #[ASCII]",
        );
        assert_eq!(rhx.is_ok(), true);
        let rhx = rhx.unwrap();
        let mut rhx_iter = rhx.iter(&v);

        assert_eq!(
            rhx_iter.next().unwrap(),
            String::from(
                "00000000 | 0000000100000000 0000001100000010 0000010100000100 0000011100000110 | ........"
            )
        );
        assert_eq!(
            rhx_iter.next().unwrap(),
            String::from(
                "00000008 | 0000100100001000 0000101100001010 0000110100001100 0000111100001110 | ........"
            )
        );
        assert_eq!(rhx_iter.next().is_none(), true);
    }

    #[test]
    fn rhx_custom_oct() {
        let v = (0..0x10).collect::<Vec<u8>>();
        let rhx = Rhexdump::new(
            Base::Oct,
            Endianess::LittleEndian,
            4,
            4,
            true,
            "- #[OFFSET] - #[RAW] - #[ASCII] -",
        );
        assert_eq!(rhx.is_ok(), true);
        let rhx = rhx.unwrap();
        let mut rhx_iter = rhx.iter(&v);

        assert_eq!(
            rhx_iter.next().unwrap(),
            String::from("- 00000000 - 00300400400 - .... -")
        );
        assert_eq!(
            rhx_iter.next().unwrap(),
            String::from("- 00000004 - 00701402404 - .... -")
        );
        assert_eq!(
            rhx_iter.next().unwrap(),
            String::from("- 00000008 - 01302404410 - .... -")
        );
        assert_eq!(
            rhx_iter.next().unwrap(),
            String::from("- 0000000c - 01703406414 - .... -")
        );
        assert_eq!(rhx_iter.next().is_none(), true);
    }

    #[test]
    fn rhx_custom_dec() {
        let v = (0..0x10).collect::<Vec<u8>>();
        let rhx = Rhexdump::new(
            Base::Dec,
            Endianess::BigEndian,
            4,
            16,
            true,
            "# #[OFFSET] # #[RAW] # #[ASCII] #",
        );
        assert_eq!(rhx.is_ok(), true);
        let rhx = rhx.unwrap();
        let mut rhx_iter = rhx.iter(&v);

        assert_eq!(
            rhx_iter.next().unwrap(),
            String::from(
                "# 00000000 # 0000066051 0067438087 0134810123 0202182159 # ................ #"
            )
        );
        assert_eq!(rhx_iter.next().is_none(), true);
    }

    #[test]
    fn rhx_custom_hex() {
        let v = (0..0x10).collect::<Vec<u8>>();
        let rhx = Rhexdump::new(
            Base::Hex,
            Endianess::BigEndian,
            2,
            16,
            true,
            "#[ASCII] | #[RAW] | #[OFFSET]",
        );
        assert_eq!(rhx.is_ok(), true);
        let rhx = rhx.unwrap();
        let mut rhx_iter = rhx.iter(&v);

        assert_eq!(
            rhx_iter.next().unwrap(),
            String::from("................ | 0001 0203 0405 0607 0809 0a0b 0c0d 0e0f | 00000000")
        );
        assert_eq!(rhx_iter.next().is_none(), true);
    }

    #[test]
    fn rhx_base_offset() {
        let v = (0..0x10).collect::<Vec<u8>>();
        let rhx = Rhexdump::default();
        let mut rhx_iter = rhx.iter_offset(&v, 0x1000);

        assert_eq!(
            rhx_iter.next().unwrap(),
            String::from(
                "00001000: 00 01 02 03 04 05 06 07 08 09 0a 0b 0c 0d 0e 0f | ................"
            )
        );
        assert_eq!(rhx_iter.next().is_none(), true);
    }

    #[test]
    #[cfg(not(target_os = "windows"))]
    fn rhx_file() {
        let rhx = Rhexdump::default();
        let mut f = OpenOptions::new()
            .write(true)
            .read(true)
            .create(true)
            .open("/tmp/rhexdump.test")
            .expect("Cannot create /tmp/rhexdump.test");
        let v = (0..0x10).collect::<Vec<u8>>();
        f.write_all(&v).expect("Cannot write to /tmp/rhexdump.test");

        f.seek(SeekFrom::Start(0));
        let mut rhx_iter = rhx.iter_file(&mut f, None);
        assert_eq!(
            rhx_iter.next().unwrap(),
            String::from(
                "00000000: 00 01 02 03 04 05 06 07 08 09 0a 0b 0c 0d 0e 0f | ................"
            )
        );

        f.seek(SeekFrom::Start(0));
        let mut rhx_iter = rhx.iter_file(&mut f, Some(0x8));
        assert_eq!(
            rhx_iter.next().unwrap(),
            String::from("00000000: 00 01 02 03 04 05 06 07 | ........")
        );

        f.seek(SeekFrom::Start(0));
        let mut rhx_iter = rhx.iter_file_offset(&mut f, None, 0x1000);
        assert_eq!(
            rhx_iter.next().unwrap(),
            String::from(
                "00001000: 00 01 02 03 04 05 06 07 08 09 0a 0b 0c 0d 0e 0f | ................"
            )
        );

        f.seek(SeekFrom::Start(0));
        let mut rhx_iter = rhx.iter_file_offset(&mut f, Some(0x8), 0x1000);
        assert_eq!(
            rhx_iter.next().unwrap(),
            String::from("00001000: 00 01 02 03 04 05 06 07 | ........")
        );
    }
}
