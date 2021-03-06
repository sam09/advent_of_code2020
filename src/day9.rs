use std::collections::HashMap;
use crate::utils::read_input_int64;

fn is_valid(a: &i64, set: &HashMap<i64, i64>) -> bool {
    for (i, _) in set {
        let tp = a-*i;
        if set.contains_key(&tp) && ((tp != *i && set[&tp] > 0) || (tp == *i && set[&tp] > 1) ) {
            return true
        }
        
    }
    false
}

fn find_first(a: &Vec<i64>) -> i64 {
    let mut set = HashMap::new();
    let index = 25;
    for i in 0..a.len() {
        if i >= index {
            if !is_valid(&a[i], &set) {
                return a[i]
            }
            let v = set[&a[i-index]];
            set.remove(&a[i-index]);
            set.insert(a[i-index], v-1);
            if set[&a[i-index]] <= 0 {
                set.remove(&a[i-index]);
            }
        }
        if set.contains_key(&a[i]) {
            let v = set[&a[i]];
            set.remove(&a[i]);
            set.insert(a[i], v+1);
        } else {
            set.insert(a[i], 1);
        }
    }
    -1
}

fn solve_pt2(a: Vec<i64>) -> i64 {
    let smallest = find_first(&a); //393911906
    let mut sum = 0;
    let mut last_index = 0;
    for i in 0..a.len() {
        sum += a[i];
        if sum == smallest {
            let mut min = a[last_index];
            let mut max = a[last_index];
            for j in last_index+1..i+1 {
                if a[j] > max {
                    max = a[j];
                }
                if a[j] < min {
                    min = a[j]
                }
            }
            return max + min;
        } else if sum > smallest {
            while sum > smallest && last_index <= i {
                sum -= a[last_index];
                last_index+=1;
            }
            if sum == smallest  && last_index != i {
                let mut min = a[last_index];
                let mut max = a[last_index];
                for j in last_index+1..i+1 {
                    if a[j] > max {
                        max = a[j];
                    }
                    if a[j] < min {
                        min = a[j]
                    }
                }
                return max + min;
            }
        }
    }
    -1
}

pub fn run(part: char) {
    let v = read_input_int64("data/day9.txt");
    match v {
        Ok(values) => {
            let ans = if part == 'a' { find_first(&values) } else { solve_pt2(values) };
            println!("{}", ans);
        }
        _ => println!("error occurred parsing input")
    };
}