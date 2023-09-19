use std::io;
struct Coord {
    row: u64,
    col: u64,
}
fn lex_num(v: &Vec<char>, index: usize) -> (u64, usize) {
    let mut res = 0_u64;
    let mut i = index;
    while let Some(&c) = v.get(i) {
        if c.is_digit(10) {
            res = res * 10 + (c.to_digit(10).unwrap()) as u64;
            i = i + 1;
        } else {
            return (res, i);
        }
    }
    (res, i)
}

fn lex_rxcy(v: &Vec<char>) -> Option<Coord> {
    if v.get(0) != Some(&'R') {
        return None;
    }
    let (row, row_end) = lex_num(v, 1);
    if row_end == 1 {
        return None;
    }

    if v.get(row_end) != Some(&'C') {
        return None;
    }
    let (col, col_end) = lex_num(v, row_end + 1);
    // if col_end != v.len() - 1 {
    // last char is \n
    // return None;
    // }
    Some(Coord { row: row, col: col })
}
fn lex_alphabet(v: &Vec<char>, index: usize) -> (u64, usize) {
    let mut res = 0;
    let mut i = index;
    while let Some(&c) = v.get(i) {
        if c.is_ascii_alphabetic() && c.is_uppercase() {
            i += 1;
            res = res * 26 + (c as u8 - ('A' as u8) + 1) as u64;
        } else {
            return (res, i);
        }
    }
    (res, i)
}
fn print_rxcy(coord: &Coord)  {
    println!("R{}C{}", coord.row, coord.col)
}
fn lex_abxy(v: &Vec<char>) -> Option<Coord> {
    let (col, col_end) = lex_alphabet(v, 0);
    let (row, row_end) = lex_num(v, col_end);
    // if row_end != v.len() - 1 {
    //     return None;
    // }
    Some(Coord { row: row, col: col })
}
fn num_ab(num: u64) -> String {
    let mut n = num;
    let mut v: Vec<char> = Vec::new();
    while n > 0 {
        let d = n % 26;
        if d == 0 {
            v.push('Z');
            n = (n - 1) / 26;
        } else {
            v.push((d + 'A' as u64 - 1) as u8 as char);
            n = n / 26;
        }
    }
    v.iter().rev().collect()
}

fn print_abxy(coord: &Coord) {
    println!("{}{}", num_ab(coord.col), coord.row)
}

fn count() -> u64 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}
fn main() {
    let mut count = count();
    while count > 0 {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let chars = input.chars().collect::<Vec<char>>();
        if let Some(coord) = lex_rxcy(&chars) {
             print_abxy(&coord);
        } else if let Some(coord) = lex_abxy(&chars) {
           print_rxcy(&coord);
        } else {
            println!("{}", input);
            panic!("unexpected input")
        }
        count -= 1;
    }
    // R228C494
    // S@228
    // RZ228

    // RC593
}
