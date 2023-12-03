use std::collections::HashMap;
use std::fs;
fn main() {
    let input: &str = &fs::read_to_string("input.txt").expect("Unable to read file");
    let limits = HashMap::from([
        ("red", 12),
        ("green", 13),
        ("blue", 14),
    ]);
    let mut good = 0;
    for line in input.lines() {
        let mut parts = line.split(":");
        let tmp: &str = &parts.next().unwrap();
        let game = &tmp[5..tmp.len()].parse::<i32>().unwrap();
        println!("game:{}", game);
        let sets: Vec<&str> = parts.next().unwrap().split(";").collect();
        let mut correct = true;
        'sets: for set in sets {
            // let amount: i32 = set[0..1]
            let items: Vec<&str> = set.split(",").collect();
            for item in items {
                let sets: Vec<&str> = item[1..item.len()].split(" ").collect();
                let amount = sets[0].parse::<i32>().unwrap();
                let colour: &str = sets[1];
                if amount > *limits.get(colour).unwrap() {
                    correct = false;
                    break 'sets
                }
                println!("{}df", amount);
                println!("{}df", colour);
            }


        }
        if correct {
        good = good + game
        }

    }
    println!("good is{good}")
}