fn main() {
    let string: &str = "";
    use std::collections::HashMap;

    let solar_distance = HashMap::from([
        ("one", '1'), ("two", '2'), ("three", '3'), ("four", '4'),
        ("five", '5'),
        ("six", '6'), ("seven", '7'), ("eight", '8'), ("nine", '9')
    ]);
    let mut total = 0;
    for line in string.lines() {
        let mut first_found: bool = false;
        let mut first_nmb: char = '1';
        let mut last_nmb: char = '1';
        let mut i: usize = 0;
        while i < line.len() {
            let c: char = line[i];
            if c.is_ascii_alphabetic() {
                continue;
            }
            if !first_found {
                first_found = true;
                first_nmb = c;
                println!("first {}", c);
            }
            println!("{}", c);
            last_nmb = c;
            i = i + 1;
        }
        println!("last{}", last_nmb);
        let combined_nmb = format!("{}{}", first_nmb, last_nmb);
        let combined_int = combined_nmb.parse::<i32>().unwrap();
        println!("combined{}", combined_int);
        total = total + combined_int;
    }
    println!("total {}", total)
}
