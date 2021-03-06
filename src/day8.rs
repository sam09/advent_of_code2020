extern crate regex;
use std::str::FromStr;
use std::collections::HashSet;
use std::convert::TryInto;
use crate::utils::read_input_string;

#[derive(Debug)]
enum InstType {
    NOP,
    JMP,
    ACC,
}

impl FromStr for InstType {

    type Err = ();

    fn from_str(input: &str) -> Result<InstType, Self::Err> {
        match input {
            "nop"  => Ok(InstType::NOP),
            "jmp"  => Ok(InstType::JMP),
            "acc"  => Ok(InstType::ACC),
            _      => Err(()),
        }
    }
}

#[derive(Debug)]
struct Inst {
    inst: InstType,
    value: i32
}

fn create_instruction_set(a: Vec<String>) -> Vec<Inst> {
    let mut inst_set = Vec::new();

    for i in &a {
        let temp = i.split(" ").collect::<Vec<&str>>();
        inst_set.push(Inst{ inst: InstType::from_str(temp[0]).unwrap(), value:temp[1].parse::<i32>().unwrap()});
    }

    inst_set
}


fn detect_for_infinite_loop(inst_set: &Vec<Inst>, p_counter: i32, n: i32, mut visited: HashSet<i32>, prev_val:i32) -> (bool, i32) {
    let mut acc = prev_val;
    let mut pc = p_counter;
    let mut flag = false;

    while pc < n {
        let index = pc as usize;
        let inst = &inst_set[index];
        if visited.contains(&pc) {
            flag = true;
            break
        }
        visited.insert(pc);
        match &inst.inst {
            InstType::NOP => pc += 1,
            InstType::JMP => pc += inst.value,
            InstType::ACC => {
                pc += 1;
                acc += inst.value;
            }
        }
    }

    (flag, acc)
}

fn solve_pt1(a: Vec<String>) -> i32 {
    let inst_set = create_instruction_set(a);
    let visited = HashSet::new();
    let n = inst_set.len().try_into().unwrap();

    detect_for_infinite_loop(&inst_set, 0, n, visited, 0).1
}

fn solve_pt2(a: Vec<String>) -> i32 {
    let inst_set = create_instruction_set(a);
    let mut visited = HashSet::new();
    let n = inst_set.len().try_into().unwrap();
    let mut pc = 0;
    let mut acc = 0;

    while pc < n {
        let index = pc as usize;
        let inst = &inst_set[index];
        visited.insert(pc);
        match &inst.inst {
            InstType::ACC => {
                pc += 1;
                acc += inst.value;
            },
            InstType::NOP => {
                if pc + inst.value >= 0 && pc + inst.value < n {
                    let (flag, acc_tp) = detect_for_infinite_loop(&inst_set, pc + inst.value, n, visited.clone(), acc);
                    if flag == false {
                        return acc_tp;
                    } else {
                        pc += 1;
                    }
                } else {
                    pc += 1;
                }

            },
            InstType::JMP => {
                let (flag, acc_tp) = detect_for_infinite_loop(&inst_set, pc + 1, n, visited.clone(), acc);
                if flag == false {
                    return acc_tp;
                } else {
                    pc += inst.value;
                }
            },
        }
    }
    -1
}

pub fn run(part: char) {
    let v = read_input_string("data/day8.txt");
    match v {
        Ok(values) => {
            let ans = if part == 'a' { solve_pt1(values) } else { solve_pt2(values) };
            println!("{}", ans)
        }
        _ => println!("error occurred parsing input")
    };
}