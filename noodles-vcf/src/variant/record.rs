//! Variant record.

mod alternate_bases;
mod filters;
mod ids;
pub mod info;
pub mod samples;

use std::io;

use noodles_core::Position;

pub use self::{
    alternate_bases::AlternateBases, filters::Filters, ids::Ids, info::Info, samples::Samples,
};
use crate::Header;

/// A variant record.
pub trait Record {
    /// Returns the reference sequence name.
    fn reference_sequence_name<'a, 'h: 'a>(&'a self, header: &'h Header) -> io::Result<&'a str>;

    /// Returns the position.
    fn position(&self) -> Option<io::Result<Position>>;

    /// Returns the IDs.
    fn ids(&self) -> Box<dyn Ids + '_>;

    /// Returns the reference bases.
    fn reference_bases(&self) -> &str;

    /// Returns the alternate bases.
    fn alternate_bases(&self) -> Box<dyn AlternateBases + '_>;

    /// Returns the quality scores.
    fn quality_score(&self) -> Option<io::Result<f32>>;

    /// Returns the filters.
    fn filters(&self) -> Box<dyn Filters + '_>;

    /// Return the info fields.
    fn info(&self) -> Box<dyn Info + '_>;

    /// Returns the samples.
    fn samples(&self) -> io::Result<Box<dyn Samples + '_>>;

    /// Returns or calculates the end.
    fn end(&self, header: &Header) -> io::Result<Position> {
        use self::info::field::Value;
        use super::record_buf::info::field::key;

        if let Some(Some(value)) = self.info().get(header, key::END_POSITION).transpose()? {
            match value {
                Value::Integer(n) => {
                    usize::try_from(n)
                        .and_then(Position::try_from)
                        .map_err(|_| {
                            io::Error::new(io::ErrorKind::InvalidData, "invalid INFO END position")
                        })
                }
                _ => Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "invalid INFO END position value",
                )),
            }
        } else {
            let start = self.position().transpose()?.unwrap_or(Position::MIN);
            let reference_bases = self.reference_bases();

            if reference_bases.is_empty() {
                Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "invalid reference bases length",
                ))
            } else {
                let len = reference_bases.len();
                start
                    .checked_add(len - 1)
                    .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "position overflow"))
            }
        }
    }
}
