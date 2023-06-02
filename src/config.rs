//! Rhexdump configuration structure.

use std::fmt;

use crate::builder::*;

/// Main object used to configure the output format.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct RhexdumpConfig {
    /// Numeral base.
    pub(crate) base: Base,
    /// Endianness mode.
    pub(crate) endianness: Endianness,
    /// Offset bit width.
    pub(crate) bit_width: BitWidth,
    /// Formatted bytes can be grouped together. If the actual data is `de ad be ef`, grouping them
    /// by two with a little endian output format would result in `adde efbe`.
    /// `bytes_per_group` is the number of bytes in such a group.
    pub(crate) group_size: GroupSize,
    /// Number of groups per formatted line.
    pub(crate) groups_per_line: usize,
    /// Number of data bytes per formatted line (`group_size * groups_per_line`).
    pub(crate) bytes_per_line: usize,
    /// Specifies if we want to omit duplicate lines and replace them by a single '*'.
    pub(crate) hide_duplicate_lines: bool,
}

unsafe impl Send for RhexdumpConfig {}
unsafe impl Sync for RhexdumpConfig {}

impl Default for RhexdumpConfig {
    fn default() -> Self {
        Self {
            base: Base::default(),
            endianness: Endianness::default(),
            bit_width: BitWidth::default(),
            group_size: GroupSize::default(),
            groups_per_line: 16,
            bytes_per_line: 16,
            hide_duplicate_lines: false,
        }
    }
}

impl fmt::Display for RhexdumpConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "RhexdumpConfig {{ \
                base: {}, \
                endianness: {}, \
                bit_width: {}, \
                group_size: {}, \
                groups_per_line: {}, \
                hide_duplicate_lines: {} \
            }}",
            self.base,
            self.endianness,
            self.bit_width,
            self.group_size,
            self.groups_per_line,
            self.hide_duplicate_lines,
        )
    }
}

pub trait RhexdumpGetConfig {
    fn get_config(&self) -> RhexdumpConfig;

    /// Returns the total size of a formatted line.
    #[inline]
    fn get_size_line(&self) -> usize {
        let config = self.get_config();
        let ascii_hex_len = config.bit_width as usize
            + 1
            + (config.group_size.get_size(config.base) + 1) * config.groups_per_line;
        ascii_hex_len + 2 + config.bytes_per_line + 1
    }
}
