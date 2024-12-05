use std::collections::HashMap;
use std::io;

fn is_valid(ordering_map: &HashMap<i64,Vec<i64>>, update: &Vec<i64>) -> bool {
    for s in 0..update.len() - 1 {
        let current_page = update[s];
        for e in s + 1..update.len() {
            if ordering_map.get(&update[e]).is_some_and(|v| v.contains(&current_page)) {
                continue;
            }
            return false;
        }
    }
    true
}

#[allow(dead_code)]
fn part1(orderings: Vec<Vec<i64>>, updates: Vec<Vec<i64>>) {
    let mut result = 0;

    // build map from orderings
    let mut ordering_map = HashMap::new();
    for ordering in orderings {
        ordering_map.entry(ordering[1]).or_insert(Vec::new()).push(ordering[0]);
    }
    
    for update in updates {
        if is_valid(&ordering_map, &update) {
            result += update[update.len() / 2];
        }
    }

    println!("{}", result);
}

// algorithm:
// 
// let check = loop from the last element to the first
//   let i = loop from the first to the check - 1
//      if check comes before i, remove check and insert it before i and
//         go back to the `i` loop (don't increment check)
//   if loop completes without update, continue 'check loop
fn part2(orderings: Vec<Vec<i64>>, mut updates: Vec<Vec<i64>>) {
    let mut result = 0;

    // build map from orderings
    let mut ordering_map = HashMap::new();
    for ordering in orderings {
        ordering_map.entry(ordering[1]).or_insert(Vec::new()).push(ordering[0]);
    }

    for update in updates.iter_mut() {
        if !is_valid(&ordering_map, &update) {

            for check in (1..update.len()).rev() {
                'check_again: loop {
                    for i in 0..check {
                        // check if update[check] must be before update[i]
                        if ordering_map.get(&update[i]).is_some_and(|v| v.contains(&update[check])) {
                            // remove update[check] and place it before update[i]
                            let page = update.remove(check);
                            update.insert(i, page);
                            continue 'check_again;
                        }
                    }

                    break;
                }
            }

            // assert!(is_valid(&ordering_map, &update));
            result += update[update.len() / 2];
        }
    }

    println!("{}", result);
}
    
fn main() {
    let mut line = String::new();
    let mut orderings = Vec::new();
    let mut updates = Vec::new();

    // read orderings
    loop {
        io::stdin().read_line(&mut line).unwrap();
        let line = std::mem::take(&mut line);

        if line.trim().len() == 0 { break; }

        let ordering = line.split("|").map(|e| e.trim().parse::<i64>().unwrap()).collect::<Vec<_>>();
        orderings.push(ordering);
    }

    // read updates
    while io::stdin().read_line(&mut line).is_ok_and(|v| v != 0) {
        let line = std::mem::take(&mut line);

        let update = line.split(",").map(|e| e.trim().parse::<i64>().unwrap()).collect::<Vec<_>>();
        updates.push(update);
    }

    // part1(orderings, updates);
    part2(orderings, updates);
}
