use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let sequences = parse(input);
    let mut x = 0;
    let mut y = 0;
    for sequence in &sequences {
        let o = part_1(sequence);
        let m = part_2(sequence);
        println!("{:?}: {m}", sequence);
        x += o;
        y += m;
    }
    println!("{x}, {y}");
}

fn part_1(input: &Vec<i64>) -> i64 {
    // the final output should be the interpolated next number in the vec
    // which is equal to input.iter().last() + the output of the n - 1 loop of the recursion
    let mut out: Vec<i64> = vec![];
    for i in 0..input.len() - 1 {
        out.push(input[i + 1] - input[i]);
    }
    if out.iter().all(|&n| n == 0) {
        return *input.iter().last().unwrap();
    }
    let r_val = input.iter().last().unwrap() + part_1(&out);
    r_val
}

fn part_2(input: &Vec<i64>) -> i64 {
    let mut out: Vec<i64> = vec![];
    for i in 0..input.len() - 1 {
        out.push(input[i + 1] - input[i]);
    }
    println!("{:?}", out);
    if out.iter().all(|&n| n == 0) {
        return input[0];
    }
    let r_val = input[0] - part_2(&out);
    println!("returning {r_val}");
    r_val
}

fn parse(input: String) -> Vec<Vec<i64>> {
    let mut output: Vec<Vec<i64>> = vec![];

    for line in input.lines() {
        let sequence: Vec<i64> = line
            .split_whitespace()
            .map(|num| num.parse().unwrap())
            .collect();
        output.push(sequence);
    }
    output
}
