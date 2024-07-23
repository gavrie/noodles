use std::path::Path;

use noodles_bcf as bcf;
use noodles_bgzf as bgzf;
use noodles_vcf as vcf;
use tokio::{
    fs::File,
    io::{self, AsyncBufRead, AsyncBufReadExt, AsyncRead, BufReader},
};

use super::Reader;
use crate::variant::io::{CompressionMethod, Format};

/// An async variant reader builder.
#[derive(Default)]
pub struct Builder;

impl Builder {
    /// Builds an async variant reader from a path.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # #[tokio::main]
    /// # async fn main() -> tokio::io::Result<()> {
    /// use noodles_util::variant::r#async::io::reader::Builder;
    /// let _reader = Builder::default().build_from_path("samples.vcf").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn build_from_path<P>(
        self,
        src: P,
    ) -> io::Result<Reader<Box<dyn AsyncBufRead + Unpin>>>
    where
        P: AsRef<Path>,
    {
        let file = File::open(src).await?;
        self.build_from_reader(file).await
    }

    /// Builds an async variant reader from a reader.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[tokio::main]
    /// # async fn main() -> tokio::io::Result<()> {
    /// use noodles_util::variant::r#async::io::reader::Builder;
    /// use tokio::io;
    /// let reader = Builder::default().build_from_reader(io::empty()).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn build_from_reader<R>(
        self,
        reader: R,
    ) -> io::Result<Reader<Box<dyn AsyncBufRead + Unpin>>>
    where
        R: AsyncRead + Unpin + 'static,
    {
        use crate::variant::io::reader::builder::{detect_compression_method, detect_format};

        let mut reader = BufReader::new(reader);

        let mut src = reader.fill_buf().await?;
        let compression_method = detect_compression_method(&mut src)?;

        let mut src = reader.fill_buf().await?;
        let format = detect_format(&mut src, compression_method)?;

        let reader = match (format, compression_method) {
            (Format::Vcf, None) => {
                let inner: Box<dyn AsyncBufRead + Unpin> = Box::new(reader);
                Reader::Vcf(vcf::r#async::io::Reader::new(inner))
            }
            (Format::Vcf, Some(CompressionMethod::Bgzf)) => {
                let decoder: Box<dyn AsyncBufRead + Unpin> =
                    Box::new(bgzf::r#async::Reader::new(reader));
                Reader::Vcf(vcf::r#async::io::Reader::new(decoder))
            }
            (Format::Bcf, None) => {
                let inner: Box<dyn AsyncBufRead + Unpin> = Box::new(reader);
                Reader::Bcf(bcf::r#async::io::Reader::from(inner))
            }
            (Format::Bcf, Some(CompressionMethod::Bgzf)) => {
                let decoder: Box<dyn AsyncBufRead + Unpin> =
                    Box::new(bgzf::r#async::Reader::new(reader));
                Reader::Bcf(bcf::r#async::io::Reader::from(decoder))
            }
        };

        Ok(reader)
    }
}
