use regex::Regex;


struct Instruction{
    value: i32,
    index: usize,
    active: bool
}

fn part1(instructions: &Vec<Instruction>){
   

    let mut result: i32 = 0;
    for i in instructions{
       result += i.value; 
    }
    println!("Result Part 1: {}", result);
}

fn part2(instructions: &Vec<Instruction>){
   

    let mut result: i32 = 0;
    for i in instructions{
        if i.active { result += i.value; }
    }
    println!("Result Part 2: {}", result);
}
pub fn solution(input: &String){
    

    //Capture group 0 == entire instruction
    //        group 1 == first number
    //        group 2 == second number
    let re = Regex::new(r"mul\((\d+),(\d+)\)")
        .expect("Could not create regex");

    let mut instructions = Vec::<Instruction>::new();
    let mut activates = Vec::<usize>::new();
    let mut disables = Vec::<usize>::new();

    for cap in re.captures_iter(input){
  
        //println!("{:?}", cap.get(0).unwrap().start() );

        let index = cap.get(0)
            .unwrap()
            .start();

        let first: i32 = cap.get(1)
            .unwrap()
            .as_str()
            .parse()
            .unwrap();

        let second: i32 = cap.get(2)
            .unwrap()
            .as_str()
            .parse()
            .unwrap();
    
        //println!("{} * {} = {}", first, second, first*second);
        instructions.push(Instruction{value: first*second, index: index, active: true});
    }

    part1(&instructions);
   
    //println!("{}", input);

    //Capture do()
    let re_do = Regex::new(r"do\(\)")
        .expect("Could not create regex");

    for cap in re_do.captures_iter(input){
    
        let index = cap.get(0)
            .unwrap()
            .start();

        activates.push(index);

        //println!("Activate {} found at [{}]", first, index);

    }

    //Capture don't()
    let re_do = Regex::new(r"don't\(\)")
        .expect("Could not create regex");

    for cap in re_do.captures_iter(input){
    
        let index = cap.get(0)
            .unwrap()
            .start();

        disables.push(index);

        //println!("Activate {} found at [{}]", first, index);

    }

    //Disable instructions
    for i in &mut instructions{
        
        let d: Vec<usize> = disables.clone()
            .into_iter()
            .filter(|x| *x < i.index)
            .map(|x| i.index - x)
            .collect();

        let e: Vec<usize> = activates.clone()
            .into_iter()
            .filter(|x| *x < i.index)
            .map(|x| i.index - x)
            .collect();

        let mut min_enable_value: usize = 0;
        let min_enable = e.iter().min();
        match min_enable {
            Some(min) => min_enable_value = *min,
            None      => min_enable_value = 1000000,
        }

        let mut min_disable_value: usize = 0;
        let min_disable = d.iter().min();
        match min_disable {
            Some(min) => min_disable_value = *min,
            None      => min_disable_value = 1000000,
        
        }

        println!("Instruction at [{}], disable={:?} enable={:?}", i.index, min_disable_value, min_enable_value);

        if min_disable_value < min_enable_value && min_disable_value != 1000000{ i.active = false;}

    }

    part2(&instructions);

}
