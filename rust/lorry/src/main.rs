use std::io::{stdin, BufRead, BufReader};

fn main() {
    let mut reader = BufReader::new(stdin());
    let mut input = String::new();

    reader.read_line(&mut input).unwrap();
    let mut spilt = input.split_whitespace();
    let count = spilt.next().unwrap().parse::<usize>().unwrap();
    let cap = spilt.next().unwrap().parse::<usize>().unwrap();
    let mut items = Vec::new();
    for _ in 0..count {
        input.clear();
         reader.read_line(&mut input);
         let space ;
    }
}
