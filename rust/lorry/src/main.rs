use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut reader = BufReader::new(stdin());
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let mut split = input.split_whitespace();
    let count = split.next().unwrap().parse::<usize>().unwrap();
    let cap = split.next().unwrap().parse::<usize>().unwrap();
    let mut vehicles = Vec::new();
    for _ in 0..count {
        input.clear();
        reader.read_line(&mut input).unwrap();
        split = input.split_whitespace();
        let space = split.next().unwrap().parse::<usize>().unwrap();
        let value = split.next().unwrap().parse::<usize>().unwrap();
        vehicles.push((space, value))
    }
    let (max, choices) = search_cache_one_dimension(&vehicles, cap);
    let mut writer = BufWriter::new(stdout());
    writeln!(&mut writer, "{}", max).unwrap();
    for choice in choices {
        writeln!(&mut writer, "{}", choice + 1).unwrap();
    }
}
pub fn search_cache_one_dimension(items: &Vec<(usize, usize)>, cap: usize) -> (usize, Vec<usize>) {
    let mut cache: Vec<(usize, Vec<usize>)> = Vec::new();
    let (space_fst, value_fst) = items[0];
    for col in 0..=cap {
        if col >= space_fst {
            cache.push((value_fst, vec![0]));
        } else {
            cache.push((0, Vec::new()));
        }
    }
    for (row, &(space, value)) in items.iter().enumerate() {
        if row == 0 {
            continue;
        }
        for col in (1..=cap).rev() {
            if col >= space as usize {
                let (no_choose, _) = cache[col];
                let (choose, mut choices) = cache[col - space].clone();
                if no_choose < choose + value {
                    choices.push(row);
                    cache[col] = (choose + value, choices);
                }
            }
        }
    }
    cache[cache.len() - 1].clone()
}
