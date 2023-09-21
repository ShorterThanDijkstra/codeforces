use std::{
    collections::HashMap,
    io::{stdin, stdout, BufRead, BufReader, BufWriter, Write},
};
fn find_least(value: i32, seq: &Vec<(i32, usize)>) -> usize {
    let mut sum = 0;
    for (score, round) in seq {
        sum = *score + sum;
        if sum >= value {
            return *round;
        }
    }
    panic!()
}
fn main() {
    let mut reader = BufReader::new(stdin());
    let mut writer = BufWriter::new(stdout());

    let mut rounds = String::new();
    reader.read_line(&mut rounds).unwrap();
    let rounds = rounds.trim().parse::<usize>().unwrap();

    let mut scores: HashMap<String, i32> = HashMap::new();
    let mut scores_seq: HashMap<String, Vec<(i32, usize)>> = HashMap::new();
    let mut input = String::new();
    let mut round = 0;
    while round < rounds {
        reader.read_line(&mut input).unwrap();
        let v = input.split_whitespace().collect::<Vec<&str>>();
        let name = v[0];
        let score = v[1].parse::<i32>().unwrap();
        if let Some(seq) = scores_seq.get_mut(name) {
            seq.push((score, round));
        } else {
            let mut v = Vec::new();
            v.push((score, round));
            scores_seq.insert(name.to_string(), v);
        }
        if let Some(old_score) = scores.get(name) {
            scores.insert(name.to_string(), *old_score + score);
        } else {
            scores.insert(name.to_string(), score);
        }
        input.clear();
        round += 1;
    }
    // println!("{:?}", &scores);
    // println!("{:?}", &scores_seq);
    let max = scores.values().max().unwrap();
    let mut max_name = "";
    let mut least_round = rounds;
    for (name, score) in &scores {
        if score == max {
            let least_round_tmp = find_least(*max, scores_seq.get(name).unwrap());
            if least_round_tmp < least_round {
                least_round = least_round_tmp;
                max_name = name;
            }
        }
    }
    writeln!(&mut writer, "{}", max_name).unwrap();
}
