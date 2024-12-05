use std::io::{self, Read};

enum State {
    Start,
    Number1,
    Number2,
}

#[allow(dead_code)]
fn part1(lines: Vec<&str>) {
    let mut current_state = State::Start;
    let mut total = 0;
    
    for line in lines {
        let mut pc = 0;
        let mut number1 = 0;
        let mut number2 = 0;

        while pc < line.len() {
            match current_state {
                State::Start => {
                    if line[pc..].starts_with("mul(") {
                        current_state = State::Number1;
                        number1 = 0;
                        pc += 4;
                    } else {
                        pc += 1;
                    }
                },

                State::Number1 => {
                    if let Ok(digit) = line[pc..pc+1].parse::<u32>() {
                        number1 = number1 * 10 + digit;
                        pc += 1;
                    } else if line[pc..pc+1].starts_with(",") {
                        current_state = State::Number2;
                        number2 = 0;
                        pc += 1;
                    } else { // invalid input, retry character (might be mut(...))
                        current_state = State::Start;
                    }
                },

                State::Number2 => {
                    if let Ok(digit) = line[pc..pc+1].parse::<u32>() {
                        number2 = number2 * 10 + digit;
                        pc += 1;
                    } else if line[pc..pc+1].starts_with(")") {
                        total += number1 * number2;
                        pc += 1;
                        current_state = State::Start;
                    } else { // invalid input, retry character (might be mut(...))
                        current_state = State::Start;
                    }
                },
            }
        }
    }

    println!("{}", total);
}

fn part2(lines: Vec<&str>) {
    let mut current_state = State::Start;
    let mut mul_enabled = true;
    let mut total = 0;
    
    for line in lines {
        let mut pc = 0;
        let mut number1 = 0;
        let mut number2 = 0;

        while pc < line.len() {
            match current_state {
                State::Start => {
                    if line[pc..].starts_with("mul(") {
                        current_state = State::Number1;
                        number1 = 0;
                        pc += 4;
                    } else if line[pc..].starts_with("do()") {
                        mul_enabled = true;
                        pc += 4;
                    } else if line[pc..].starts_with("don't()") {
                        mul_enabled = false;
                        pc += 7;
                    } else {
                        pc += 1;
                    }
                },

                State::Number1 => {
                    if let Ok(digit) = line[pc..pc+1].parse::<u32>() {
                        number1 = number1 * 10 + digit;
                        pc += 1;
                    } else if line[pc..pc+1].starts_with(",") {
                        current_state = State::Number2;
                        number2 = 0;
                        pc += 1;
                    } else { // invalid input, retry character (might be mut(...))
                        current_state = State::Start;
                    }
                },

                State::Number2 => {
                    if let Ok(digit) = line[pc..pc+1].parse::<u32>() {
                        number2 = number2 * 10 + digit;
                        pc += 1;
                    } else if line[pc..pc+1].starts_with(")") {
                        if mul_enabled {
                            total += number1 * number2;
                        }
                        pc += 1;
                        current_state = State::Start;
                    } else { // invalid input, retry character (might be mut(...))
                        current_state = State::Start;
                    }
                },
            }
        }
    }

    println!("{}", total);
}


fn main() {
    let mut input = String::new();

    io::stdin().read_to_string(&mut input).unwrap();
    let lines = input.lines().collect::<Vec<_>>();

    // part1(lines);
    part2(lines);
}
