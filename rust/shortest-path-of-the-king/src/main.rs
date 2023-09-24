use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Stdout, Write};
fn parse_coord(input: String) -> (u32, u32) {
    let mut chars = input.chars();
    let col = chars.nth(0).unwrap() as u32 - ('a' as u32);
    let row = chars.nth(0).unwrap() as u32 - ('1' as u32);
    (row, col)
}
fn println_res(
    steps1: u32,
    direc1_1: &str,
    direc1_2: &str,

    steps2: u32,
    direc2: &str,
    writer: &mut BufWriter<Stdout>,
) {
    writeln!(writer, "{}", steps1 + steps2).unwrap();
    for _ in 0..steps1 {
        writeln!(writer, "{}{}", direc1_1, direc1_2).unwrap();
    }
    for _ in 0..steps2 {
        writeln!(writer, "{}", direc2).unwrap();
    }
}
fn main() {
    let mut reader = BufReader::new(stdin());
    let mut start = String::new();
    reader.read_line(&mut start).unwrap();
    let mut end = String::new();
    reader.read_line(&mut end).unwrap();

    let (mut start_row, mut start_col) = parse_coord(start);
    let (end_row, end_col) = parse_coord(end);
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
    let direc_1_right = start_col < end_col;
    let direc_1_up = start_row < end_row;
    let mut steps1 = 0_u32;
    while start_row != end_row && start_col != end_col {
        start_row = if direc_1_up {
            start_row + 1
        } else {
            start_row - 1
        };
        start_col = if direc_1_right {
            start_col + 1
        } else {
            start_col - 1
        };
        steps1 += 1;
    }
    let direc1_1 = if direc_1_right { "R" } else { "L" };
    let direc1_2 = if direc_1_up { "U" } else { "D" };

    if start_row == end_row {
        let steps2 = start_col.abs_diff(end_col);
        let direc2 = if start_col < end_col { "R" } else { "L" };
        println_res(steps1, direc1_1, direc1_2, steps2, direc2, &mut writer)
    } else if start_col == end_col {
        let steps2 = start_row.abs_diff(end_row);
        let direc2 = if start_row < end_row { "U" } else { "D" };
        println_res(steps1, direc1_1, direc1_2, steps2, direc2, &mut writer)
    }
   
}
