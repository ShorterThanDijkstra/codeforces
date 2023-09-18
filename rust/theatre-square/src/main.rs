use std::io;

fn solution(n: u64, m: u64, a: u64) -> u64 {
    let n_hat = (n as f64 / a as f64).ceil() as u64;
    let m_hat = (m as f64 / a as f64).ceil() as u64;
    n_hat * m_hat
}

fn main() {
    let mut input_text = String::new();
    io::stdin().read_line(&mut input_text).unwrap();
    let nums = input_text
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();
    let n = nums[0];
    let m = nums[1];
    let a = nums[2];
    println!("{}", solution(n, m, a))
}
