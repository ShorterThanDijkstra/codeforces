use rand::Rng;
use std::{
    cmp,
    time::{SystemTime, UNIX_EPOCH},
};

pub fn search_brute_force(items: &Vec<(u32, u32)>, index: usize, cap: u32) -> (u32, Vec<usize>) {
    if index == items.len() {
        return (0, Vec::new());
    }
    if cap <= 0 {
        return (0, Vec::new());
    }
    let (curr_space, curr_value) = items[index];
    let (search1, vec1) = search_brute_force(items, index + 1, cap);
    if cap < curr_space {
        return (search1, vec1);
    }
    let (search2, mut vec2) = search_brute_force(items, index + 1, cap - curr_space);

    if search1 >= search2 + curr_value {
        return (search1, vec1);
    } else {
        vec2.push(index);
        return (search2 + curr_value, vec2);
    }
}

pub fn search_brute_force_no_choices(items: &Vec<(u32, u32)>, index: usize, cap: u32) -> u32 {
    if index == items.len() {
        return 0;
    }
    if cap <= 0 {
        return 0;
    }
    let (curr_space, curr_value) = items[index];
    let search1 = search_brute_force_no_choices(items, index + 1, cap);
    if cap < curr_space {
        return search1;
    }
    let search2 = search_brute_force_no_choices(items, index + 1, cap - curr_space);

    if search1 >= search2 + curr_value {
        return search1;
    } else {
        return search2 + curr_value;
    }
}
pub fn search_cache_no_choices(items: &Vec<(u32, u32)>, cap: u32) -> u32 {
    let mut cache: Vec<Vec<u32>> = Vec::new();
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
                if j >= space as usize {
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
                let choose = cache[i - 1][j - space as usize] + value;
                cache[i][j] = cmp::max(no_choose, choose);
            }
        }
    }
    cache[items.len() - 1][cap as usize]
}

pub fn search_cache(items: &Vec<(u32, u32)>, cap: u32) -> (u32, Vec<usize>) {
    let mut cache: Vec<Vec<u32>> = Vec::new();
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
                if j >= space as usize {
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
                let choose = cache[i - 1][j - space as usize] + value;
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

pub fn search_cache_one_dimension(items: &Vec<(u32, u32)>, cap: u32) -> (u32, Vec<usize>) {
    let mut cache: Vec<(u32, Vec<usize>)> = Vec::new();
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
        for col in (1..=(cap as usize)).rev() {
            if col >= space as usize {
                let (no_choose, _) = cache[col];
                let (choose,  mut choices) = cache[col - space as usize].clone();
                if no_choose < choose + value {
                    choices.push(row);
                    cache[col] = (choose + value, choices);
                }
            }
        }
    }
    cache[cache.len() - 1].clone()
}

#[test]
pub fn timer_brute_force() {
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
    for _ in 0..900 {
        search_brute_force_no_choices(&items, 0, cap);
    }
    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    println!("duration:{}", end.as_millis() - start.as_millis());
}

#[test]
pub fn timer_cache() {
    let cap = 30;
    let mut items = Vec::new();
    let mut rng = rand::thread_rng();
    for _ in 0..1000 {
        let space = rng.gen::<u32>() % cap;
        let value = rng.gen::<u32>() % 10000;
        items.push((space, value))
    }
    println!("initialized");
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    for _ in 0..900 {
        search_cache(&items, cap);
    }
    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    println!("duration:{}", end.as_millis() - start.as_millis());
}

#[test]
pub fn timer_cache_one_dimension() {
    let cap = 30;
    let mut items = Vec::new();
    let mut rng = rand::thread_rng();
    for _ in 0..1000 {
        let space = rng.gen::<u32>() % cap;
        let value = rng.gen::<u32>() % 10000;
        items.push((space, value))
    }
    println!("initialized");
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    for _ in 0..900 {
        search_cache_one_dimension(&items, cap);
    }
    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    println!("duration:{}", end.as_millis() - start.as_millis());
}

#[test]
pub fn test_cache_one_dimension() {
    let items = vec![(1, 15), (2, 20), (4, 36)];
    let cap = 4;
    println!("{:?}", search_cache_one_dimension(&items, cap));
}
