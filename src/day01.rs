// https://adventofcode.com/2022/day/1

pub fn solve(data: &[String]) -> (i32, i32) {
    // Count the number of lines that have a higher value than the previous line.
    let s1 = data
        .iter()
        .map(|x| x.parse::<i32>().unwrap())
        .fold(
            (0, 0),
            |(acc, prev), x| {
                if x > prev {
                    (acc + 1, x)
                } else {
                    (acc, x)
                }
            },
        )
        .0
        - 1;

    // Count how many sums of 3 numbers are larger than the previous sum of 3 numbers, using a sliding window.
    let s2 = data
        .iter()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
        .windows(3)
        .fold((0, 0), |(acc, prev), x| {
            if x.iter().sum::<i32>() > prev {
                (acc + 1, x.iter().sum::<i32>())
            } else {
                (acc, x.iter().sum::<i32>())
            }
        })
        .0
        - 1;

    (s1, s2)
}

#[allow(dead_code)]
pub fn run() {
    let res = solve(&crate::library::read_file("data/day01.txt"));
    println!("Day 01:\nStar 1: {}\nStar 2: {}\n", res.0, res.1);
}

#[allow(dead_code)]
pub fn benchmark(c: &mut criterion::Criterion) {
    let data = crate::library::read_file("data/day01.txt");
    c.bench_function("Day 01", |b| b.iter(|| solve(&data)));
}
