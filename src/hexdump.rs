//! Hexdump interfaces and utilities.

use std::fmt;
use std::io::{self, Cursor, Read, Write};

use crate::config::*;
use crate::iter::*;

// ===============================================================================================
// Generic Rhexdump
// ===============================================================================================

/// Formats data from a source implementing [`std::io::Read`] to a destination implementing
/// [`std::io::Write`].
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Rhexdump {
    /// Configuration object.
    config: RhexdumpConfig,
}

impl Rhexdump {
    /// Creates a new instance of `Rhexdump` with the following default parameters:
    ///
    /// - **Base**: hexadecimal
    /// - **Endianness**: little endian
    /// - **Offset bit width**: 32 bits
    /// - **Group size**: byte (8-bit)
    /// - **Groups per line**: 16
    /// - **Bytes per line**: 16
    /// - **Hide duplicate lines**: no
    ///
    /// # Example:
    ///
    /// ```
    /// use rhexdump::prelude::*;
    ///
    /// let rhx = Rhexdump::new();
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a new instance of `Rhexdump` using the configuration passed as argument.
    ///
    /// # Example
    ///
    /// ```
    /// use rhexdump::prelude::*;
    ///
    /// let config = RhexdumpBuilder::new().config();
    /// let rhx = Rhexdump::with_config(config);
    /// ```
    pub fn with_config(config: RhexdumpConfig) -> Self {
        Self { config }
    }

    /// Hexdumps, with an offset, data from a source implementing the [`std::io::Read`] trait into
    /// a destination implementing the [`std::io::Write`] trait.
    ///
    /// # Example
    ///
    /// The following example formats a `String` into a `File`.
    ///
    /// ```
    /// use rhexdump::prelude::*;
    /// use std::env::temp_dir;
    /// use std::fs::OpenOptions;
    /// use std::io::prelude::*;
    /// use std::io::*;
    ///
    /// // Create a Rhexdump instance.
    /// let rhx = Rhexdump::new();
    ///
    /// // Create a temporary output file.
    /// let filename = "rhx_rhexdump_hexdump_offset.doctest";
    /// let mut test_file = temp_dir();
    /// test_file.push(filename);
    /// let mut f = OpenOptions::new()
    ///     .write(true)
    ///     .read(true)
    ///     .create(true)
    ///     .truncate(true)
    ///     .open(test_file)
    ///     .expect(&format!("Cannot create {}", filename));
    ///
    /// // String that will be formatted.
    /// let input = String::from("Lorem ipsum dolor sit amet, consectetur adipiscing elit");
    /// let mut cur = Cursor::new(&input);
    ///
    /// // Formatting data to the temp file.
    /// rhx.hexdump_offset(&mut f, &mut cur, 0x12340000);
    ///
    /// // Reading the temp file content and making sure it corresponds to the expected output.
    /// f.seek(SeekFrom::Start(0))
    ///     .expect(&format!("Could not seek to start of {}", filename));
    /// let mut output = Vec::new();
    /// f.read_to_end(&mut output)
    ///     .expect(&format!("Cannot read from {}", filename));
    /// assert_eq!(
    ///     &String::from_utf8_lossy(&output),
    ///     "12340000: 4c 6f 72 65 6d 20 69 70 73 75 6d 20 64 6f 6c 6f  Lorem.ipsum.dolo\n\
    ///      12340010: 72 20 73 69 74 20 61 6d 65 74 2c 20 63 6f 6e 73  r.sit.amet,.cons\n\
    ///      12340020: 65 63 74 65 74 75 72 20 61 64 69 70 69 73 63 69  ectetur.adipisci\n\
    ///      12340030: 6e 67 20 65 6c 69 74                             ng.elit\n"
    /// );
    /// ```
    #[inline]
    pub fn hexdump_offset<W: Write, R: Read>(&self, dst: &mut W, src: &mut R, offset: u64) {
        let iter = RhexdumpIter::new(*self, dst, src).offset(offset);
        iter.for_each(|_| {});
    }

    /// Hexdumps data from a source implementing the [`std::io::Read`] trait into a destination
    /// implementing the [`std::io::Write`] trait.
    ///
    /// # Example
    ///
    /// ```
    /// use rhexdump::prelude::*;
    /// use std::env::temp_dir;
    /// use std::fs::OpenOptions;
    /// use std::io::prelude::*;
    /// use std::io::*;
    ///
    /// // Create a Rhexdump instance.
    /// let rhx = Rhexdump::new();
    ///
    /// // Create a temporary output file.
    /// let filename = "rhx_rhexdump_hexdump.doctest";
    /// let mut test_file = temp_dir();
    /// test_file.push(filename);
    /// let mut f = OpenOptions::new()
    ///     .write(true)
    ///     .read(true)
    ///     .create(true)
    ///     .truncate(true)
    ///     .open(test_file)
    ///     .expect(&format!("Cannot create {}", filename));
    ///
    /// // String that will be formatted.
    /// let input = String::from("Lorem ipsum dolor sit amet, consectetur adipiscing elit");
    /// let mut cur = Cursor::new(&input);
    ///
    /// // Formatting data to the temp file.
    /// rhx.hexdump(&mut f, &mut cur);
    ///
    /// // Reading the temp file content and making sure it corresponds to the expected output.
    /// f.seek(SeekFrom::Start(0))
    ///     .expect(&format!("Could not seek to start of {}", filename));
    /// let mut output = Vec::new();
    /// f.read_to_end(&mut output)
    ///     .expect(&format!("Cannot read from {}", filename));
    /// assert_eq!(
    ///     &String::from_utf8_lossy(&output),
    ///     "00000000: 4c 6f 72 65 6d 20 69 70 73 75 6d 20 64 6f 6c 6f  Lorem.ipsum.dolo\n\
    ///      00000010: 72 20 73 69 74 20 61 6d 65 74 2c 20 63 6f 6e 73  r.sit.amet,.cons\n\
    ///      00000020: 65 63 74 65 74 75 72 20 61 64 69 70 69 73 63 69  ectetur.adipisci\n\
    ///      00000030: 6e 67 20 65 6c 69 74                             ng.elit\n"
    /// );
    /// ```
    #[inline]
    pub fn hexdump<W: Write, R: Read>(&self, dst: &mut W, src: &mut R) {
        self.hexdump_offset(dst, src, 0)
    }

    /// Creates an iterator over a data source implementing [`std::io::Read`] and formats it to
    /// a destination implementing [`std::io::Write`].
    ///
    /// # Example
    ///
    /// ```
    /// use rhexdump::prelude::*;
    /// use std::env::temp_dir;
    /// use std::fs::OpenOptions;
    /// use std::io::prelude::*;
    /// use std::io::*;
    ///
    /// // Create a Rhexdump instance.
    /// let rhx = Rhexdump::new();
    ///
    /// // Create a temporary output file.
    /// let filename = "rhx_rhexdump_iter_offset.doctest";
    /// let mut test_file = temp_dir();
    /// test_file.push(filename);
    /// let mut f = OpenOptions::new()
    ///     .write(true)
    ///     .read(true)
    ///     .create(true)
    ///     .truncate(true)
    ///     .open(test_file)
    ///     .expect(&format!("Cannot create {}", filename));
    ///
    /// // String that will be formatted.
    /// let input = String::from("Lorem ipsum dolor sit amet, consectetur adipiscing elit");
    /// let mut cur = Cursor::new(&input);
    ///
    /// // Formatting data to the temp file.
    /// let mut iter = rhx.iter(&mut f, &mut cur).offset(0x12340000);
    /// // Formatting only two lines.
    /// iter.next();
    /// iter.next();
    ///
    /// // Reading the temp file content and making sure it corresponds to the expected output.
    /// f.seek(SeekFrom::Start(0))
    ///     .expect(&format!("Could not seek to start of {}", filename));
    /// let mut output = Vec::new();
    /// f.read_to_end(&mut output)
    ///     .expect(&format!("Cannot read from {}", filename));
    /// assert_eq!(
    ///     &String::from_utf8_lossy(&output),
    ///     "12340000: 4c 6f 72 65 6d 20 69 70 73 75 6d 20 64 6f 6c 6f  Lorem.ipsum.dolo\n\
    ///      12340010: 72 20 73 69 74 20 61 6d 65 74 2c 20 63 6f 6e 73  r.sit.amet,.cons\n"
    /// );
    pub fn iter<'r, 'w, R: Read, W: Write>(
        &self,
        dst: &'w mut W,
        src: &'r mut R,
    ) -> RhexdumpIter<'r, 'w, R, W, Self> {
        RhexdumpIter::new(*self, dst, src)
    }
}

unsafe impl Send for Rhexdump {}
unsafe impl Sync for Rhexdump {}

impl fmt::Display for Rhexdump {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Rhexdump {{ \
                base: {}, \
                endianness: {}, \
                bit_width: {}, \
                group_size: {}, \
                groups_per_line: {}, \
                hide_duplicate_lines: {} \
            }}",
            self.config.base,
            self.config.endianness,
            self.config.bit_width,
            self.config.group_size,
            self.config.groups_per_line,
            self.config.hide_duplicate_lines,
        )
    }
}

impl From<RhexdumpConfig> for Rhexdump {
    fn from(config: RhexdumpConfig) -> Self {
        Self::with_config(config)
    }
}

impl RhexdumpGetConfig for Rhexdump {
    #[inline]
    fn get_config(&self) -> RhexdumpConfig {
        self.config
    }
}

// ===============================================================================================
// String Rhexdump
// ===============================================================================================

/// Formats byte slices and data from a source implementing [`std::io::Read`] to a [`String`].
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct RhexdumpString {
    /// Configuration object
    config: RhexdumpConfig,
}

impl RhexdumpString {
    /// Creates a new instance of `RhexdumpString` with the following default parameters:
    ///
    /// - **Base**: hexadecimal
    /// - **Endianness**: little endian
    /// - **Offset bit width**: 32 bits
    /// - **Group size**: byte (8-bit)
    /// - **Groups per line**: 16
    /// - **Bytes per line**: 16
    /// - **Hide duplicate lines**: no
    ///
    /// # Example:
    ///
    /// ```
    /// use rhexdump::prelude::*;
    ///
    /// let rhx = RhexdumpString::new();
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a new instance of `RhexdumpString` using the configuration passed as argument.
    ///
    /// # Example
    ///
    /// ```
    /// use rhexdump::prelude::*;
    ///
    /// let config = RhexdumpBuilder::new().config();
    /// let rhx = Rhexdump::with_config(config);
    /// ```
    pub fn with_config(config: RhexdumpConfig) -> Self {
        Self { config }
    }

    /// Hexdumps, with an offset, data from a source implementing [`std::io::Read`] to a
    /// [`String`].
    ///
    /// # Example
    ///
    /// ```
    /// use rhexdump::prelude::*;
    ///
    /// let v = (0..0x14).collect::<Vec<u8>>();
    /// let rh = RhexdumpString::new();
    /// let mut cur = std::io::Cursor::new(&v);
    /// let out = rh.hexdump_offset(&mut cur, 0x12340000);
    /// assert_eq!(
    ///     &out,
    ///     "12340000: 00 01 02 03 04 05 06 07 08 09 0a 0b 0c 0d 0e 0f  ................\n\
    ///      12340010: 10 11 12 13                                      ....\n"
    /// );
    /// ```
    #[inline]
    pub fn hexdump_offset<R: Read>(&self, src: &mut R, offset: u64) -> String {
        let mut dst = Vec::new();
        let iter = RhexdumpIter::new(*self, &mut dst, src).offset(offset);
        iter.for_each(|_| {});
        // UNSAFE: every single byte is a result of the hexdump formatting. We are therefore sure
        //         that it is valid UTF-8 and we can proceed to convert the vec to string without
        //         any check.
        unsafe { String::from_utf8_unchecked(dst) }
    }

    /// Hexdumps data from a source implementing [`std::io::Read`] to a [`String`].
    ///
    /// # Example
    ///
    /// ```
    /// use rhexdump::prelude::*;
    ///
    /// let v = (0..0x14).collect::<Vec<u8>>();
    /// let rh = RhexdumpString::new();
    /// let mut cur = std::io::Cursor::new(&v);
    /// let out = rh.hexdump(&mut cur);
    /// assert_eq!(
    ///     &out,
    ///     "00000000: 00 01 02 03 04 05 06 07 08 09 0a 0b 0c 0d 0e 0f  ................\n\
    ///      00000010: 10 11 12 13                                      ....\n"
    /// );
    /// ```
    #[inline]
    pub fn hexdump<R: Read>(&self, src: &mut R) -> String {
        self.hexdump_offset(src, 0)
    }

    /// Hexdumps, with an offset, a slice of bytes to a [`String`].
    ///
    /// # Example
    ///
    /// ```
    /// use rhexdump::prelude::*;
    ///
    /// let v = (0..0x14).collect::<Vec<u8>>();
    /// let rh = RhexdumpString::new();
    /// let out = rh.hexdump_bytes_offset(&v, 0x12340000);
    /// assert_eq!(
    ///     &out,
    ///     "12340000: 00 01 02 03 04 05 06 07 08 09 0a 0b 0c 0d 0e 0f  ................\n\
    ///      12340010: 10 11 12 13                                      ....\n"
    /// );
    /// ```
    #[inline]
    pub fn hexdump_bytes_offset(&self, src: impl AsRef<[u8]>, offset: u64) -> String {
        let line_size = self.get_size_line();
        let line_count =
            (src.as_ref().len() as f64 / self.config.bytes_per_line as f64).ceil() as usize;
        let mut dst = Vec::with_capacity(line_count * line_size);
        let mut cur = Cursor::new(src);
        let iter = RhexdumpIter::new(*self, &mut dst, &mut cur).offset(offset);
        iter.for_each(|_| {});
        // UNSAFE: every single byte is a result of the hexdump formatting. We are therefore sure
        //         that it is valid UTF-8 and we can proceed to convert the vec to string without
        //         any check.
        unsafe { String::from_utf8_unchecked(dst) }
    }

    /// Hexdumps a slice of bytes to a [`String`].
    ///
    /// # Example
    ///
    /// ```
    /// use rhexdump::prelude::*;
    ///
    /// let v = (0..0x14).collect::<Vec<u8>>();
    /// let rh = RhexdumpString::new();
    /// let out = rh.hexdump_bytes(&v);
    /// assert_eq!(
    ///     &out,
    ///     "00000000: 00 01 02 03 04 05 06 07 08 09 0a 0b 0c 0d 0e 0f  ................\n\
    ///      00000010: 10 11 12 13                                      ....\n"
    /// );
    /// ```
    #[inline]
    pub fn hexdump_bytes(&self, src: impl AsRef<[u8]>) -> String {
        self.hexdump_bytes_offset(src, 0)
    }

    /// Creates an iterator over a data source implementing [`std::io::Read`] and returns
    /// [`String`]s.
    ///
    /// # Example
    ///
    /// ```
    /// use rhexdump::prelude::*;
    ///
    /// let v = (0..0x14).collect::<Vec<u8>>();
    /// let rh = RhexdumpString::new();
    /// let mut cur = std::io::Cursor::new(&v);
    /// let mut iter = rh.iter(&mut cur);
    /// let out = iter.next().unwrap();
    /// assert_eq!(
    ///     &out,
    ///     "00000000: 00 01 02 03 04 05 06 07 08 09 0a 0b 0c 0d 0e 0f  ................"
    /// );
    /// ```
    pub fn iter<'r, R: Read>(&self, src: &'r mut R) -> RhexdumpStringIter<'r, R, Self> {
        RhexdumpStringIter::new(*self, src)
    }
}

unsafe impl Send for RhexdumpString {}
unsafe impl Sync for RhexdumpString {}

impl fmt::Display for RhexdumpString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "RhexdumpString {{ \
                base: {}, \
                endianness: {}, \
                bit_width: {}, \
                group_size: {}, \
                groups_per_line: {}, \
                hide_duplicate_lines: {} \
            }}",
            self.config.base,
            self.config.endianness,
            self.config.bit_width,
            self.config.group_size,
            self.config.groups_per_line,
            self.config.hide_duplicate_lines,
        )
    }
}

impl From<RhexdumpConfig> for RhexdumpString {
    fn from(config: RhexdumpConfig) -> Self {
        Self::with_config(config)
    }
}

impl RhexdumpGetConfig for RhexdumpString {
    /// Returns the config associated with a rhexdump instant.
    #[inline]
    fn get_config(&self) -> RhexdumpConfig {
        self.config
    }
}

// ===============================================================================================
// Stdout Rhexdump
// ===============================================================================================

/// Formats byte slices and data from a source implementing [`std::io::Read`] to
/// [`std::io::Stdout`].
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct RhexdumpStdout {
    /// Configuration object
    config: RhexdumpConfig,
}

impl RhexdumpStdout {
    /// Creates a new instance of `RhexdumpStdout` with the following default parameters:
    ///
    /// - **Base**: hexadecimal
    /// - **Endianness**: little endian
    /// - **Offset bit width**: 32 bits
    /// - **Group size**: byte (8-bit)
    /// - **Groups per line**: 16
    /// - **Bytes per line**: 16
    /// - **Hide duplicate lines**: no
    ///
    /// # Example:
    ///
    /// ```
    /// use rhexdump::prelude::*;
    ///
    /// let rhx = RhexdumpStdout::new();
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a new instance of `RhexdumpStdout` using the configuration passed as argument.
    ///
    /// # Example
    ///
    /// ```
    /// use rhexdump::prelude::*;
    ///
    /// let config = RhexdumpBuilder::new().config();
    /// let rhx = RhexdumpStdout::with_config(config);
    /// ```
    pub fn with_config(config: RhexdumpConfig) -> Self {
        Self { config }
    }

    /// Hexdumps, with an offset, data from a source implementing [`std::io::Read`] to
    /// [`std::io::Stdout`].
    ///
    /// # Example
    ///
    /// ```
    /// use rhexdump::prelude::*;
    /// use std::io::Cursor;
    ///
    /// // Create a Rhexdump instance.
    /// let rhx = RhexdumpStdout::new();
    ///
    /// // String that will be formatted.
    /// let input = String::from("Lorem ipsum dolor sit amet, consectetur adipiscing elit");
    /// let mut cur = Cursor::new(&input);
    ///
    /// // Formatting data to the temp file.
    /// rhx.hexdump_offset(&mut cur, 0x12340000);
    /// ```
    #[inline]
    pub fn hexdump_offset<R: Read>(&self, src: &mut R, offset: u64) {
        let mut stdout = io::stdout();
        let iter = RhexdumpIter::new(*self, &mut stdout, src).offset(offset);
        iter.for_each(|_| {});
    }

    /// Hexdumps data from a source implementing [`std::io::Read`] to [`std::io::Stdout`].
    ///
    /// # Example
    ///
    /// ```
    /// use rhexdump::prelude::*;
    /// use std::io::Cursor;
    ///
    /// // Create a Rhexdump instance.
    /// let rhx = RhexdumpStdout::new();
    ///
    /// // String that will be formatted.
    /// let input = String::from("Lorem ipsum dolor sit amet, consectetur adipiscing elit");
    /// let mut cur = Cursor::new(&input);
    ///
    /// // Formatting data to the temp file.
    /// rhx.hexdump(&mut cur);
    /// ```
    #[inline]
    pub fn hexdump<R: Read>(&self, src: &mut R) {
        self.hexdump_offset(src, 0)
    }

    /// Hexdumps, with an offset, a slice of bytes to [`std::io::Stdout`].
    ///
    /// # Example
    ///
    /// ```
    /// use rhexdump::prelude::*;
    ///
    /// // Create a Rhexdump instance.
    /// let rhx = RhexdumpStdout::new();
    ///
    /// // String that will be formatted.
    /// let input = String::from("Lorem ipsum dolor sit amet, consectetur adipiscing elit");
    ///
    /// // Formatting data to the temp file.
    /// rhx.hexdump_bytes_offset(input.as_bytes(), 0x12340000);
    /// ```
    #[inline]
    pub fn hexdump_bytes_offset(&self, src: impl AsRef<[u8]>, offset: u64) {
        let mut cur = Cursor::new(&src);
        let mut stdout = io::stdout();
        let iter = RhexdumpIter::new(*self, &mut stdout, &mut cur).offset(offset);
        iter.for_each(|_| {});
    }

    /// Hexdumps a slice of bytes to [`std::io::Stdout`].
    ///
    /// # Example
    ///
    /// ```
    /// use rhexdump::prelude::*;
    ///
    /// // Create a Rhexdump instance.
    /// let rhx = RhexdumpStdout::new();
    ///
    /// // String that will be formatted.
    /// let input = String::from("Lorem ipsum dolor sit amet, consectetur adipiscing elit");
    ///
    /// // Formatting data to the temp file.
    /// rhx.hexdump_bytes(input.as_bytes());
    /// ```
    #[inline]
    pub fn hexdump_bytes(&self, src: impl AsRef<[u8]>) {
        self.hexdump_bytes_offset(src, 0)
    }

    /// Creates an iterator over a data source implementing [`std::io::Read`] and writes it to
    /// [`std::io::Stdout`].
    ///
    /// # Example
    ///
    /// ```
    /// use rhexdump::prelude::*;
    ///
    /// let v = (0..0x14).collect::<Vec<u8>>();
    /// let rh = RhexdumpStdout::new();
    /// let mut cur = std::io::Cursor::new(&v);
    /// let mut iter = rh.iter(&mut cur).offset(0x12340000);
    /// iter.next();
    /// iter.next();
    /// ```
    pub fn iter<'r, R: Read>(&self, src: &'r mut R) -> RhexdumpStdoutIter<'r, R, Self> {
        RhexdumpStdoutIter::new(*self, src)
    }
}

unsafe impl Send for RhexdumpStdout {}
unsafe impl Sync for RhexdumpStdout {}

impl fmt::Display for RhexdumpStdout {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "RhexdumpStdout {{ \
                base: {}, \
                endianness: {}, \
                bit_width: {}, \
                group_size: {}, \
                groups_per_line: {}, \
                hide_duplicate_lines: {} \
            }}",
            self.config.base,
            self.config.endianness,
            self.config.bit_width,
            self.config.group_size,
            self.config.groups_per_line,
            self.config.hide_duplicate_lines,
        )
    }
}

impl From<RhexdumpConfig> for RhexdumpStdout {
    fn from(config: RhexdumpConfig) -> Self {
        Self::with_config(config)
    }
}

impl RhexdumpGetConfig for RhexdumpStdout {
    #[inline]
    fn get_config(&self) -> RhexdumpConfig {
        self.config
    }
}

// ===============================================================================================
// Tests
// ===============================================================================================

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use std::env::temp_dir;
    use std::fs::OpenOptions;
    use std::io::prelude::*;
    use std::io::*;

    // -------------------------------------------------------------------------------------------
    // Rhexdump

    #[test]
    fn rhx_rhexdump_hexdump_offset() {
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

    #[test]
    fn rhx_rhexdump_hexdump() {
        // Create a Rhexdump instance.
        let rhx = Rhexdump::new();

        // Create a temporary output file.
        let filename = "rhx_rhexdump_hexdump.test";
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
        rhx.hexdump(&mut f, &mut cur);

        // Reading the temp file content and making sure it corresponds to the expected output.
        f.seek(SeekFrom::Start(0))
            .expect(&format!("Could not seek to start of {}", filename));
        let mut output = Vec::new();
        f.read_to_end(&mut output)
            .expect(&format!("Cannot read from {}", filename));
        assert_eq!(
            &String::from_utf8_lossy(&output),
            "00000000: 4c 6f 72 65 6d 20 69 70 73 75 6d 20 64 6f 6c 6f  Lorem.ipsum.dolo\n\
             00000010: 72 20 73 69 74 20 61 6d 65 74 2c 20 63 6f 6e 73  r.sit.amet,.cons\n\
             00000020: 65 63 74 65 74 75 72 20 61 64 69 70 69 73 63 69  ectetur.adipisci\n\
             00000030: 6e 67 20 65 6c 69 74                             ng.elit\n"
        );
    }

    #[test]
    fn rhx_rhexdump_iter_offset() {
        // Create a Rhexdump instance.
        let rhx = Rhexdump::new();

        // Create a temporary output file.
        let filename = "rhx_rhexdump_iter_offset.test";
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
        let mut iter = rhx.iter(&mut f, &mut cur).offset(0x12340000);
        iter.next();
        iter.next();

        // Reading the temp file content and making sure it corresponds to the expected output.
        f.seek(SeekFrom::Start(0))
            .expect(&format!("Could not seek to start of {}", filename));
        let mut output = Vec::new();
        f.read_to_end(&mut output)
            .expect(&format!("Cannot read from {}", filename));
        assert_eq!(
            &String::from_utf8_lossy(&output),
            "12340000: 4c 6f 72 65 6d 20 69 70 73 75 6d 20 64 6f 6c 6f  Lorem.ipsum.dolo\n\
             12340010: 72 20 73 69 74 20 61 6d 65 74 2c 20 63 6f 6e 73  r.sit.amet,.cons\n"
        );
    }

    #[test]
    fn rhx_rhexdump_iter() {
        // Create a Rhexdump instance.
        let rhx = Rhexdump::new();

        // Create a temporary output file.
        let filename = "rhx_rhexdump_iter.test";
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
        let mut iter = rhx.iter(&mut f, &mut cur);
        iter.next();
        iter.next();

        // Reading the temp file content and making sure it corresponds to the expected output.
        f.seek(SeekFrom::Start(0))
            .expect(&format!("Could not seek to start of {}", filename));
        let mut output = Vec::new();
        f.read_to_end(&mut output)
            .expect(&format!("Cannot read from {}", filename));
        assert_eq!(
            &String::from_utf8_lossy(&output),
            "00000000: 4c 6f 72 65 6d 20 69 70 73 75 6d 20 64 6f 6c 6f  Lorem.ipsum.dolo\n\
             00000010: 72 20 73 69 74 20 61 6d 65 74 2c 20 63 6f 6e 73  r.sit.amet,.cons\n"
        );
    }

    // -------------------------------------------------------------------------------------------
    // RhexdumpString

    #[test]
    fn rhx_rhexdump_string_hexdump_offset() {
        let v = (0..0x14).collect::<Vec<u8>>();
        let rh = RhexdumpString::new();
        let mut cur = Cursor::new(&v);
        let out = rh.hexdump_offset(&mut cur, 0x12340000);
        assert_eq!(
            &out,
            "12340000: 00 01 02 03 04 05 06 07 08 09 0a 0b 0c 0d 0e 0f  ................\n\
             12340010: 10 11 12 13                                      ....\n"
        );
    }

    #[test]
    fn rhx_rhexdump_string_hexdump() {
        let v = (0..0x14).collect::<Vec<u8>>();
        let rh = RhexdumpString::new();
        let mut cur = Cursor::new(&v);
        let out = rh.hexdump(&mut cur);
        assert_eq!(
            &out,
            "00000000: 00 01 02 03 04 05 06 07 08 09 0a 0b 0c 0d 0e 0f  ................\n\
             00000010: 10 11 12 13                                      ....\n"
        );
    }

    #[test]
    fn rhx_rhexdump_string_hexdump_bytes_offset() {
        let v = (0..0x14).collect::<Vec<u8>>();
        let rh = RhexdumpString::new();
        let out = rh.hexdump_bytes_offset(&v, 0x12340000);
        assert_eq!(
            &out,
            "12340000: 00 01 02 03 04 05 06 07 08 09 0a 0b 0c 0d 0e 0f  ................\n\
             12340010: 10 11 12 13                                      ....\n"
        );
    }

    #[test]
    fn rhx_rhexdump_string_hexdump_bytes() {
        let v = (0..0x14).collect::<Vec<u8>>();
        let rh = RhexdumpString::new();
        let out = rh.hexdump_bytes(&v);
        assert_eq!(
            &out,
            "00000000: 00 01 02 03 04 05 06 07 08 09 0a 0b 0c 0d 0e 0f  ................\n\
             00000010: 10 11 12 13                                      ....\n"
        );
    }

    #[test]
    fn rhx_rhexdump_string_iter_offset() {
        let v = (0..0x14).collect::<Vec<u8>>();
        let rh = RhexdumpString::new();
        let mut cur = Cursor::new(&v);
        let mut iter = rh.iter(&mut cur).offset(0x12340000);
        let out = iter.next().unwrap();
        assert_eq!(
            &out,
            "12340000: 00 01 02 03 04 05 06 07 08 09 0a 0b 0c 0d 0e 0f  ................"
        );
    }

    #[test]
    fn rhx_rhexdump_string_iter() {
        let v = (0..0x14).collect::<Vec<u8>>();
        let rh = RhexdumpString::new();
        let mut cur = Cursor::new(&v);
        let mut iter = rh.iter(&mut cur);
        let out = iter.next().unwrap();
        assert_eq!(
            &out,
            "00000000: 00 01 02 03 04 05 06 07 08 09 0a 0b 0c 0d 0e 0f  ................"
        );
    }

    // -------------------------------------------------------------------------------------------
    // RhexdumpStdout

    #[test]
    fn rhx_rhexdump_stdout_hexdump_offset() {
        // Create a Rhexdump instance.
        let rhx = RhexdumpStdout::new();

        // String that will be formatted.
        let input = String::from("Lorem ipsum dolor sit amet, consectetur adipiscing elit");
        let mut cur = Cursor::new(&input);

        // Formatting data to the temp file.
        rhx.hexdump_offset(&mut cur, 0x12340000);
    }

    #[test]
    fn rhx_rhexdump_stdout_hexdump() {
        // Create a Rhexdump instance.
        let rhx = RhexdumpStdout::new();

        // String that will be formatted.
        let input = String::from("Lorem ipsum dolor sit amet, consectetur adipiscing elit");
        let mut cur = Cursor::new(&input);

        // Formatting data to the temp file.
        rhx.hexdump(&mut cur);
    }

    #[test]
    fn rhx_rhexdump_stdout_hexdump_bytes_offset() {
        // Create a Rhexdump instance.
        let rhx = RhexdumpStdout::new();

        // String that will be formatted.
        let input = String::from("Lorem ipsum dolor sit amet, consectetur adipiscing elit");

        // Formatting data to the temp file.
        rhx.hexdump_bytes_offset(input.as_bytes(), 0x12340000);
    }

    #[test]
    fn rhx_rhexdump_stdout_hexdump_bytes() {
        // Create a Rhexdump instance.
        let rhx = RhexdumpStdout::new();

        // String that will be formatted.
        let input = String::from("Lorem ipsum dolor sit amet, consectetur adipiscing elit");

        // Formatting data to the temp file.
        rhx.hexdump_bytes(input.as_bytes());
    }

    #[test]
    fn rhx_rhexdump_stdout_iter_offset() {
        let v = (0..0x14).collect::<Vec<u8>>();
        let rh = RhexdumpStdout::new();
        let mut cur = Cursor::new(&v);
        let mut iter = rh.iter(&mut cur).offset(0x12340000);
        iter.next();
        iter.next();
    }

    #[test]
    fn rhx_rhexdump_stdout_iter() {
        let v = (0..0x14).collect::<Vec<u8>>();
        let rh = RhexdumpStdout::new();
        let mut cur = Cursor::new(&v);
        let mut iter = rh.iter(&mut cur);
        iter.next();
        iter.next();
    }
}
