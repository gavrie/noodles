mod alternative_allele;
mod filter;
mod format;
mod info;
mod number;
mod record;

pub use self::{
    alternative_allele::AlternativeAllele, filter::Filter, format::Format, info::Info,
    number::Number,
};

use std::{convert::TryFrom, str::FromStr};

use self::record::Record;

#[derive(Debug, Default)]
pub struct Header {
    infos: Vec<Info>,
    filters: Vec<Filter>,
    formats: Vec<Format>,
    alternative_alleles: Vec<AlternativeAllele>,
}

impl Header {
    pub fn infos(&self) -> &[Info] {
        &self.infos
    }

    pub fn filters(&self) -> &[Filter] {
        &self.filters
    }

    pub fn formats(&self) -> &[Format] {
        &self.formats
    }

    pub fn alternative_alleles(&self) -> &[AlternativeAllele] {
        &self.alternative_alleles
    }
}

#[derive(Debug)]
pub enum ParseError {
    InvalidRecord(record::ParseError),
    InvalidInfo(info::ParseError),
    InvalidFilter(filter::ParseError),
    InvalidFormat(format::ParseError),
    InvalidAlternativeAllele(alternative_allele::ParseError),
}

impl FromStr for Header {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut header = Header::default();

        for line in s.lines() {
            let record = line.parse().map_err(ParseError::InvalidRecord)?;

            match record {
                Record::Info(fields) => {
                    let info = Info::try_from(&fields[..]).map_err(ParseError::InvalidInfo)?;
                    header.infos.push(info);
                }
                Record::Filter(fields) => {
                    let filter =
                        Filter::try_from(&fields[..]).map_err(ParseError::InvalidFilter)?;
                    header.filters.push(filter);
                }
                Record::Format(fields) => {
                    let format =
                        Format::try_from(&fields[..]).map_err(ParseError::InvalidFormat)?;
                    header.formats.push(format);
                }
                Record::AlternativeAllele(fields) => {
                    let alternative_allele = AlternativeAllele::try_from(&fields[..])
                        .map_err(ParseError::InvalidAlternativeAllele)?;
                    header.alternative_alleles.push(alternative_allele);
                }
            }
        }

        Ok(header)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str() -> Result<(), ParseError> {
        let s = r#"##INFO=<ID=NS,Number=1,Type=Integer,Description="Number of samples with data">
##FILTER=<ID=q10,Description="Quality below 10">
##FORMAT=<ID=GT,Number=1,Type=String,Description="Genotype">
##ALT=<ID=DEL,Description="Deletion">
"#;

        let header: Header = s.parse()?;

        assert_eq!(header.infos().len(), 1);
        assert_eq!(header.filters().len(), 1);
        assert_eq!(header.formats().len(), 1);
        assert_eq!(header.alternative_alleles().len(), 1);

        Ok(())
    }
}
