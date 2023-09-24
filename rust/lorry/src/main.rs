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
fn main() {
    let cap = 4;
    let items = vec![(1, 15), (3, 20), (4, 30)];
    let max = search(&items, 0, cap);
    println!("{:?}", max)
}
