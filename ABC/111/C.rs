use std::collections::HashMap;

fn read_usize() -> usize {
    let mut input = String::new();
    let _ = std::io::stdin().read_line(&mut input);
    input.trim().parse().unwrap()
}

fn read_usizevec() -> Vec<usize> {
    let mut input = String::new();
    let _ = std::io::stdin().read_line(&mut input);
    input.trim().split_whitespace().map(|x| x.parse().unwrap()).collect()
}

fn map_update(map: &mut HashMap<usize, usize>, item: &usize) -> () {
    if map.contains_key(item) {
        if let Some(x) = map.get_mut(item) {
            *x += 1;
        }
    } else {
        map.insert(*item, 1usize);
    }
}

fn count_1st_2nd(map: &HashMap<usize, usize>) -> (usize, usize) {
    let (mut num_1st, mut num_2nd) = (0, 0);
    for value in map.values() {
        if value >= &num_1st {
            num_2nd = num_1st;
            num_1st = *value;
        }
    }
    (num_1st, num_2nd)
}

fn main() {
    let n = read_usize();
    let v = read_usizevec();
    let mut map_evens: HashMap<usize, usize> = HashMap::new();
    let mut map_odds: HashMap<usize, usize> = HashMap::new();
    for (i, item) in v.iter().enumerate() {
        if i % 2 == 0 {
            map_update(&mut map_evens, item);
        } else {
            map_update(&mut map_odds, item);
        }
    }

    let (even_1st, even_2nd) = count_1st_2nd(&map_evens);
    let (odd_1st, odd_2nd) = count_1st_2nd(&map_odds);

    println!("{:?}", map_evens);
    println!("{:?}", map_odds);
}
