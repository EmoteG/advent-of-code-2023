use std::fs;
fn is_symbol(c: char) -> bool {
    if c.is_numeric() {return false}
    if c == '.' {return false}
    return true;
}
fn main() {
    let input: &str = &fs::read_to_string("input.txt").expect("Unable to read file");
    let mut result = 0;
    let lines: Vec<&str> = input.lines().collect::<Vec<&str>>();
    let mut i = 0;
    while i < lines.len() {
        let mut cursor = 0;
        let line = lines[i];
        while cursor < line.len() {
            let mut ran_while = false;
            let mut start_index_defined = false;
            let mut start_index:usize = 0;
            let     stop_index:usize;
            let mut number_string = String::new();
            while line.chars().nth(cursor).unwrap().is_numeric() {
                ran_while = true;
                if !start_index_defined {
                    start_index = cursor;
                    start_index_defined = true;
                }
                number_string.push_str(&line.chars().nth(cursor).unwrap().to_string());
                cursor = cursor + 1;
                if line.chars().nth(cursor) == None {
                    break
                }
            }
            if ran_while {
                stop_index = cursor-1;
                println!("start{}stop{}num{}", start_index, stop_index, number_string);
                let mut valid = false;
                // check sides
                if !(start_index == 0) {
                    if is_symbol(line.chars().nth(start_index-1).unwrap()) {valid = true;}
                }
                if !(stop_index == line.len()-1) {
                    if is_symbol(line.chars().nth(stop_index+1).unwrap()) {valid = true;}
                }

                let mut j = start_index;
                let mut u = stop_index;
                let check_before = !(i==0);
                let check_after = !(i==lines.len()-1);

                if !(start_index == 0){j=start_index-1}
                if !(stop_index == line.len()-1) {u = u+1}
                // println!("{},{},{}", j, u, line.len()-1);
                while !(j==u+1) {
                    if check_before {
                        // println!("checking{},{}", i-1, j);
                        if is_symbol(lines[i-1].chars().nth(j).unwrap()) {
                            valid = true;
                            // println!("found")
                        }

                    }
                    if check_after {
                        // println!("checking{},{}", i+1, j);
                        if is_symbol(lines[i+1].chars().nth(j).unwrap()) {
                            valid = true;
                            // println!("found")

                        }

                    }
                    j = j+1;
                }
                println!{"valid{}", valid}
                if valid {
                    result = result + number_string.parse::<i32>().unwrap()
                }
            }
            cursor = cursor + 1;
            if line.chars().nth(cursor) == None {
                break
            }
            
        }
        println!("{}", line);
        i = i+1;
    }
    println!("result is {result}")
}