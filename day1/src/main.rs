use std::collections::HashMap;
use std::env;
use std::fs;
fn main() {
    let contents = fs::read_to_string("./input.txt").expect("Values");
    let mut list1: Vec<u32> = vec![];
    let mut list2: Vec<u32> = vec![];
    for line in contents.lines() {
        let (v1, v2) = line.split_once("   ").unwrap();

        if v1.len() == 0 {
            break;
        }

        list1.push(v1.parse::<u32>().unwrap());
        list2.push(v2.parse::<u32>().unwrap());
    }

    list1.sort();
    list2.sort();

    let mut total_distance = 0;

    let count_map1 = to_count_map(list1);
    let count_map2 = to_count_map(list2);

    for (k, v) in count_map1.iter() {
        if let Some(v2) = count_map2.get(k) {
            total_distance += k * v * v2;
        }
    }

    print!("{}", total_distance);
}

fn to_count_map(list: Vec<u32>) -> HashMap<u32, u32> {
    let mut map: HashMap<u32, u32> = HashMap::new();
    for item in list {
        map.entry(item).or_default();
        map.entry(item).and_modify(|x| *x += 1);
    }
    map
}
