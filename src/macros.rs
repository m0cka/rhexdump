// ===============================================================================================
// Macros
// ===============================================================================================

/// Hexdump to [`std::io::Stdout`].
///
/// # Example
///
/// ```
/// use rhexdump::prelude::*;
///
/// // Data to format.
/// let v = (0..0x14).collect::<Vec<u8>>();
/// // Printing the formatted data to stdout.
/// rhexdump!(&v);
/// // Printing the formatted data to stdout with an offset.
/// rhexdump!(&v, 0x12340000);
/// ```
#[macro_export]
macro_rules! rhexdump {
    ($data:expr) => {{
        $crate::INSTANCE.with(|i|
            $crate::hexdump::RhexdumpStdout::with_config(*i.borrow()).hexdump_bytes($data)
        )
    }};
    ($data:expr, $offset:expr) => {{
        $crate::INSTANCE.with(|i|
            $crate::hexdump::RhexdumpStdout::with_config(*i.borrow()).hexdump_bytes_offset($data, $offset)
        )
    }};
}

/// Hexdump to a [`String`].
///
/// # Example
///
/// ```
/// use rhexdump::prelude::*;
/// // Data to format.
/// let v = (0..0x14).collect::<Vec<u8>>();
/// // Formatting the vector's content and writing the output to a string.
/// let output = rhexdumps!(&v);
/// assert_eq!(
///     &output,
///     "00000000: 00 01 02 03 04 05 06 07 08 09 0a 0b 0c 0d 0e 0f  ................\n\
///      00000010: 10 11 12 13                                      ....\n"
/// );
/// // Formatting the vector's content and writing the output to a string with an offset.
/// let output = rhexdumps!(&v, 0x12340000);
/// assert_eq!(
///     &output,
///     "12340000: 00 01 02 03 04 05 06 07 08 09 0a 0b 0c 0d 0e 0f  ................\n\
///      12340010: 10 11 12 13                                      ....\n"
/// );
/// ```
#[macro_export]
macro_rules! rhexdumps {
    ($data:expr) => {{
        $crate::INSTANCE.with(|i|
            $crate::hexdump::RhexdumpString::with_config(*i.borrow()).hexdump_bytes($data)
        )
    }};
    ($data:expr, $offset:expr) => {{
        $crate::INSTANCE.with(|i|
            $crate::hexdump::RhexdumpString::with_config(*i.borrow()).hexdump_bytes_offset($data, $offset)
        )
    }};
}

/// Installs a new thread-local global configuration
/// [`RhexdumpConfig`](`crate::config::RhexdumpConfig`).
///
/// # Example
///
/// ```
/// use rhexdump::prelude::*;
///
/// // Data to format.
/// let v = (0..0x14).collect::<Vec<u8>>();
/// // Hexdump with the default global configuration.
/// let output = rhexdumps!(&v);
/// assert_eq!(
///     &output,
///     "00000000: 00 01 02 03 04 05 06 07 08 09 0a 0b 0c 0d 0e 0f  ................\n\
///      00000010: 10 11 12 13                                      ....\n"
/// );
/// // Creating a new rhexdump configuration.
/// let config = RhexdumpBuilder::new()
///     .base(Base::Oct)
///     .bit_width(BitWidth::BW64)
///     .group_size(GroupSize::Word)
///     .groups_per_line(4)
///     .config();
/// // Installing the configuration globally.
/// rhexdump::rhexdump_install!(config);
/// // Hexdump of the same data with the new configuration.
/// let output = rhexdumps!(&v);
/// assert_eq!(
///     &output,
///     "0000000000000000: 000400 001402 002404 003406  ........\n\
///      0000000000000008: 004410 005412 006414 007416  ........\n\
///      0000000000000010: 010420 011422                ....\n"
/// );
/// ```
#[macro_export]
macro_rules! rhexdump_install {
    ($config:expr) => {{
        $crate::INSTANCE.with(|i| {
            *i.borrow_mut() = $config;
        });
    }};
}

// ===============================================================================================
// Tests
// ===============================================================================================

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn rhx_macro_rhexdump() {
        // Data to format.
        let v = (0..0x14).collect::<Vec<u8>>();
        // Printing the formatted data to stdout.
        rhexdump!(&v);
        // Printing the formatted data to stdout with an offset.
        rhexdump!(&v, 0x12340000);
    }

    #[test]
    fn rhx_macro_rhexdumps() {
        // Data to format.
        let v = (0..0x14).collect::<Vec<u8>>();
        // Formatting the vector's content and writing the output to a string.
        let output = rhexdumps!(&v);
        assert_eq!(
            &output,
            "00000000: 00 01 02 03 04 05 06 07 08 09 0a 0b 0c 0d 0e 0f  ................\n\
             00000010: 10 11 12 13                                      ....\n"
        );
        // Formatting the vector's content and writing the output to a string with an offset.
        let output = rhexdumps!(&v, 0x12340000);
        assert_eq!(
            &output,
            "12340000: 00 01 02 03 04 05 06 07 08 09 0a 0b 0c 0d 0e 0f  ................\n\
             12340010: 10 11 12 13                                      ....\n"
        );
    }
    #[test]
    fn rhx_macro_install() {
        // Data to format.
        let v = (0..0x14).collect::<Vec<u8>>();
        // Hexdump with the default global configuration.
        let output = rhexdumps!(&v);
        assert_eq!(
            &output,
            "00000000: 00 01 02 03 04 05 06 07 08 09 0a 0b 0c 0d 0e 0f  ................\n\
             00000010: 10 11 12 13                                      ....\n"
        );
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
}