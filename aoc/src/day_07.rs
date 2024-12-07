use itertools::Itertools;

fn all_combinations(n: usize, ops: &Vec<i64>) -> Vec<Vec<i64>> {

    //let op = vec![0,1];

    let combinations: Vec<Vec<i64>> = (1..n)
        .map(|_| ops.clone() )
        .multi_cartesian_product()
        .collect();

    //for c in &combinations { println!("{:?}", c); }

    combinations

}


fn calcalute(equation: &(i64, Vec<i64>), operators: &Vec<i64>) -> i64 {

    let mut result: i64 = equation.1[0];
   
    // Map 0 == *
    //     1 == +
    //     2 == ||


    for i in 1..equation.1.len() {
        if operators[i-1] == 0 { result *= equation.1[i]; }
        else if operators[i-1] == 1 { result += equation.1[i]; }
        else if operators[i-1] == 2 { result = format!("{}{}", result, equation.1[i]).parse().expect("Could not concat"); }
    }

    result
}


pub fn solution(input: &str){
    
    let lines = input.split("\n");

    let mut equations = Vec::<(i64, Vec<i64>)>::new();

    for line in lines{
        
        if line.is_empty() {continue;}

        let p: Vec<&str> = line.split(": ")
            .collect();

        let mut coefficiants = Vec::<i64>::new();
        let p1: Vec<&str> = p[1].split(' ')
            .collect();
   
        for c in p1 {
         
            //println!("{c}");
            coefficiants.push(c.parse().expect("Could not parse {c} to i32"));
        }

        //println!("Value={}", p[0]);
        equations.push((p[0].parse().expect("Could not parse value to i32"), coefficiants));
    }


    let mut part_1: i64 = 0;
    for e in &equations {
        
        //println!("\nEquation {:?}", e);

        let ops = vec![0,1,2];
        let combinations: Vec<Vec<i64>> = all_combinations(e.1.len(), &ops);

        for c in combinations {
           
            let result = calcalute(e, &c);

            if result == e.0 {
                part_1 += result;
                //println!("Using combination {:?} = {result}", c);
                break;
            }

        }
    }

    println!("Result: {:?}", part_1);


}
