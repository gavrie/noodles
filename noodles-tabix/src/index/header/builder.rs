//! Tabix index header builder.

use super::{format::CoordinateSystem, Format, Header};

/// A tabix index header builder.
pub struct Builder {
    format: Format,
    reference_sequence_name_index: usize,
    start_position_index: usize,
    end_position_index: Option<usize>,
    line_comment_prefix: u8,
    line_skip_count: u32,
}

impl Builder {
    /// Creates a builder that targets the BED format.
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_tabix as tabix;
    /// let builder = tabix::index::header::Builder::bed();
    /// ```
    pub fn bed() -> Self {
        Builder {
            format: Format::Generic(CoordinateSystem::Bed),
            reference_sequence_name_index: 1,
            start_position_index: 2,
            end_position_index: Some(3),
            line_comment_prefix: b'#',
            line_skip_count: 0,
        }
    }

    /// Creates a builder that targets the GFF format.
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_tabix as tabix;
    /// let builder = tabix::index::header::Builder::gff();
    /// ```
    pub fn gff() -> Self {
        Builder {
            format: Format::Generic(CoordinateSystem::Gff),
            reference_sequence_name_index: 1,
            start_position_index: 4,
            end_position_index: Some(5),
            line_comment_prefix: b'#',
            line_skip_count: 0,
        }
    }

    /// Creates a builder that targets the SAM format.
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_tabix as tabix;
    /// let builder = tabix::index::header::Builder::sam();
    /// ```
    pub fn sam() -> Self {
        Builder {
            format: Format::Sam,
            reference_sequence_name_index: 3,
            start_position_index: 4,
            end_position_index: None,
            line_comment_prefix: b'@',
            line_skip_count: 0,
        }
    }

    /// Creates a builder that targets the VCF format.
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_tabix as tabix;
    /// let builder = tabix::index::header::Builder::vcf();
    /// ```
    pub fn vcf() -> Self {
        Builder {
            format: Format::Vcf,
            reference_sequence_name_index: 1,
            start_position_index: 2,
            end_position_index: None,
            line_comment_prefix: b'#',
            line_skip_count: 0,
        }
    }

    /// Sets a format.
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_tabix::{self as tabix, index::header::Format};
    ///
    /// let header = tabix::index::Header::builder()
    ///     .set_format(Format::Vcf)
    ///     .build();
    ///
    /// assert_eq!(header.format(), Format::Vcf);
    /// ```
    pub fn set_format(mut self, format: Format) -> Self {
        self.format = format;
        self
    }

    /// Sets a reference sequence name index.
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_tabix as tabix;
    ///
    /// let header = tabix::index::Header::builder()
    ///     .set_reference_sequence_name_index(1)
    ///     .build();
    ///
    /// assert_eq!(header.reference_sequence_name_index(), 1);
    /// ```
    pub fn set_reference_sequence_name_index(
        mut self,
        reference_sequence_name_index: usize,
    ) -> Self {
        self.reference_sequence_name_index = reference_sequence_name_index;
        self
    }

    /// Sets a start position index.
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_tabix as tabix;
    ///
    /// let header = tabix::index::Header::builder()
    ///     .set_start_position_index(4)
    ///     .build();
    ///
    /// assert_eq!(header.start_position_index(), 4);
    /// ```
    pub fn set_start_position_index(mut self, start_position_index: usize) -> Self {
        self.start_position_index = start_position_index;
        self
    }

    /// Sets an end position index.
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_tabix as tabix;
    ///
    /// let header = tabix::index::Header::builder()
    ///     .set_end_position_index(Some(5))
    ///     .build();
    ///
    /// assert_eq!(header.end_position_index(), Some(5));
    /// ```
    pub fn set_end_position_index(mut self, end_position_index: Option<usize>) -> Self {
        self.end_position_index = end_position_index;
        self
    }

    /// Sets a line comment prefix.
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_tabix as tabix;
    ///
    /// let header = tabix::index::Header::builder()
    ///     .set_line_comment_prefix(b'#')
    ///     .build();
    ///
    /// assert_eq!(header.line_comment_prefix(), b'#');
    /// ```
    pub fn set_line_comment_prefix(mut self, line_comment_prefix: u8) -> Self {
        self.line_comment_prefix = line_comment_prefix;
        self
    }

    /// Sets a line skip count.
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_tabix as tabix;
    ///
    /// let header = tabix::index::Header::builder()
    ///     .set_line_skip_count(0)
    ///     .build();
    ///
    /// assert_eq!(header.line_skip_count(), 0);
    /// ```
    pub fn set_line_skip_count(mut self, line_skip_count: u32) -> Self {
        self.line_skip_count = line_skip_count;
        self
    }

    /// Builds a tabix index header.
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_tabix as tabix;
    /// let index = tabix::index::Header::builder().build();
    /// ```
    pub fn build(self) -> Header {
        Header {
            format: self.format,
            reference_sequence_name_index: self.reference_sequence_name_index,
            start_position_index: self.start_position_index,
            end_position_index: self.end_position_index,
            line_comment_prefix: self.line_comment_prefix,
            line_skip_count: self.line_skip_count,
        }
    }
}

impl Default for Builder {
    fn default() -> Self {
        Self::gff()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bed() {
        let builder = Builder::bed();
        assert_eq!(builder.format, Format::Generic(CoordinateSystem::Bed));
        assert_eq!(builder.reference_sequence_name_index, 1);
        assert_eq!(builder.start_position_index, 2);
        assert_eq!(builder.end_position_index, Some(3));
        assert_eq!(builder.line_comment_prefix, b'#');
        assert_eq!(builder.line_skip_count, 0);
    }

    #[test]
    fn test_gff() {
        let builder = Builder::gff();
        assert_eq!(builder.format, Format::Generic(CoordinateSystem::Gff));
        assert_eq!(builder.reference_sequence_name_index, 1);
        assert_eq!(builder.start_position_index, 4);
        assert_eq!(builder.end_position_index, Some(5));
        assert_eq!(builder.line_comment_prefix, b'#');
        assert_eq!(builder.line_skip_count, 0);
    }

    #[test]
    fn test_sam() {
        let builder = Builder::sam();
        assert_eq!(builder.format, Format::Sam);
        assert_eq!(builder.reference_sequence_name_index, 3);
        assert_eq!(builder.start_position_index, 4);
        assert_eq!(builder.end_position_index, None);
        assert_eq!(builder.line_comment_prefix, b'@');
        assert_eq!(builder.line_skip_count, 0);
    }

    #[test]
    fn test_vcf() {
        let builder = Builder::vcf();
        assert_eq!(builder.format, Format::Vcf);
        assert_eq!(builder.reference_sequence_name_index, 1);
        assert_eq!(builder.start_position_index, 2);
        assert_eq!(builder.end_position_index, None);
        assert_eq!(builder.line_comment_prefix, b'#');
        assert_eq!(builder.line_skip_count, 0);
    }
}
