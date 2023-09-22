use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};
fn get_coord(input: String) -> (u32, u32) {
    let mut chars = input.chars();
    let col = chars.next().unwrap().to_digit(10).unwrap() - ('a' as u32);
    let row = chars.next().unwrap().to_digit(10).unwrap() - ('1' as u32);
    (row, col)
}
fn main() {
    let mut reader = BufReader::new(stdin());
    let mut start = String::new();
    reader.read_line(&mut start).unwrap();
    let mut end = String::new();
    reader.read_line(&mut end).unwrap();

    let (start_row, start_col) = get_coord(start);
    let (end_row, end_col) = get_coord(end);
    let mut writer = BufWriter::new(stdout());

    if start_row == end_row {
        let mut steps = start_col.abs_diff(end_col);
        let vert_direc = if start_col < end_col { 'R' } else { 'L' };
        writeln!(&mut writer, "{}", steps).unwrap();
        while steps > 0 {
            writeln!(&mut writer, "{}", vert_direc).unwrap();
            steps -= 1;
        }
        return;
    }
    if start_col == end_col {
        let mut steps = start_row.abs_diff(end_row);
        let hori_direc = if start_row < end_row { 'U' } else { 'D' };
        writeln!(&mut writer, "{}", steps).unwrap();
        while steps > 0 {
            writeln!(&mut writer, "{}", hori_direc).unwrap();
            steps -= 1;
        }
        return;
    }
    if start_col < end_col
}
