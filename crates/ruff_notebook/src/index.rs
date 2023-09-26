use serde::{Deserialize, Serialize};

use ruff_source_file::{OneIndexed, SourceLocation};

/// Jupyter Notebook indexing table
///
/// When we lint a jupyter notebook, we have to translate the row/column based on
/// [`ruff_text_size::TextSize`] to jupyter notebook cell/row/column.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct NotebookIndex {
    /// Enter a row (1-based), get back the cell (1-based)
    pub(super) row_to_cell: Vec<u32>,
    /// Enter a row (1-based), get back the row in cell (1-based)
    pub(super) row_to_row_in_cell: Vec<u32>,
}

impl NotebookIndex {
    /// Returns the cell number (1-based) for the given row (1-based).
    pub fn cell(&self, row: usize) -> Option<u32> {
        self.row_to_cell.get(row).copied()
    }

    /// Returns the row number (1-based) in the cell (1-based) for the
    /// given row (1-based).
    pub fn cell_row(&self, row: usize) -> Option<u32> {
        self.row_to_row_in_cell.get(row).copied()
    }

    /// Translates the given source location based on the indexing table.
    ///
    /// This will translate the row/column in the concatenated source code
    /// to the row/column in the Jupyter Notebook.
    pub fn translated_location(&self, source_location: &SourceLocation) -> SourceLocation {
        SourceLocation {
            row: OneIndexed::new(self.cell_row(source_location.row.get()).unwrap_or(1) as usize)
                .unwrap(),
            column: source_location.column,
        }
    }
}
