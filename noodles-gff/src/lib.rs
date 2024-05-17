#![warn(missing_docs)]

//! **noodles-gff** handles the reading and writing of the [GFF3 format][gff3-spec].
//!
//! GFF (Generic Feature Format) is a text-based format used to represent genomic features.
//!
//! [gff3-spec]: https://github.com/The-Sequence-Ontology/Specifications/blob/be6e1af7243ba4235c30b69660e2669e444e2f3e/gff3.md
//!
//! # Examples
//!
//! ## Read all records in a GFF3 file
//!
//! ```no_run
//! # use std::{fs::File, io::{self, BufReader}};
//! use noodles_gff as gff;
//!
//! let mut reader = File::open("annotations.gff3")
//!     .map(BufReader::new)
//!     .map(gff::io::Reader::new)?;
//!
//! for result in reader.records() {
//!     let record = result?;
//!
//!     println!(
//!         "{}\t{}\t{}",
//!         record.reference_sequence_name(),
//!         record.start(),
//!         record.end(),
//!     );
//! }
//! # Ok::<(), io::Error>(())
//! ```

#[cfg(feature = "async")]
pub mod r#async;

pub mod directive;
pub mod io;
pub mod lazy;
pub mod line;
pub mod record;
mod writer;

pub use self::{directive::Directive, line::Line, record::Record, writer::Writer};

#[deprecated(since = "0.33.0", note = "Use `noodles_gff::io::Reader` instead.")]
pub use self::io::Reader;

#[cfg(feature = "async")]
pub use self::r#async::io::Reader as AsyncReader;
