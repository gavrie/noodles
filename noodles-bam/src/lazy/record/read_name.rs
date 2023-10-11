use std::{io, num::NonZeroUsize};

use noodles_sam as sam;

/// A raw BAM record read name.
#[derive(Debug, Eq, PartialEq)]
pub struct ReadName<'a>(&'a [u8]);

impl<'a> ReadName<'a> {
    pub(super) fn new(src: &'a [u8]) -> Self {
        Self(src)
    }

    /// Returns the read name as a byte slice.
    ///
    /// The returned slice will _not_ have the trailing `NUL` terminator.
    pub fn as_bytes(&self) -> &[u8] {
        const NUL: u8 = 0x00;
        self.as_ref().strip_suffix(&[NUL]).unwrap_or(self.as_ref())
    }
}

impl<'a> AsRef<[u8]> for ReadName<'a> {
    fn as_ref(&self) -> &[u8] {
        self.0
    }
}

impl<'a> TryFrom<ReadName<'a>> for sam::record::ReadName {
    type Error = io::Error;

    fn try_from(bam_read_name: ReadName<'a>) -> Result<Self, Self::Error> {
        use crate::record::codec::decoder::get_read_name;

        let mut src = bam_read_name.0;

        let mut read_name = None;
        let len = NonZeroUsize::try_from(src.len())
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        get_read_name(&mut src, &mut read_name, len)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        Ok(read_name.unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_as_bytes() {
        let read_name = ReadName::new(b"r0\x00");
        assert_eq!(read_name.as_bytes(), b"r0");

        let read_name = ReadName::new(b"r0");
        assert_eq!(read_name.as_bytes(), b"r0");
    }
}
