use std::io;
struct Coord {
    row: u64,
    col: u64,
}
fn lex_rxcy(input: &String) -> Option<Coord> {
    let mut row = 0;
    let mut col = 0;

    for (i, c) in input.char_indices() {
        if i == 0 && c != 'R' {
            return None;
        }
        if row == -1 {

        }
    }
    None
}
fn print_rxcy(coord:&Coord) {
}
fn count() -> u64 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}
fn main() {
    let cout = count();
    for i in 0..cout {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        if let Some(coord) = lex_rxcy(&input) {
            print_rxcy(&coord);
        }else if let Some(coord =  {
            
        }
    }
}
