use std::collections::HashMap;
use std::fs;
fn main() {
    let input: &str = &fs::read_to_string("input.txt").expect("Unable to read file");
    let mut result = 0;
    for line in input.lines() {
        let mut minimum = HashMap::from([
            ("red", 0),
            ("green", 0),
            ("blue", 0),
        ]);
        let mut parts = line.split(":");
        let tmp: &str = &parts.next().unwrap();
        let game = &tmp[5..tmp.len()].parse::<i32>().unwrap();
        println!("game:{}", game);
        let sets: Vec<&str> = parts.next().unwrap().split(";").collect();
        for set in sets {
            let items: Vec<&str> = set.split(",").collect();
            for item in items {
                let sets: Vec<&str> = item[1..item.len()].split(" ").collect();
                let amount = sets[0].parse::<i32>().unwrap();
                let colour: &str = sets[1];
                if amount > *minimum.get(colour).unwrap() {
                    *minimum.get_mut(colour).unwrap() = amount;
                }
            }


        }
        println!("red minimum {}", minimum.get("red").unwrap());
        println!("green minimum {}", minimum.get("green").unwrap());
        println!("blue minimum {}", minimum.get("blue").unwrap());
        result = result + (minimum.get("red").unwrap()* minimum.get("green").unwrap() * minimum.get("blue").unwrap())

    }
    println!("result is {result}")
}