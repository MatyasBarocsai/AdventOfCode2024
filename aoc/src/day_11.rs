use std::collections::HashMap;

fn remove_leading_zeros(s: &str) -> &str {

    // If no leading zero, return the input string
    if s.as_bytes()[0] as char != '0' {return s;}

    for (index,c) in s.chars().enumerate() {
        if c != '0' {return &s[index..];}
    }

    "0"
}

fn calculate_stone(stone: &str) -> (String,String) {
    
    if stone == "0" {
        return ("1".to_string(),"".to_string());
    }
    else if stone.len() % 2 == 0 {

        let left = &stone[..stone.len()/2];
        let tmp_left = remove_leading_zeros(left).to_string();

        let right = &stone[stone.len()/2..]; 
        let tmp_right = remove_leading_zeros(right).to_string();

        return (tmp_left, tmp_right);
    }

    else {
        let tmp = (remove_leading_zeros(stone).parse::<i64>().unwrap() as i64 * 2024).to_string();
        return (tmp, "".to_string());
    }
}

pub fn solution(input: &str){

    let lines = input.split("\n");

    //let mut stones = Vec::<String>::new();
    let mut stones = HashMap::<String, i64>::new();
    let mut stone_conversion = HashMap::<String, (String,String)>::new();

    for line in lines {
        if line.is_empty() {continue;}

        let raw_stones = line.split(" ");

        for stone in raw_stones {
            //stones.push(stone.parse().unwrap());
            stones.insert(stone.parse().unwrap(), 1);
        }

    }

    println!("Initial configurations: {:?}", stones.keys());
    
    for i in 1..76 {
        
        // Freezed in place, we need a temporary map from the prev it 
        let keys: Vec<_> = stones.keys().cloned().collect();
        let tmp = stones.clone(); 

        for stone in keys {

            // Get conversion for stone
            if !stone_conversion.contains_key(&stone) {

                //let tmp = calculate_stone(&stone);
                //println!("Stone {:?} => {:?}", &stone, tmp);

                stone_conversion.insert(stone.to_string(), calculate_stone(&stone));
            }

            let conversion = stone_conversion.get(&stone).unwrap();

            let n = *tmp.get(&stone.to_string()).unwrap();

            //println!("Analyzing: {:?} of {} instance(s)", stone, n);

            //Remove current element n times
            *stones.entry(stone.to_string()).or_insert(1) -= n;
            if *stones.get(&stone.to_string()).unwrap() <= 0 {
                stones.remove(&stone.to_string());
            }
            
            //Add first element
            *stones.entry(conversion.0.clone()).or_insert(0) += n;

            //Add second element
            if !conversion.1.is_empty() {
                *stones.entry(conversion.1.clone()).or_insert(0) += n;
            }

        } 

        //println!("Blink [{}]: {:?}", i, stones);
    }

    //println!("Result: {}", stones.len());

    let mut result: i64 = 0;
    for k in stones.keys() {
        
        let n = *stones.get(k).unwrap();
        if n > 0 { result += n; }
    }
    println!("Result: {}",result);
}
