//! Core types for Stockholm format alignments.

#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A Stockholm format alignment.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Alignment {
    /// File-level annotations (#=GF)
    pub file_annotations: Vec<FileAnnotation>,
    /// Sequences in the alignment
    pub sequences: Vec<Sequence>,
    /// Per-sequence annotations (#=GS)
    pub sequence_annotations: HashMap<String, Vec<SequenceAnnotation>>,
    /// Per-column annotations (#=GC)
    pub column_annotations: Vec<ColumnAnnotation>,
    /// Per-residue annotations (#=GR)
    pub residue_annotations: HashMap<String, Vec<ResidueAnnotation>>,
}

/// A sequence in the alignment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sequence {
    /// Sequence identifier (may include coordinates like "id/start-end")
    pub id: String,
    /// Sequence data (with gaps)
    pub data: String,
}

/// File-level annotation (#=GF tag value).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileAnnotation {
    pub tag: String,
    pub value: String,
}

/// Per-sequence annotation (#=GS seqid tag value).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SequenceAnnotation {
    pub tag: String,
    pub value: String,
}

/// Per-column annotation (#=GC tag data).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColumnAnnotation {
    pub tag: String,
    pub data: String,
}

/// Per-residue annotation (#=GR seqid tag data).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResidueAnnotation {
    pub tag: String,
    pub data: String,
}

impl Alignment {
    /// Create a new empty alignment.
    pub fn new() -> Self {
        Self::default()
    }

    /// Get the number of sequences.
    pub fn num_sequences(&self) -> usize {
        self.sequences.len()
    }

    /// Get the alignment width (number of columns).
    pub fn width(&self) -> usize {
        self.sequences.first().map(|s| s.data.len()).unwrap_or(0)
    }

    /// Get the consensus secondary structure annotation if present.
    pub fn ss_cons(&self) -> Option<&str> {
        self.column_annotations
            .iter()
            .find(|a| a.tag == "SS_cons")
            .map(|a| a.data.as_str())
    }

    /// Get a mutable reference to the consensus secondary structure.
    pub fn ss_cons_mut(&mut self) -> Option<&mut String> {
        self.column_annotations
            .iter_mut()
            .find(|a| a.tag == "SS_cons")
            .map(|a| &mut a.data)
    }

    /// Get the reference sequence annotation if present.
    pub fn rf(&self) -> Option<&str> {
        self.column_annotations
            .iter()
            .find(|a| a.tag == "RF")
            .map(|a| a.data.as_str())
    }

    /// Check if all sequences have the same length.
    pub fn is_valid(&self) -> bool {
        if self.sequences.is_empty() {
            return true;
        }
        let width = self.sequences[0].data.len();
        self.sequences.iter().all(|s| s.data.len() == width)
            && self
                .column_annotations
                .iter()
                .all(|a| a.data.len() == width)
    }

    /// Get the maximum sequence ID length (for formatting).
    pub fn max_id_len(&self) -> usize {
        self.sequences
            .iter()
            .map(|s| s.id.len())
            .max()
            .unwrap_or(0)
    }

    /// Insert a gap at a specific position in all sequences and annotations.
    pub fn insert_gap_column(&mut self, col: usize, gap_char: char) {
        for seq in &mut self.sequences {
            if col <= seq.data.len() {
                seq.data.insert(col, gap_char);
            }
        }
        for ann in &mut self.column_annotations {
            if col <= ann.data.len() {
                ann.data.insert(col, gap_char);
            }
        }
        for annotations in self.residue_annotations.values_mut() {
            for ann in annotations {
                if col <= ann.data.len() {
                    ann.data.insert(col, gap_char);
                }
            }
        }
    }

    /// Delete a column if it contains only gaps in all sequences.
    pub fn delete_gap_column(&mut self, col: usize, gap_chars: &[char]) -> bool {
        // Check if column is all gaps
        let all_gaps = self.sequences.iter().all(|s| {
            s.data
                .chars()
                .nth(col)
                .map(|c| gap_chars.contains(&c))
                .unwrap_or(false)
        });

        if !all_gaps {
            return false;
        }

        // Delete from all sequences
        for seq in &mut self.sequences {
            if col < seq.data.len() {
                seq.data.remove(col);
            }
        }
        for ann in &mut self.column_annotations {
            if col < ann.data.len() {
                ann.data.remove(col);
            }
        }
        for annotations in self.residue_annotations.values_mut() {
            for ann in annotations {
                if col < ann.data.len() {
                    ann.data.remove(col);
                }
            }
        }

        true
    }

    /// Get character at a specific position.
    pub fn get_char(&self, row: usize, col: usize) -> Option<char> {
        self.sequences.get(row)?.data.chars().nth(col)
    }

    /// Set character at a specific position.
    pub fn set_char(&mut self, row: usize, col: usize, ch: char) -> bool {
        if let Some(seq) = self.sequences.get_mut(row) {
            let mut chars: Vec<char> = seq.data.chars().collect();
            if col < chars.len() {
                chars[col] = ch;
                seq.data = chars.into_iter().collect();
                return true;
            }
        }
        false
    }
}

impl Sequence {
    /// Create a new sequence.
    pub fn new(id: impl Into<String>, data: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            data: data.into(),
        }
    }

    /// Insert a gap at a specific position.
    pub fn insert_gap(&mut self, pos: usize, gap_char: char) {
        if pos <= self.data.len() {
            self.data.insert(pos, gap_char);
        }
    }

    /// Delete a character at a specific position if it's a gap.
    pub fn delete_gap(&mut self, pos: usize, gap_chars: &[char]) -> bool {
        if let Some(ch) = self.data.chars().nth(pos) {
            if gap_chars.contains(&ch) {
                self.data.remove(pos);
                return true;
            }
        }
        false
    }

    /// Shift sequence left by one position (moves content to next gap on left).
    pub fn shift_left(&mut self, col: usize, gap_chars: &[char]) -> bool {
        let chars: Vec<char> = self.data.chars().collect();

        // Find the nearest gap to the left
        let mut gap_pos = None;
        for i in (0..col).rev() {
            if gap_chars.contains(&chars[i]) {
                gap_pos = Some(i);
                break;
            }
        }

        if let Some(gp) = gap_pos {
            // Remove gap at gp, then insert gap after the character at col
            // After removal, indices shift left, so we insert at col (not col-1)
            let mut new_chars = chars.clone();
            new_chars.remove(gp);
            new_chars.insert(col, gap_chars[0]);
            self.data = new_chars.into_iter().collect();
            return true;
        }

        false
    }

    /// Shift sequence right by one position (moves content to next gap on right).
    pub fn shift_right(&mut self, col: usize, gap_chars: &[char]) -> bool {
        let chars: Vec<char> = self.data.chars().collect();

        // Find the nearest gap to the right
        let mut gap_pos = None;
        for i in (col + 1)..chars.len() {
            if gap_chars.contains(&chars[i]) {
                gap_pos = Some(i);
                break;
            }
        }

        if let Some(gp) = gap_pos {
            // Remove gap at gp, insert gap at col
            let mut new_chars = chars.clone();
            new_chars.remove(gp);
            new_chars.insert(col, gap_chars[0]);
            self.data = new_chars.into_iter().collect();
            return true;
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alignment_width() {
        let mut alignment = Alignment::new();
        alignment.sequences.push(Sequence::new("seq1", "ACGU..ACGU"));
        alignment.sequences.push(Sequence::new("seq2", "ACGU..ACGU"));
        assert_eq!(alignment.width(), 10);
    }

    #[test]
    fn test_insert_gap_column() {
        let mut alignment = Alignment::new();
        alignment.sequences.push(Sequence::new("seq1", "ACGU"));
        alignment.insert_gap_column(2, '.');
        assert_eq!(alignment.sequences[0].data, "AC.GU");
    }

    #[test]
    fn test_sequence_shift_left() {
        let mut seq = Sequence::new("test", "A.CGU");
        assert!(seq.shift_left(2, &['.']));
        assert_eq!(seq.data, "AC.GU");
    }

    #[test]
    fn test_sequence_shift_right() {
        let mut seq = Sequence::new("test", "ACG.U");
        assert!(seq.shift_right(2, &['.']));
        assert_eq!(seq.data, "AC.GU");
    }
}
