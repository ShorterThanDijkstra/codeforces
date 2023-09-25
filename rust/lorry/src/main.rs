use std::{
    time::{SystemTime, UNIX_EPOCH},
};
use rand::Rng;


fn search(items: &Vec<(u32, u32)>, index: usize, cap: u32) -> (u32, Vec<usize>) {
    if index == items.len() {
        return (0, Vec::new());
    }
    if cap <= 0 {
        return (0, Vec::new());
    }
    let (curr_space, curr_value) = items[index];
    let (search1, vec1) = search(items, index + 1, cap);
    if cap < curr_space {
        return (search1, vec1);
    }
    let (search2, mut vec2) = search(items, index + 1, cap - curr_space);

    if search1 >= search2 + curr_value {
        return (search1, vec1);
    } else {
        vec2.push(index);
        return (search2 + curr_value, vec2);
    }
}
fn timer_1() {
    let cap = 30;
    let mut items = Vec::new();
    let mut rng = rand::thread_rng();
    for _ in 0..50 {
        let space = rng.gen::<u32>() % cap;
        let value = rng.gen::<u32>() % 10000;
        items.push((space, value))
    }
    println!("initialized");
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    for _ in 0..100 {
        search(&items, 0, cap);
    }
    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    println!("duration:{}", end.as_millis() - start.as_millis());
}
fn main() {
    timer_1()
}
