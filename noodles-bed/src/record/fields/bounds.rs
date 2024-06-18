use std::ops::Range;

pub(super) struct Bounds {
    reference_sequence_name_end: usize,
    feature_start_end: usize,
    feature_end_end: usize,
    other_fields_ends: Vec<usize>,
}

impl Bounds {
    pub fn reference_sequence_name_range(&self) -> Range<usize> {
        0..self.reference_sequence_name_end
    }

    pub fn feature_start_range(&self) -> Range<usize> {
        self.reference_sequence_name_end..self.feature_start_end
    }

    pub fn feature_end_range(&self) -> Range<usize> {
        self.feature_start_end..self.feature_end_end
    }

    pub fn get(&self, i: usize) -> Option<Range<usize>> {
        let end = self.other_fields_ends.get(i).copied()?;

        let start = i
            .checked_sub(1)
            .and_then(|prev_i| self.other_fields_ends.get(prev_i).copied())
            .unwrap_or_default();

        Some(start..end)
    }
}
