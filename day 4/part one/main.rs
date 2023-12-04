use std::collections::HashSet;


fn main() {
    let string :&str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
    let mut result = 0;
    
    for line in string.lines() {
        let line_r: Vec<&str> = line.split(':').collect::<Vec<&str>>();
        let str: Vec<&str> = line_r[1].split('|').collect::<Vec<&str>>();
        let a_vec: Vec<i32> = str[0].split(' ').collect::<Vec<&str>>().iter().filter(|x| x.parse::<i32>().is_ok()).map(|x| x.parse::<i32>().unwrap()).collect();
        let b_vec: Vec<i32> = str[1].split(' ').collect::<Vec<&str>>().iter().filter(|x| x.parse::<i32>().is_ok()).map(|x| x.parse::<i32>().unwrap()).collect();
        let a: HashSet<i32> = HashSet::from_iter(a_vec.iter().cloned());
        let b: HashSet<i32> = HashSet::from_iter(b_vec.iter().cloned());
        let count = a.intersection(&b).count();
        let score = if count == 0 {
            0
        } else {
            1*2_i32.pow((count as u32)-1)
        };
        result = result + score;
        
    }
    println!("rsult:{}", result)
}