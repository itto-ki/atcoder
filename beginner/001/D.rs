#[allow(unused_must_use)]
fn read_int() -> usize {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input);
    input.trim().parse::<usize>().unwrap()
}

#[allow(unused_must_use)]
fn read_int_pair() -> (i32, i32) {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input);
    let v: Vec<i32> = input
        .trim()
        .split('-')
        .map(|x| x.parse().unwrap())
        .collect();
    (v[0], v[1])
}

fn convert_timeint_to_int(timeint_range_list: &mut Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    let mut int_range_list: Vec<(i32, i32)> = Vec::with_capacity(timeint_range_list.len());
    for (i, &(start, end)) in timeint_range_list.iter().enumerate() {
        let convert = |time: i32| -> i32 { time / 100 * 60 + time % 100 };
        let floor_5min_interval = |x: i32| -> i32 { x - (x % 5) };
        let ceil_5min_interval = |x: i32| -> i32 { x + 5 - (x % 5) };
        let start = floor_5min_interval(convert(start));
        let end = ceil_5min_interval(convert(end));
        int_range_list.push((start, end));
    }
    int_range_list
}

fn main() {
    let number_of_line: usize = read_int();
    let mut timeint_range_list: Vec<(i32, i32)> = Vec::with_capacity(number_of_line);
    for _ in 0..number_of_line {
        timeint_range_list.push(read_int_pair());
    }
    println!("{:?}", timeint_range_list);
    println!("{:?}", convert_timeint_to_int(&mut timeint_range_list));
    let mut timeline = [0_i32; 2400 * 60];
    for item in convert_timeint_to_int(&mut timeint_range_list).iter() {
        for i in item.0..item.1 {
            timeline[i as usize] = 1;
        }
    }
}
