//! Editor commands for alignment manipulation.

#![allow(dead_code)]

use crate::app::App;

impl App {
    /// Insert a gap at the cursor position in the current sequence.
    pub fn insert_gap(&mut self) {
        self.save_undo_state();

        if let Some(seq) = self.alignment.sequences.get_mut(self.cursor_row) {
            seq.insert_gap(self.cursor_col, self.gap_char);

            // Also update associated #=GR annotations
            if let Some(annotations) = self.alignment.residue_annotations.get_mut(&seq.id) {
                for ann in annotations {
                    if self.cursor_col <= ann.data.len() {
                        ann.data.insert(self.cursor_col, self.gap_char);
                    }
                }
            }
        }

        self.mark_modified();
        self.cursor_right();
    }

    /// Delete a gap at the cursor position in the current sequence.
    pub fn delete_gap(&mut self) -> bool {
        if !self.is_current_gap() {
            self.set_status("Not a gap character");
            return false;
        }

        self.save_undo_state();

        let seq_id = self.alignment.sequences.get(self.cursor_row)
            .map(|s| s.id.clone());

        if let Some(seq) = self.alignment.sequences.get_mut(self.cursor_row) {
            if seq.delete_gap(self.cursor_col, &self.gap_chars) {
                // Also update associated #=GR annotations
                if let Some(id) = seq_id {
                    if let Some(annotations) = self.alignment.residue_annotations.get_mut(&id) {
                        for ann in annotations {
                            if self.cursor_col < ann.data.len() {
                                ann.data.remove(self.cursor_col);
                            }
                        }
                    }
                }
                self.mark_modified();
                return true;
            }
        }

        false
    }

    /// Insert a gap column at the cursor position.
    pub fn insert_gap_column(&mut self) {
        self.save_undo_state();
        self.alignment.insert_gap_column(self.cursor_col, self.gap_char);
        self.mark_modified();
        self.update_structure_cache();
    }

    /// Delete a gap column at the cursor position.
    pub fn delete_gap_column(&mut self) -> bool {
        if self.alignment.delete_gap_column(self.cursor_col, &self.gap_chars) {
            self.save_undo_state();
            self.mark_modified();
            self.clamp_cursor();
            self.update_structure_cache();
            true
        } else {
            self.set_status("Column contains non-gap characters");
            false
        }
    }

    /// Shift current sequence left.
    pub fn shift_sequence_left(&mut self) -> bool {
        self.save_undo_state();

        let seq_id = self.alignment.sequences.get(self.cursor_row)
            .map(|s| s.id.clone());

        if let Some(seq) = self.alignment.sequences.get_mut(self.cursor_row) {
            if seq.shift_left(self.cursor_col, &self.gap_chars) {
                // Also shift associated #=GR annotations
                if let Some(id) = seq_id {
                    if let Some(annotations) = self.alignment.residue_annotations.get_mut(&id) {
                        for ann in annotations {
                            let mut temp = crate::stockholm::Sequence::new("temp", ann.data.clone());
                            temp.shift_left(self.cursor_col, &self.gap_chars);
                            ann.data = temp.data;
                        }
                    }
                }
                self.mark_modified();
                return true;
            }
        }

        self.set_status("Cannot shift left (no gap found)");
        false
    }

    /// Shift current sequence right.
    pub fn shift_sequence_right(&mut self) -> bool {
        self.save_undo_state();

        let seq_id = self.alignment.sequences.get(self.cursor_row)
            .map(|s| s.id.clone());

        if let Some(seq) = self.alignment.sequences.get_mut(self.cursor_row) {
            if seq.shift_right(self.cursor_col, &self.gap_chars) {
                // Also shift associated #=GR annotations
                if let Some(id) = seq_id {
                    if let Some(annotations) = self.alignment.residue_annotations.get_mut(&id) {
                        for ann in annotations {
                            let mut temp = crate::stockholm::Sequence::new("temp", ann.data.clone());
                            temp.shift_right(self.cursor_col, &self.gap_chars);
                            ann.data = temp.data;
                        }
                    }
                }
                self.mark_modified();
                return true;
            }
        }

        self.set_status("Cannot shift right (no gap found)");
        false
    }

    /// Throw sequence left (shift as far as possible).
    pub fn throw_sequence_left(&mut self) {
        self.save_undo_state();
        let mut shifted = false;
        while self.shift_sequence_left_internal() {
            shifted = true;
        }
        if shifted {
            self.mark_modified();
        } else {
            self.set_status("Cannot throw left (no gaps found)");
        }
    }

    /// Throw sequence right (shift as far as possible).
    pub fn throw_sequence_right(&mut self) {
        self.save_undo_state();
        let mut shifted = false;
        while self.shift_sequence_right_internal() {
            shifted = true;
        }
        if shifted {
            self.mark_modified();
        } else {
            self.set_status("Cannot throw right (no gaps found)");
        }
    }

    /// Internal shift left without undo/status.
    fn shift_sequence_left_internal(&mut self) -> bool {
        let seq_id = self.alignment.sequences.get(self.cursor_row)
            .map(|s| s.id.clone());

        if let Some(seq) = self.alignment.sequences.get_mut(self.cursor_row) {
            if seq.shift_left(self.cursor_col, &self.gap_chars) {
                if let Some(id) = seq_id {
                    if let Some(annotations) = self.alignment.residue_annotations.get_mut(&id) {
                        for ann in annotations {
                            let mut temp = crate::stockholm::Sequence::new("temp", ann.data.clone());
                            temp.shift_left(self.cursor_col, &self.gap_chars);
                            ann.data = temp.data;
                        }
                    }
                }
                return true;
            }
        }
        false
    }

    /// Internal shift right without undo/status.
    fn shift_sequence_right_internal(&mut self) -> bool {
        let seq_id = self.alignment.sequences.get(self.cursor_row)
            .map(|s| s.id.clone());

        if let Some(seq) = self.alignment.sequences.get_mut(self.cursor_row) {
            if seq.shift_right(self.cursor_col, &self.gap_chars) {
                if let Some(id) = seq_id {
                    if let Some(annotations) = self.alignment.residue_annotations.get_mut(&id) {
                        for ann in annotations {
                            let mut temp = crate::stockholm::Sequence::new("temp", ann.data.clone());
                            temp.shift_right(self.cursor_col, &self.gap_chars);
                            ann.data = temp.data;
                        }
                    }
                }
                return true;
            }
        }
        false
    }

    /// Undo the last action.
    pub fn undo(&mut self) {
        if let Some(snapshot) = self.history.undo(
            &self.alignment,
            self.cursor_row,
            self.cursor_col,
        ) {
            self.alignment = snapshot.alignment;
            self.cursor_row = snapshot.cursor_row;
            self.cursor_col = snapshot.cursor_col;
            self.modified = true; // Still modified from original save
            self.update_structure_cache();
            self.set_status("Undo");
        } else {
            self.set_status("Nothing to undo");
        }
    }

    /// Redo the last undone action.
    pub fn redo(&mut self) {
        if let Some(snapshot) = self.history.redo(
            &self.alignment,
            self.cursor_row,
            self.cursor_col,
        ) {
            self.alignment = snapshot.alignment;
            self.cursor_row = snapshot.cursor_row;
            self.cursor_col = snapshot.cursor_col;
            self.modified = true;
            self.update_structure_cache();
            self.set_status("Redo");
        } else {
            self.set_status("Nothing to redo");
        }
    }

    /// Save current state for undo.
    fn save_undo_state(&mut self) {
        self.history.save(&self.alignment, self.cursor_row, self.cursor_col);
    }

    /// Delete the current sequence.
    pub fn delete_sequence(&mut self) {
        if self.alignment.sequences.is_empty() {
            return;
        }

        self.save_undo_state();

        let seq_id = self.alignment.sequences[self.cursor_row].id.clone();
        self.alignment.sequences.remove(self.cursor_row);

        // Remove associated annotations
        self.alignment.sequence_annotations.remove(&seq_id);
        self.alignment.residue_annotations.remove(&seq_id);

        self.mark_modified();
        self.clamp_cursor();
    }

    /// Convert alignment to uppercase.
    pub fn uppercase_alignment(&mut self) {
        self.save_undo_state();
        for seq in &mut self.alignment.sequences {
            seq.data = seq.data.to_uppercase();
        }
        self.mark_modified();
    }

    /// Convert alignment to lowercase.
    pub fn lowercase_alignment(&mut self) {
        self.save_undo_state();
        for seq in &mut self.alignment.sequences {
            seq.data = seq.data.to_lowercase();
        }
        self.mark_modified();
    }

    /// Convert T to U in all sequences.
    pub fn convert_t_to_u(&mut self) {
        self.save_undo_state();
        for seq in &mut self.alignment.sequences {
            seq.data = seq.data.replace('T', "U").replace('t', "u");
        }
        self.mark_modified();
    }

    /// Convert U to T in all sequences.
    pub fn convert_u_to_t(&mut self) {
        self.save_undo_state();
        for seq in &mut self.alignment.sequences {
            seq.data = seq.data.replace('U', "T").replace('u', "t");
        }
        self.mark_modified();
    }
}
