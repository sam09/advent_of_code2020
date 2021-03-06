use std::collections::{HashMap, HashSet};
use crate::utils::read_input_string;

fn parse_rule(rule: &str) -> (i32, i32) {
    let v = rule.split("-").collect::<Vec<&str>>();
    (v[0].parse::<i32>().unwrap(), v[1].parse::<i32>().unwrap())
}

fn parse_arr(val: &str) -> Vec<i32> {
    val.split(",").collect::<Vec<&str>>().into_iter()
        .map(|f| f.parse::<i32>().unwrap()).collect()
}

fn invalid(ticket: &Vec<i32>, rules: &HashMap<String, Vec<(i32, i32)>> ) -> Vec<i32> {
    let mut v = Vec::new();

    for i in ticket {
        let mut valid = false;
        for (_, v) in rules {
            for (x, y) in v {
                if i >= x && i <= y {
                    valid = true;
                }
            }
        }
        if valid == false {
            v.push(*i);
        }
    }
    v
}

fn parse_input(a: Vec<String>) -> (HashMap<String, Vec<(i32, i32)>>, Vec<i32>, Vec<Vec<i32>>, i32) {
    let mut i = 0;
    let mut all_rules = HashMap::new();
    let mut own_ticket = Vec::new();
    let mut nearby_tickets = Vec::new();
    while i < a.len() {
        if a[i].is_empty() {
            break;
        }
        let temp = a[i].split(": ").collect::<Vec<&str>>();
        let rules = temp[1].split(" or ").collect::<Vec<&str>>();
        let temp_rules = vec![parse_rule(rules[0]), parse_rule(rules[1])];
        all_rules.insert(temp[0].to_string(), temp_rules);
        i+=1;
    }

    i += 2;
    while i < a.len() {
        if a[i].is_empty() {
            break;
        }
        own_ticket = parse_arr(&a[i]);
        i+=1;
    }

    i += 2;
    while i < a.len() {
        if a[i].is_empty() {
            break;
        }
        nearby_tickets.push(parse_arr(&a[i]));
        i+=1;
    }

    let mut new_tickets = Vec::new();
    let mut sum = 0;
    for nearby_ticket in nearby_tickets {
        let v = invalid(&nearby_ticket, &all_rules);
        if v.len() == 0 {
            new_tickets.push(nearby_ticket);
        }
        sum += v.into_iter().fold(0, |a,b| a+b);
    }

    (all_rules, own_ticket, new_tickets, sum)
}

fn is_valid(ticket: i32, rule: Vec<(i32, i32)>) -> bool {
    for (x,y) in rule {
        if ticket >= x && ticket <= y {
            return true;
        }
    }
    false
}

fn solve_pt2(a: Vec<String>) -> i64 {
    let (rules, own_ticket, new_tickets, _) = parse_input(a);

    let n = new_tickets.len();
    let m = own_ticket.len();
    let mut candidate_list = vec![HashSet::new(); m];
    
    
    for i in 0..m {
        for (key, rule) in &rules {
            let mut valid = true;
            for j in 0..n {
                if !is_valid(new_tickets[j][i], rule.clone()) {
                    valid = false;
                }    
            }
            if valid == true {
                candidate_list[i].insert(key);
            }
        }
    }

    let mut mapping = HashMap::new();

    loop {
        for v in 0..m {
            if candidate_list[v].len() == 1 && !mapping.contains_key(&v) {
                for key in &candidate_list[v] {
                    mapping.insert(v, key.clone());
                }

                for i in 0..candidate_list.len() {
                    candidate_list[i].remove(mapping[&v]);
                }
                break;
            }
        }
        
        if mapping.len() == m {
            break;
        }
    }
    
    let mut prod = 1;
    for (key, value) in mapping {
        match value.find("departure") {
            Some(_) => {
                prod *= own_ticket[key] as i64;
            },
            None => ()
        }
    }
    prod
}

fn solve_pt1(a: Vec<String>) -> i64 {
    let (_, _, _, sum) = parse_input(a);
    sum as i64
}

pub fn run(part: char) {
    let v = read_input_string("data/day16.txt");
    match v {
        Ok(values) =>{
            let ans = if part == 'a' { solve_pt1(values) } else { solve_pt2(values) };
            println!("{}", ans);
        }
        _ => println!("error occurred parsing input")
    };
}
