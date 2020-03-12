mod molecule_topology;
mod tag;

use std::{collections::HashMap, convert::TryFrom};

pub use self::{molecule_topology::MoleculeTopology, tag::Tag};

#[derive(Debug)]
pub struct ReferenceSequence {
    name: String,
    len: i32,
    fields: HashMap<Tag, String>,
}

impl ReferenceSequence {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn len(&self) -> i32 {
        self.len
    }

    pub fn get(&self, tag: &Tag) -> Option<&String> {
        self.fields.get(tag)
    }
}

impl Default for ReferenceSequence {
    fn default() -> Self {
        Self {
            name: String::new(),
            len: 0,
            fields: HashMap::new(),
        }
    }
}

impl TryFrom<&[(String, String)]> for ReferenceSequence {
    type Error = ();

    fn try_from(raw_fields: &[(String, String)]) -> Result<Self, Self::Error> {
        let mut reference_sequence = ReferenceSequence::default();

        let mut has_name = false;
        let mut has_len = false;

        for (raw_tag, value) in raw_fields {
            let tag = raw_tag.parse()?;

            match tag {
                Tag::Name => {
                    reference_sequence.name = value.into();
                    has_name = true;
                }
                Tag::Len => {
                    reference_sequence.len = value.parse().map_err(|_| ())?;
                    has_len = true;
                }
                _ => {}
            }

            reference_sequence.fields.insert(tag, value.into());
        }

        if !has_name {
            return Err(());
        } else if !has_len {
            return Err(());
        }

        Ok(reference_sequence)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str_with_missing_name() {
        let fields = [
            (String::from("LN"), String::from("1")),
            (
                String::from("M5"),
                String::from("d7eba311421bbc9d3ada44709dd61534"),
            ),
        ];

        assert!(ReferenceSequence::try_from(&fields[..]).is_err());
    }

    #[test]
    fn test_from_str_with_missing_length() {
        let fields = [
            (String::from("SN"), String::from("sq0")),
            (
                String::from("M5"),
                String::from("d7eba311421bbc9d3ada44709dd61534"),
            ),
        ];

        assert!(ReferenceSequence::try_from(&fields[..]).is_err());
    }

    #[test]
    fn test_from_str_with_missing_name_and_length() {
        let fields = [(
            String::from("M5"),
            String::from("d7eba311421bbc9d3ada44709dd61534"),
        )];

        assert!(ReferenceSequence::try_from(&fields[..]).is_err());
    }
}
