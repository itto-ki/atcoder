fn read_i32_pair() -> (i32, i32) {
    let mut input = String::new();
    let _ = std::io::stdin().read_line(&mut input);
    let v: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    (v[0], v[1])
}

fn tsurukame(people: i32, legs: i32) -> Option<(i32, i32)> {
    let adult = (people * 4 - legs) / 2;
    let baby = people - adult;
    if adult * 2 + baby * 4 == legs && adult >= 0 && adult <= people {
        Some((adult, baby))
    } else {
        None
    }
}


fn main() {
    let (n, m) = read_i32_pair();
    // 老人が2人以上の場合は大人1人と子供1人で代用できる
    // よって、老人は0もしくは1人の場合のみ考える
    for old in 0..2 {
        let adult_and_baby = n - old;
        let adult_and_baby_legs = m - old * 3;
        match tsurukame(adult_and_baby, adult_and_baby_legs) {
            Some((adult, baby)) => {
                println!("{} {} {}", adult, old, baby);
                return;
            },
            None => {},
        }
    }
    println!("-1 -1 -1");
}
