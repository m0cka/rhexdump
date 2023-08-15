//! Rhexdump instance builder object and methods.

use std::fmt;

use crate::config::*;
use crate::hexdump::*;

// ===============================================================================================
// Settings
// ===============================================================================================

/// Supported numeral bases.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub enum Base {
    /// Formats the output in base 2.
    Bin = 2,
    /// Formats the output in base 8.
    Oct = 8,
    /// Formats the output in base 10.
    Dec = 10,
    /// Formats the output in base 16.
    #[default]
    Hex = 16,
}

unsafe impl Send for Base {}
unsafe impl Sync for Base {}

impl fmt::Display for Base {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Base::Bin => write!(f, "Binary"),
            Base::Oct => write!(f, "Octal"),
            Base::Dec => write!(f, "Decimal"),
            Base::Hex => write!(f, "Hexadecimal"),
        }
    }
}

// -----------------------------------------------------------------------------------------------

/// Supported endianness modes.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub enum Endianness {
    /// Big endian.
    BigEndian,
    /// Little endian.
    #[default]
    LittleEndian,
}

unsafe impl Send for Endianness {}
unsafe impl Sync for Endianness {}

impl fmt::Display for Endianness {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Endianness::LittleEndian => write!(f, "LittleEndian"),
            Endianness::BigEndian => write!(f, "BigEndian"),
        }
    }
}

// -----------------------------------------------------------------------------------------------

/// Supported offset bit widths.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub enum BitWidth {
    /// 64-bit mode.
    BW64 = 16,
    /// 32-bit mode.
    #[default]
    BW32 = 8,
}

unsafe impl Send for BitWidth {}
unsafe impl Sync for BitWidth {}

impl fmt::Display for BitWidth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BitWidth::BW64 => write!(f, "64-bits"),
            BitWidth::BW32 => write!(f, "32-bits"),
        }
    }
}

// -----------------------------------------------------------------------------------------------

/// Supported byte group sizes.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub enum GroupSize {
    /// Data grouped as 8-bit values.
    #[default]
    Byte = 1,
    /// Data grouped as 16-bit values.
    Word = 2,
    /// Data grouped as 32-bit values.
    Dword = 4,
    /// Data grouped as 64-bit values.
    Qword = 8,
}

impl GroupSize {
    #[inline]
    pub fn get_size(&self, base: Base) -> usize {
        match self {
            GroupSize::Byte => (u8::MAX as f64).log(base as u8 as f64).ceil() as usize,
            GroupSize::Word => (u16::MAX as f64).log(base as u8 as f64).ceil() as usize,
            GroupSize::Dword => (u32::MAX as f64).log(base as u8 as f64).ceil() as usize,
            GroupSize::Qword => (u64::MAX as f64).log(base as u8 as f64).ceil() as usize,
        }
    }
}

unsafe impl Send for GroupSize {}
unsafe impl Sync for GroupSize {}

impl fmt::Display for GroupSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GroupSize::Byte => write!(f, "Byte (8-bit)"),
            GroupSize::Word => write!(f, "Word (16-bit)"),
            GroupSize::Dword => write!(f, "Dword (32-bit)"),
            GroupSize::Qword => write!(f, "Qword (64-bit)"),
        }
    }
}

/// Maximum number of bytes per group.
pub const MAX_BYTES_PER_GROUP: usize = GroupSize::Qword as usize;

// ===============================================================================================
// Builder
// ===============================================================================================

/// Builder for a Rhexdump instance.
///
///
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct RhexdumpBuilder(RhexdumpConfig);

unsafe impl Send for RhexdumpBuilder {}
unsafe impl Sync for RhexdumpBuilder {}

impl RhexdumpBuilder {
    /// Creates a new instance of the builder.
    ///
    /// # Showcase
    ///
    /// ```
    /// use rhexdump::prelude::*;
    ///
    /// // Creates a new `Rhexdump` builder.
    /// let builder = RhexdumpBuilder::new();
    /// ```
    ///
    /// # Example
    ///
    /// ```
    /// use rhexdump::prelude::*;
    ///
    /// // New instance of the builder.
    /// let builder = RhexdumpBuilder::new();
    ///
    /// // It can then be configured and built into an actual `Rhexdump` instance.
    /// let rh = builder
    ///     .base(Base::Hex)
    ///     .endianness(Endianness::LittleEndian)
    ///     .bit_width(BitWidth::BW64)
    ///     .group_size(GroupSize::Byte)
    ///     .groups_per_line(16)
    ///     .build();
    /// ```
    #[inline]
    pub fn new() -> Self {
        RhexdumpBuilder::default()
    }

    /// Consumes the builder and returns the current [`RhexdumpConfig`].
    ///
    /// # Showcase
    ///
    /// ```
    /// use rhexdump::prelude::*;
    ///
    /// // Instanciating a `Rhexdump` object using the `build` function.
    /// let config = RhexdumpBuilder::new().config();
    /// ```
    #[inline]
    pub fn config(mut self) -> RhexdumpConfig {
        self.0.bytes_per_line = self.0.group_size as usize * self.0.groups_per_line;
        self.0
    }

    /// Builds the current builder into a [`Rhexdump`] instance.
    ///
    /// # Showcase
    ///
    /// ```
    /// use rhexdump::prelude::*;
    ///
    /// // Instanciating a `Rhexdump` object using the `build` function.
    /// let rh = RhexdumpBuilder::new().build();
    /// ```
    ///
    /// # Example
    ///
    /// ```
    /// use rhexdump::prelude::*;
    ///
    /// let v = (0..0x10).collect::<Vec<u8>>();
    /// let rh = RhexdumpBuilder::new().build_string();
    /// let out = rh.hexdump_bytes(&v);
    /// assert_eq!(
    ///     &out,
    ///     "00000000: 00 01 02 03 04 05 06 07 08 09 0a 0b 0c 0d 0e 0f  ................\n"
    /// );
    /// ```
    #[inline]
    pub fn build(self) -> Rhexdump {
        Rhexdump::with_config(self.config())
    }

    /// Builds the current builder into a [`RhexdumpString`] instance.
    ///
    /// # Showcase
    ///
    /// ```
    /// use rhexdump::prelude::*;
    ///
    /// // Instanciating a `Rhexdump` object using the `build` function.
    /// let rh = RhexdumpBuilder::new().build_string();
    /// ```
    ///
    /// # Example
    ///
    /// ```
    /// use rhexdump::prelude::*;
    ///
    /// let v = (0..0x10).collect::<Vec<u8>>();
    /// let rh = RhexdumpBuilder::new().build_string();
    /// let out = rh.hexdump_bytes(&v);
    /// assert_eq!(
    ///     &out,
    ///     "00000000: 00 01 02 03 04 05 06 07 08 09 0a 0b 0c 0d 0e 0f  ................\n"
    /// );
    /// ```
    #[inline]
    pub fn build_string(self) -> RhexdumpString {
        RhexdumpString::with_config(self.config())
    }

    /// Builds the current builder into a [`RhexdumpStdout`] instance.
    ///
    /// # Showcase
    ///
    /// ```
    /// use rhexdump::prelude::*;
    ///
    /// // Instanciating a `Rhexdump` object using the `build` function.
    /// let rh = RhexdumpBuilder::new().build_stdout();
    /// ```
    ///
    /// # Example
    ///
    /// ```
    /// use rhexdump::prelude::*;
    ///
    /// let v = (0..0x10).collect::<Vec<u8>>();
    /// let rh = RhexdumpBuilder::new().build_string();
    /// rh.hexdump_bytes(&v); // Outputs formatted data to stdout.
    /// ```
    #[inline]
    pub fn build_stdout(self) -> RhexdumpStdout {
        RhexdumpStdout::with_config(self.config())
    }

    /// Sets the numeral base [`Base`] of the builder.
    ///
    /// # Showcase
    ///
    /// ```
    /// use rhexdump::prelude::*;
    ///
    /// // Sets the base to octal.
    /// let builder = RhexdumpBuilder::new().base(Base::Oct);
    /// ```
    ///
    /// # Example
    ///
    /// ```
    /// use rhexdump::prelude::*;
    ///
    /// let v = (0..0x10).collect::<Vec<u8>>();
    /// let rh = RhexdumpBuilder::new().base(Base::Oct).build_string();
    /// let out = rh.hexdump_bytes(&v);
    /// assert_eq!(
    ///     &out,
    ///     "00000000: 000 001 002 003 004 005 006 007 010 011 012 013 014 015 016 017  ................\n"
    /// );
    /// ```
    #[inline]
    pub fn base(mut self, base: Base) -> Self {
        self.0.base = base;
        self
    }

    /// Sets the endianness [`Endianness`] of the builder.
    ///
    /// # Showcase
    ///
    /// ```
    /// use rhexdump::prelude::*;
    ///
    /// // Sets the base to big endian.
    /// let builder = RhexdumpBuilder::new().endianness(Endianness::BigEndian);
    /// ```
    ///
    /// # Example
    ///
    /// ```
    /// use rhexdump::prelude::*;
    ///
    /// let v = (0..0x10).collect::<Vec<u8>>();
    /// let rh = RhexdumpBuilder::new()
    ///     .group_size(GroupSize::Dword)
    ///     .groups_per_line(4)
    ///     .endianness(Endianness::BigEndian)
    ///     .build_string();
    /// let out = rh.hexdump_bytes(&v);
    /// assert_eq!(
    ///     &out,
    ///     "00000000: 00010203 04050607 08090a0b 0c0d0e0f  ................\n"
    /// );
    /// ```
    #[inline]
    pub fn endianness(mut self, endianness: Endianness) -> Self {
        self.0.endianness = endianness;
        self
    }

    /// Sets the offset bit width [`BitWidth`] of the builder.
    ///
    /// # Showcase
    ///
    /// ```
    /// use rhexdump::prelude::*;
    ///
    /// // Sets the offset bit width to 64 bits.
    /// let builder = RhexdumpBuilder::new().bit_width(BitWidth::BW64);
    /// ```
    ///
    /// # Example
    ///
    /// ```
    /// use rhexdump::prelude::*;
    ///
    /// let v = (0..0x10).collect::<Vec<u8>>();
    /// let rh = RhexdumpBuilder::new()
    ///     .bit_width(BitWidth::BW64)
    ///     .build_string();
    /// let out = rh.hexdump_bytes(&v);
    /// assert_eq!(
    ///     &out,
    ///     "0000000000000000: 00 01 02 03 04 05 06 07 08 09 0a 0b 0c 0d 0e 0f  ................\n"
    /// );
    /// ```
    #[inline]
    pub fn bit_width(mut self, bit_width: BitWidth) -> Self {
        self.0.bit_width = bit_width;
        self
    }

    /// Sets the byte group size [`GroupSize`] of the builder.
    ///
    /// # Shocase
    ///
    /// ```
    /// use rhexdump::prelude::*;
    ///
    /// // Sets the group size to `Dword` (32-bit).
    /// let builder = RhexdumpBuilder::new().group_size(GroupSize::Dword);
    /// ```
    ///
    /// # Example
    ///
    /// ```
    /// use rhexdump::prelude::*;
    ///
    /// let v = (0..0x10).collect::<Vec<u8>>();
    /// let rh = RhexdumpBuilder::new()
    ///     .group_size(GroupSize::Dword)
    ///     .groups_per_line(4)
    ///     .build_string();
    /// let out = rh.hexdump_bytes(&v);
    /// assert_eq!(
    ///     &out,
    ///     "00000000: 03020100 07060504 0b0a0908 0f0e0d0c  ................\n"
    /// );
    /// ```
    #[inline]
    pub fn group_size(mut self, group_size: GroupSize) -> Self {
        self.0.group_size = group_size;
        self
    }

    /// Sets the number of groups per line of the builder.
    ///
    /// # Shocase
    ///
    /// ```
    /// use rhexdump::prelude::*;
    ///
    /// // Sets the number of groups per line to four.
    /// let builder = RhexdumpBuilder::new().groups_per_line(4);
    /// ```
    ///
    /// # Example
    ///
    /// ```
    /// use rhexdump::prelude::*;
    ///
    /// let v = (0..0x10).collect::<Vec<u8>>();
    /// let rh = RhexdumpBuilder::new()
    ///     .groups_per_line(4)
    ///     .build_string();
    /// let out = rh.hexdump_bytes(&v);
    /// assert_eq!(
    ///     &out,
    ///     "00000000: 00 01 02 03  ....\n\
    ///     00000004: 04 05 06 07  ....\n\
    ///     00000008: 08 09 0a 0b  ....\n\
    ///     0000000c: 0c 0d 0e 0f  ....\n"
    /// );
    /// ```
    #[inline]
    pub fn groups_per_line(mut self, groups_per_line: usize) -> Self {
        self.0.groups_per_line = if groups_per_line == 0 {
            1
        } else {
            groups_per_line
        };
        self
    }

    /// Sets whether or not duplicate lines should be shown.
    ///
    /// # Shocase
    ///
    /// ```
    /// use rhexdump::prelude::*;
    ///
    /// // Hides duplicate lines.
    /// let builder = RhexdumpBuilder::new().hide_duplicate_lines(true);
    /// ```
    ///
    /// # Example
    ///
    /// ```
    /// use rhexdump::prelude::*;
    ///
    /// let v = vec![0u8; 0x10];
    /// let rh = RhexdumpBuilder::new()
    ///     .hide_duplicate_lines(true)
    ///     .groups_per_line(4)
    ///     .build_string();
    /// let out = rh.hexdump_bytes(&v);
    /// assert_eq!(
    ///     &out,
    ///     "00000000: 00 00 00 00  ....\n\
    ///     *\n\
    ///     0000000c: 00 00 00 00  ....\n"
    /// );
    /// ```
    #[inline]
    pub fn hide_duplicate_lines(mut self, hide_duplicate_lines: bool) -> Self {
        self.0.hide_duplicate_lines = hide_duplicate_lines;
        self
    }
}

impl fmt::Display for RhexdumpBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RhexdumpBuilder {{ {} }}", self.0)
    }
}

// TODO from Rhexdump

// ===============================================================================================
// Tests
// ===============================================================================================

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn rhx_builder_build() {
        let v = (0..0x10).collect::<Vec<u8>>();
        let rh = RhexdumpBuilder::new().build_string();
        let out = rh.hexdump_bytes(&v);
        assert_eq!(
            &out,
            "00000000: 00 01 02 03 04 05 06 07 08 09 0a 0b 0c 0d 0e 0f  ................\n"
        );
    }

    #[test]
    fn rhx_builder_base() {
        let v = (0..0x10).collect::<Vec<u8>>();
        let rh = RhexdumpBuilder::new().base(Base::Oct).build_string();
        let out = rh.hexdump_bytes(&v);
        assert_eq!(
            &out,
            "00000000: 000 001 002 003 004 005 006 007 010 011 012 013 014 015 016 017  ................\n"
        );
    }

    #[test]
    fn rhx_builder_endianness() {
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

    #[test]
    fn rhx_builder_bit_width() {
        let v = (0..0x10).collect::<Vec<u8>>();
        let rh = RhexdumpBuilder::new()
            .bit_width(BitWidth::BW64)
            .build_string();
        let out = rh.hexdump_bytes(&v);
        assert_eq!(
            &out,
            "0000000000000000: 00 01 02 03 04 05 06 07 08 09 0a 0b 0c 0d 0e 0f  ................\n"
        );
    }

    #[test]
    fn rhx_builder_group_size() {
        let v = (0..0x10).collect::<Vec<u8>>();
        let rh = RhexdumpBuilder::new()
            .group_size(GroupSize::Dword)
            .groups_per_line(4)
            .build_string();
        let out = rh.hexdump_bytes(&v);
        assert_eq!(
            &out,
            "00000000: 03020100 07060504 0b0a0908 0f0e0d0c  ................\n"
        );
    }

    #[test]
    fn rhx_builder_groups_per_line() {
        let v = (0..0x10).collect::<Vec<u8>>();
        let rh = RhexdumpBuilder::new().groups_per_line(4).build_string();
        let out = rh.hexdump_bytes(&v);
        assert_eq!(
            &out,
            "00000000: 00 01 02 03  ....\n\
            00000004: 04 05 06 07  ....\n\
            00000008: 08 09 0a 0b  ....\n\
            0000000c: 0c 0d 0e 0f  ....\n"
        );
    }

    #[test]
    fn rhx_builder_hide_duplicate_lines() {
        let v = vec![0u8; 0x10];
        let rh = RhexdumpBuilder::new()
            .hide_duplicate_lines(true)
            .groups_per_line(4)
            .build_string();
        let out = rh.hexdump_bytes(&v);
        assert_eq!(
            &out,
            "00000000: 00 00 00 00  ....\n\
            *\n\
            0000000c: 00 00 00 00  ....\n"
        );
    }
}
