// https://adventofcode.com/2022/day/02

pub fn solve(data: &[(String, String)]) -> (i32, i32) {
    // Have a depth and distance variable, distance is affected by "forward" and "backward" commands.
    // Depth is affected by "up" and "down" commands. Return product of distance and depth.
    let p1 = data.iter().fold((0, 0), |(d, p), (cmd, val)| {
        let val = val.parse::<i32>().unwrap();
        match cmd.as_str() {
            "forward" => (d + val, p),
            "up" => (d, p - val),
            "down" => (d, p + val),
            _ => (d, p),
        }
    });

    let p2 = data.iter().fold((0, 0, 0), |(d, p, a), (cmd, val)| {
        let val = val.parse::<i32>().unwrap();
        match cmd.as_str() {
            "forward" => (d + val, p + (a * val), a),
            "up" => (d, p, a - val),
            "down" => (d, p, a + val),
            _ => (d, p, a),
        }
    });

    (p1.0 * p1.1, p2.0 * p2.1)
}

// Function to convert a vector of string into a vector of tuples of strings using split_string().
fn parse(data: &[String]) -> Vec<(String, String)> {
    let mut parsed: Vec<(String, String)> = vec![];
    for line in data {
        parsed.push(crate::library::split_string(line, " "));
    }
    parsed
}

#[allow(dead_code)]
pub fn run() {
    let res = solve(&parse(&crate::library::read_file("data/day02.txt")));
    println!("Day 02:\nStar 1: {}\nStar 2: {}\n", res.0, res.1);
}

#[allow(dead_code)]
pub fn benchmark(c: &mut criterion::Criterion) {
    let data = parse(&crate::library::read_file("data/day02.txt"));
    c.bench_function("Day 02", |b| b.iter(|| solve(&data)));
}
