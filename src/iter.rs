//! Iterators over hexdump-formatted data.

use std::io::{Read, Write};

use crate::builder::*;
use crate::config::*;

// ===============================================================================================
// String Iterator
// ===============================================================================================

/// Iterator over a data source implementing [`std::io::Read`] and returning [`String`]s
/// containing the formatted lines.
#[derive(Debug)]
pub struct RhexdumpStringIter<'r, R: Read, X: RhexdumpGetConfig + Copy> {
    /// The original Rhexdump object.
    rhx: X,
    /// Input data source.
    src: &'r mut R,
    /// The base offset from which we want to start displaying data.
    base_offset: u64,
    /// The current offset into `data`. Gets incremented after each iterator's step.
    offset: usize,
    /// Chunk of bytes we want to format.
    data: Vec<u8>,
    /// The vector storing the formatted line.
    line: Vec<u8>,
    /// The vector storing the ascii representation.
    ascii: Vec<u8>,
    /// The raw bytes of the previous line that was returned by the iterator.
    /// Used to identify duplicate lines.
    prev_line: Option<Vec<u8>>,
    /// State value to know whether or not we've already displayed the duplicate line characters '*'
    duplicate_line_displayed: bool,
}

impl<'r, R: Read, X: RhexdumpGetConfig + Copy> RhexdumpStringIter<'r, R, X> {
    /// Creates a new instance of the iterator.
    ///
    /// # Example
    ///
    /// ```
    /// use rhexdump::prelude::*;
    ///
    /// // Create a Rhexdump instance.
    /// let rhx = Rhexdump::new();
    ///
    /// // String that will be formatted.
    /// let input = String::from("Lorem ipsum dolor sit amet, consectetur adipiscing elit");
    /// let mut cur = std::io::Cursor::new(&input);
    ///
    /// // Creating an iterator.
    /// let mut iter = RhexdumpStringIter::new(rhx, &mut cur);
    ///
    /// // Taking two lines of output.
    /// let _ = iter.next().unwrap();
    /// let output = iter.next().unwrap();
    ///
    /// assert_eq!(
    ///     &output,
    ///     "00000010: 72 20 73 69 74 20 61 6d 65 74 2c 20 63 6f 6e 73  r.sit.amet,.cons"
    /// );
    /// ```
    pub fn new(rhx: X, src: &'r mut R) -> Self {
        let config = rhx.get_config();
        Self {
            rhx,
            src,
            base_offset: 0,
            offset: 0,
            data: vec![0u8; config.bytes_per_line],
            ascii: vec![0u8; config.bytes_per_line],
            line: vec![0u8; rhx.get_size_line()],
            prev_line: None,
            duplicate_line_displayed: false,
        }
    }

    /// Formats one line of data.
    fn format_line(&mut self, end: usize) -> std::io::Result<()> {
        self.ascii.clear();
        self.line.clear();
        let config = self.rhx.get_config();
        let group_size = config.group_size.get_size(config.base);
        let offset = self.base_offset + self.offset as u64;
        let mut bytes = [0u8; MAX_BYTES_PER_GROUP];
        // Format and write the first offset.
        match config.bit_width {
            BitWidth::BW32 => write!(self.line, "{:08x}", offset as u32)?,
            BitWidth::BW64 => write!(self.line, "{:016x}", offset)?,
        };
        write!(self.line, ":")?;
        // Iterate over chunks of size `group_size`, format each group and concatenate them.
        // We also take advantage of this iterator to compute the associated ascii output.
        for b in self.data[..end].chunks(config.group_size as usize) {
            // Reset the array of bytes.
            bytes.iter_mut().for_each(|x| *x = 0);
            // Format the current bytes and add them to the ascii string, as well as the bytes
            // array.
            for (i, &c) in b.iter().enumerate() {
                self.ascii.push(if c.is_ascii_graphic() { c } else { b'.' });
                bytes[i] = c;
            }
            // Convert one group of bytes.
            let value = match config.endianness {
                Endianness::LittleEndian => u64::from_le_bytes(bytes),
                Endianness::BigEndian => {
                    bytes.rotate_right(MAX_BYTES_PER_GROUP - b.len());
                    u64::from_be_bytes(bytes)
                }
            };
            write!(self.line, " ")?;
            // Format the byte group in the user-specified base.
            match config.base {
                Base::Bin => write!(self.line, "{:0p$b}", value, p = group_size)?,
                Base::Oct => write!(self.line, "{:0p$o}", value, p = group_size)?,
                Base::Dec => write!(self.line, "{:0p$}", value, p = group_size)?,
                Base::Hex => write!(self.line, "{:0p$x}", value, p = group_size)?,
            };
        }
        // Add the ascii representation at the end of the line.
        let padding = self.rhx.get_size_line() - self.line.len() - config.bytes_per_line - 1;
        write!(self.line, "{:>p$}", "", p = padding)?;
        // Write the resulting formatted line in the destination stream.
        write!(self.line, "{}", String::from_utf8_lossy(&self.ascii))?;
        Ok(())
    }

    /// Sets the hexdump offset.
    ///
    /// # Example
    ///
    /// ```
    /// use rhexdump::prelude::*;
    ///
    /// // Create a Rhexdump instance.
    /// let rhx = Rhexdump::new();
    ///
    /// // String that will be formatted.
    /// let input = String::from("Lorem ipsum dolor sit amet, consectetur adipiscing elit");
    /// let mut cur = std::io::Cursor::new(&input);
    ///
    /// // Creating an iterator with an offset.
    /// let mut iter = RhexdumpStringIter::new(rhx, &mut cur).offset(0x12340000);
    /// ```
    pub fn offset(mut self, offset: u64) -> Self {
        self.base_offset = offset;
        self
    }
}

impl<'r, R: Read, X: RhexdumpGetConfig + Copy> Iterator for RhexdumpStringIter<'r, R, X> {
    type Item = String;

    /// Returns one line of formatted bytes from the byte array according to the configuration of
    /// the associated Rhexdump object.
    fn next(&mut self) -> Option<Self::Item> {
        let config = self.rhx.get_config();
        let mut prev_offset = self.offset;
        let mut size_read;
        // Duplicate detection loop
        loop {
            // Resetting the data buffers.
            self.data.iter_mut().for_each(|x| *x = 0);
            // Reading data from the input file
            size_read = self.src.read(&mut self.data).ok()?;
            // If there is no more data to read...
            if size_read == 0 {
                // ... and we're currently displaying duplicate lines ...
                if self.duplicate_line_displayed {
                    // ... then retrieve the previous line ...
                    if let Some(ref prev_line) = self.prev_line {
                        // update the offset and data, before formatting and writing the line
                        // to the destination.
                        self.duplicate_line_displayed = false;
                        self.offset = prev_offset;
                        self.data.copy_from_slice(prev_line);
                        self.format_line(prev_line.len()).ok()?;
                        return Some(String::from_utf8_lossy(&self.line).to_string());
                    }
                }
                return None;
            }
            // If we don't want to display duplicate lines...
            if config.hide_duplicate_lines && self.prev_line.is_some() {
                let is_duplicate = self
                    .data
                    .iter()
                    .zip(self.prev_line.as_ref().unwrap().iter())
                    .all(|(&a, &b)| a == b);
                // ... and the current one is a duplicate of the previous one...
                if is_duplicate {
                    // ... then ignore the current line and restart the process with the next
                    // one if we have already displayed the '*' character...
                    if self.duplicate_line_displayed {
                        // Update the offsets
                        prev_offset = self.offset;
                        self.offset += size_read;
                        continue;
                    }
                    // ... otherwise, display '*' and store the fact that it was shown.
                    self.duplicate_line_displayed = true;
                    // Update the offsets
                    self.offset += size_read;
                    return Some("*".to_string());
                }
            }
            break;
        }
        // If we reached this point, we can update the current previous line if we don't want
        // to display duplicates.
        if config.hide_duplicate_lines {
            if let Some(ref mut prev_line) = self.prev_line {
                prev_line.iter_mut().for_each(|x| *x = 0);
                prev_line.copy_from_slice(&self.data);
            } else {
                self.prev_line = Some(self.data.clone());
            }
            self.duplicate_line_displayed = false;
        }
        // Format and write the output to the vec.
        self.format_line(size_read).ok()?;
        // Update the offsets
        self.offset += size_read;
        // UNSAFE: every single byte is a result of the hexdump formatting. We are therefore sure
        //         that it is valid UTF-8 and we can proceed to convert the vec to string without
        //         any check.
        Some(String::from_utf8_lossy(&self.line).to_string())
    }
}

// ===============================================================================================
// Generic iterator
// ===============================================================================================

/// Iterator over a data source implementing [`std::io::Read`] and writing to a destination
/// implementing [`std::io::Write`].
#[derive(Debug)]
pub struct RhexdumpIter<'r, 'w, R: Read, W: Write, X: RhexdumpGetConfig + Copy> {
    /// The original Rhexdump object.
    iter: RhexdumpStringIter<'r, R, X>,
    /// Output data destination.
    dst: &'w mut W,
}

impl<'r, 'w, R: Read, W: Write, X: RhexdumpGetConfig + Copy> RhexdumpIter<'r, 'w, R, W, X> {
    /// Creates a new instance of the iterator.
    ///
    /// # Example
    ///
    /// ```
    /// use rhexdump::prelude::*;
    ///
    /// // Create a Rhexdump instance.
    /// let rhx = Rhexdump::new();
    ///
    /// // String that will be formatted.
    /// let input = String::from("Lorem ipsum dolor sit amet, consectetur adipiscing elit");
    /// let mut cur_in = std::io::Cursor::new(&input);
    /// let mut output = Vec::new();
    /// let mut cur_out = std::io::Cursor::new(&mut output);
    ///
    /// // Creating an iterator.
    /// let mut iter = RhexdumpIter::new(rhx, &mut cur_out, &mut cur_in);
    ///
    /// // Taking two lines of output.
    /// let _ = iter.next().unwrap();
    /// let _ = iter.next().unwrap();
    ///
    /// assert_eq!(
    ///     &String::from_utf8_lossy(&output),
    ///     "00000000: 4c 6f 72 65 6d 20 69 70 73 75 6d 20 64 6f 6c 6f  Lorem.ipsum.dolo\n\
    ///      00000010: 72 20 73 69 74 20 61 6d 65 74 2c 20 63 6f 6e 73  r.sit.amet,.cons\n"
    /// );
    /// ```
    pub fn new(rhx: X, dst: &'w mut W, src: &'r mut R) -> Self {
        Self {
            iter: RhexdumpStringIter::new(rhx, src),
            dst,
        }
    }

    /// Sets the hexdump offset.
    ///
    /// # Example
    ///
    /// ```
    /// use rhexdump::prelude::*;
    ///
    /// // Create a Rhexdump instance.
    /// let rhx = Rhexdump::new();
    ///
    /// // String that will be formatted.
    /// let input = String::from("Lorem ipsum dolor sit amet, consectetur adipiscing elit");
    /// let mut cur_in = std::io::Cursor::new(&input);
    /// let mut output = Vec::new();
    /// let mut cur_out = std::io::Cursor::new(&mut output);
    ///
    /// // Creating an iterator with an offset.
    /// let mut iter = RhexdumpIter::new(rhx, &mut cur_out, &mut cur_in).offset(0x12340000);
    /// ```
    pub fn offset(mut self, offset: u64) -> Self {
        self.iter = self.iter.offset(offset);
        self
    }
}

impl<'r, 'w, R: Read, W: Write, X: RhexdumpGetConfig + Copy> Iterator
    for RhexdumpIter<'r, 'w, R, W, X>
{
    type Item = ();

    /// Returns one line of formatted bytes from the byte array according to the configuration of
    /// the associated Rhexdump object.
    fn next(&mut self) -> Option<Self::Item> {
        let output = self.iter.next()?;
        writeln!(self.dst, "{}", output).ok()?;
        Some(())
    }
}

// ===============================================================================================
// Generic iterator
// ===============================================================================================

/// Iterator over a data source implementing [`std::io::Read`] and writing to [`std::io::Stdout`].
#[derive(Debug)]
pub struct RhexdumpStdoutIter<'r, R: Read, X: RhexdumpGetConfig + Copy> {
    /// The original Rhexdump object.
    iter: RhexdumpStringIter<'r, R, X>,
    /// Standard output.
    stdout: std::io::Stdout,
}

impl<'r, R: Read, X: RhexdumpGetConfig + Copy> RhexdumpStdoutIter<'r, R, X> {
    /// Creates a new instance of the iterator.
    ///
    /// # Example
    ///
    /// ```
    /// use rhexdump::prelude::*;
    ///
    /// // Create a Rhexdump instance.
    /// let rhx = Rhexdump::new();
    ///
    /// // String that will be formatted.
    /// let input = String::from("Lorem ipsum dolor sit amet, consectetur adipiscing elit");
    /// let mut cur = std::io::Cursor::new(&input);
    ///
    /// // Creating an iterator.
    /// let mut iter = RhexdumpStdoutIter::new(rhx, &mut cur);
    ///
    /// // Taking two lines of output.
    /// let _ = iter.next().unwrap();
    /// let _ = iter.next().unwrap();
    /// ```
    pub fn new(rhx: X, src: &'r mut R) -> Self {
        Self {
            iter: RhexdumpStringIter::new(rhx, src),
            stdout: std::io::stdout(),
        }
    }

    /// Sets the hexdump offset.
    ///
    /// # Example
    ///
    /// ```
    /// use rhexdump::prelude::*;
    ///
    /// // Create a Rhexdump instance.
    /// let rhx = Rhexdump::new();
    ///
    /// // String that will be formatted.
    /// let input = String::from("Lorem ipsum dolor sit amet, consectetur adipiscing elit");
    /// let mut cur = std::io::Cursor::new(&input);
    ///
    /// // Creating an iterator with an offset.
    /// let mut iter = RhexdumpStdoutIter::new(rhx, &mut cur).offset(0x12340000);
    /// ```
    pub fn offset(mut self, offset: u64) -> Self {
        self.iter = self.iter.offset(offset);
        self
    }
}

impl<'r, R: Read, X: RhexdumpGetConfig + Copy> Iterator for RhexdumpStdoutIter<'r, R, X> {
    type Item = ();

    /// Returns one line of formatted bytes from the byte array according to the configuration of
    /// the associated Rhexdump object.
    fn next(&mut self) -> Option<Self::Item> {
        let output = self.iter.next()?;
        writeln!(self.stdout, "{}", output).ok()?;
        Some(())
    }
}

// ===============================================================================================
// Test
// ===============================================================================================

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use std::io::*;

    #[test]
    fn rhx_iter_string() {
        // Create a Rhexdump instance.
        let rhx = Rhexdump::new();

        // String that will be formatted.
        let input = String::from("Lorem ipsum dolor sit amet, consectetur adipiscing elit");
        let mut cur = Cursor::new(&input);

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

    #[test]
    fn rhx_iter_generic() {
        // Create a Rhexdump instance.
        let rhx = Rhexdump::new();

        // String that will be formatted.
        let input = String::from("Lorem ipsum dolor sit amet, consectetur adipiscing elit");
        let mut cur_in = Cursor::new(&input);
        let mut output = Vec::new();
        let mut cur_out = Cursor::new(&mut output);

        // Creating an iterator.
        let mut iter = RhexdumpIter::new(rhx, &mut cur_out, &mut cur_in);

        // Taking two lines of output.
        let _ = iter.next().unwrap();
        let _ = iter.next().unwrap();

        assert_eq!(
            &String::from_utf8_lossy(&output),
            "00000000: 4c 6f 72 65 6d 20 69 70 73 75 6d 20 64 6f 6c 6f  Lorem.ipsum.dolo\n\
             00000010: 72 20 73 69 74 20 61 6d 65 74 2c 20 63 6f 6e 73  r.sit.amet,.cons\n"
        );
    }

    #[test]
    fn rhx_iter_stdout() {
        // Create a Rhexdump instance.
        let rhx = Rhexdump::new();

        // String that will be formatted.
        let input = String::from("Lorem ipsum dolor sit amet, consectetur adipiscing elit");
        let mut cur = Cursor::new(&input);

        // Creating an iterator.
        let mut iter = RhexdumpStdoutIter::new(rhx, &mut cur);

        // Taking two lines of output.
        let _ = iter.next().unwrap();
        let _ = iter.next().unwrap();
    }
}
