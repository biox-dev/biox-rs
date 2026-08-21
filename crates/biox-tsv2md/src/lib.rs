/// Convert TSV (tab-separated values) input into a Markdown table.
///
/// The first non-empty row is treated as the header row. Empty lines are
/// ignored, and rows are padded to the widest row so every row has the same
/// number of columns. Pipe characters (`|`) inside cells are escaped.
pub fn tsv2md(input: &str) -> String {
    let rows: Vec<Vec<String>> = input
        .lines()
        .map(|line| line.trim_end_matches('\r'))
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            line.split('\t')
                .map(|cell| cell.replace('|', "\\|"))
                .collect::<Vec<String>>()
        })
        .collect();

    let Some(header) = rows.first() else {
        return String::new();
    };

    let col_count = rows.iter().map(Vec::len).max().unwrap_or(header.len());

    let mut widths = vec![0usize; col_count];
    for row in &rows {
        for (i, cell) in row.iter().enumerate() {
            widths[i] = widths[i].max(cell.chars().count());
        }
    }

    let format_row = |row: &[String]| -> String {
        let cells = (0..col_count)
            .map(|i| {
                let cell = row.get(i).map(String::as_str).unwrap_or("");
                let width = widths[i];
                format!(" {:<width$} ", cell, width = width)
            })
            .collect::<Vec<String>>()
            .join("|");
        format!("|{}|", cells)
    };

    let separator = widths
        .iter()
        .map(|w| format!("|{}", "-".repeat(w + 2)))
        .collect::<String>()
        + "|";

    let mut out = Vec::with_capacity(rows.len() + 1);
    out.push(format_row(header));
    out.push(separator);
    for row in &rows[1..] {
        out.push(format_row(row));
    }

    out.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converts_basic_table() {
        let input = "Name\tAge\nAlice\t30\nBob\t25";
        let expected = "\
| Name  | Age |
|-------|-----|
| Alice | 30  |
| Bob   | 25  |";
        assert_eq!(tsv2md(input), expected);
    }

    #[test]
    fn pads_ragged_rows() {
        let input = "A\tB\n1";
        let expected = "\
| A   | B |
|-----|---|
| 1   |   |";
        assert_eq!(tsv2md(input), expected);
    }

    #[test]
    fn ignores_empty_lines() {
        let input = "A\tB\n\n1\t2\n";
        let expected = "\
| A   | B   |
|-----|-----|
| 1   | 2   |";
        assert_eq!(tsv2md(input), expected);
    }

    #[test]
    fn escapes_pipes() {
        let input = "A\nx|y";
        let expected = "\
| A   |
|-----|
| x\\|y |";
        assert_eq!(tsv2md(input), expected);
    }

    #[test]
    fn empty_input_is_empty_output() {
        assert_eq!(tsv2md(""), "");
        assert_eq!(tsv2md("\n\n"), "");
    }
}
