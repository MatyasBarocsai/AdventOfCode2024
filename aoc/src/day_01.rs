
fn part1(left: &Vec<i32>, right: &Vec<i32>)
{

    let mut result: i32 = 0;

    for i in 0..left.len()
    {
        println!("{} - {} = {}", left[i], right[i], (left[i] - right[i]).abs());
        result += (left[i] - right[i]).abs();
    }

    println!("Result: {result}");

}

fn part2(left: &Vec<i32>, right: &Vec<i32>)
{
    
    let mut result: i32 = 0;

    for i in 0..left.len()
    {
        let mut count = 0;
        for k in right
        {
            if *k == left[i]{count += 1;}
        }

        result += count * left[i];
    }
    
    println!("Result: {result}");
}


pub fn solution(input: &String){
    
    let lines = input.split("\n");

    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();


    for line in lines{
        
        if line == "" {continue;}

        let identifiers: Vec<&str> = line.trim().split("   ").collect();
        

        let l_id: i32 = identifiers[0].parse().expect("Could not parse");
        let r_id: i32 = identifiers[1].parse().expect("Could not parse");
        
        left.push(l_id);
        right.push(r_id);
    }
    

    // Sort Vectors
    left.sort();
    right.sort();

    part1(&left, &right);
    part2(&left, &right);
}
