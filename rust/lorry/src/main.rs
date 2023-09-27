use std::{
    collections::BinaryHeap,
    io::{stdin, stdout, BufRead, BufReader, BufWriter, Write},
};

fn main() {
    let mut reader = BufReader::new(stdin());
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let mut split = input.split_whitespace();
    let count = split.next().unwrap().parse::<usize>().unwrap();
    let cap = split.next().unwrap().parse::<usize>().unwrap();
    let mut vehicles1 = BinaryHeap::new();
    let mut vehicles2 = BinaryHeap::new();
    for i in 1..=count {
        input.clear();
        reader.read_line(&mut input).unwrap();
        split = input.split_whitespace();
        let space = split.next().unwrap().parse::<usize>().unwrap();
        let value = split.next().unwrap().parse::<usize>().unwrap();
        if space == 1 {
            vehicles1.push((value, i))
        } else {
            // assert!(space == 2);
            vehicles2.push((value, i))
        }
    }

    let (max, choices) = search_greedy(&mut vehicles1, &mut vehicles2, cap);
    let mut writer = BufWriter::new(stdout());
    writeln!(&mut writer, "{}", max).unwrap();
    for choice in choices {
        writeln!(&mut writer, "{}", choice).unwrap();
    }
}
fn search_greedy(
    vehicles1: &mut BinaryHeap<(usize, usize)>,
    vehicles2: &mut BinaryHeap<(usize, usize)>,
    cap: usize,
) -> (u128, Vec<usize>) {
    let mut cap = cap;
    let mut max: u128 = 0;
    let mut choices = Vec::new();
    let mut pre_vehi1 = None;
    while cap > 0 {
        if !vehicles1.is_empty() && !vehicles2.is_empty() {
            let &(value1, index1) = vehicles1.peek().unwrap();
            let &(value2, index2) = vehicles2.peek().unwrap();

            if value2 as f64 / 2.0 > value1 as f64 {
                vehicles2.pop();
                if cap == 1 {
                    if let Some((pre_choice1, pre_value1)) = pre_vehi1 {
                        if value2 > pre_value1 {
                            choices[pre_choice1] = index2;
                            max = max - pre_value1 as u128 + value2 as u128;
                            cap -= 1;
                        }
                    }
                } else {
                    max += value2 as u128;
                    cap -= 2;
                    choices.push(index2);
                }
            } else {
                pre_vehi1 = Some((choices.len(), value1));
                vehicles1.pop();
                max += value1 as u128;
                cap -= 1;
                choices.push(index1);
            }
        } else if vehicles1.is_empty() && !vehicles2.is_empty() {
            let (value2, index2) = vehicles2.pop().unwrap();
            if cap == 1 {
                if let Some((pre_choice1, pre_value1)) = pre_vehi1 {
                    if value2 > pre_value1 {
                        choices[pre_choice1] = index2;
                        max = max - pre_value1 as u128 + value2 as u128;
                        cap -= 1;
                    }
                }
            } else {
                max += value2 as u128;
                cap -= 2;
                choices.push(index2);
            }
        } else if vehicles2.is_empty() && !vehicles1.is_empty() {
            let (value1, index1) = vehicles1.pop().unwrap();
            pre_vehi1 = Some((choices.len(), value1));
            max += value1 as u128;
            cap -= 1;
            choices.push(index1);
        } else {
            break;
        }
    }
    (max, choices)
}

#[test]
pub fn test() {
    use rand::Rng;
    use std::time::SystemTime;
    use std::time::UNIX_EPOCH;

    let mut rng = rand::thread_rng();
   
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    for _ in 0..10 {
        let mut vehicles1 = BinaryHeap::new();
        let mut vehicles2 = BinaryHeap::new();
        for i in 1..=10000 {
            let space = rng.gen::<usize>() % 2 + 1;
            let value = rng.gen::<usize>() % 100;
            if space == 1 {
                vehicles1.push((value, i))
            } else {
                vehicles2.push((value, i))
            }
        }
        let cap = rng.gen::<usize>() % 10000 + 10000;
        println!("cap:{}",cap);
        let (max, _) = search_greedy(&mut vehicles1, &mut vehicles2, cap);
        println!("max:{}", max);
        println!()
    }
    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    println!("duration:{}", end.as_millis() - start.as_millis());
}
