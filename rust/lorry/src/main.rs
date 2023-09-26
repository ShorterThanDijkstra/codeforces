mod knapsack;
use std::{
    cmp,
    io::{stdin, stdout, BufRead, BufReader, BufWriter, Write},
};

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
    let (max, choices) = search_cache(&vehicles, cap);
    let mut writer = BufWriter::new(stdout());
    writeln!(&mut writer, "{}", max).unwrap();
    for choice in choices {
        writeln!(&mut writer, "{}", choice + 1).unwrap();
    }
}
fn search_cache(items: &Vec<(usize, usize)>, cap: usize) -> (usize, Vec<usize>) {
    let mut cache: Vec<Vec<usize>> = Vec::new();
    for _ in items {
        let mut row = Vec::new();
        for _ in 0..=cap {
            row.push(0);
        }
        cache.push(row);
    }
    for (i, &(space, value)) in items.iter().enumerate() {
        if i == 0 {
            for j in 0..=cap as usize {
                if j >= space {
                    cache[0][j] = value;
                }
            }
            continue;
        }
        for j in 1..=cap as usize {
            let no_choose = cache[i - 1][j];
            if j < space as usize {
                cache[i][j] = no_choose;
            } else {
                let choose = cache[i - 1][j - space] + value;
                cache[i][j] = cmp::max(no_choose, choose);
            }
        }
    }
    let mut choices = Vec::new();
    let mut j = cap as usize;
    for i in (0..items.len()).rev() {
        if i == 0 && j != 0 {
            assert!(items[0].0 as usize == j);
            choices.push(0);
            continue;
        }
        if i != 0 && cache[i][j] > cache[i - 1][j] {
            choices.push(i);
            j = j - (items[i].0 as usize)
        }
    }
    (cache[items.len() - 1][cap as usize], choices)
}
