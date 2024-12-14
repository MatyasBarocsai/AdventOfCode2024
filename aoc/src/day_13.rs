use regex::Regex;
use std::collections::HashMap;

extern crate nalgebra as na;
use na::{DMatrix, DVector};


fn naive_part1(equations: &Vec<Vec<Vec<f64>>>) {

    let mut result: f64 = 0.0;
    'outer: for eq in equations {

        for a in 0..101 {
            for b in 0..101 {
           
                let x = a as f64 * eq[0][0] + b as f64 * eq[1][0];
                let y = a as f64 * eq[0][1] + b as f64 * eq[1][1];

                if x == eq[2][0] && y == eq[2][1] {
                    
                    println!("x={} and y={}", x, y);

                    result += a as f64 * 3.0 + b as f64;
                    continue 'outer;
                } 
            }
        }
    }
    println!("Result {} tokens", result);
}

fn part2(equations: &Vec<Vec<Vec<f64>>>) {

    let error: f64 = 10000000000000.0;
    //let error: f64 = 0.0;

    let mut result: f64 = 0.0;
    for eq in equations {

        let x1 = [eq[0][0], eq[1][0]];
        let x2 = [eq[0][1], eq[1][1]];
        
        let matrix_a = DMatrix::from_row_slice(2,2,&[x1[0], x1[1], x2[0], x2[1]]);
        let matrix_b = DVector::from_row_slice(&[eq[2][0] + error, eq[2][1] + error]);

        let solution = matrix_a.lu().solve(&matrix_b).unwrap();
      
       
        let precision = 1000.0;
        let p1 = ((solution[0] * precision).round() / precision).to_string(); 
        let p2 = ((solution[1] * precision).round() / precision).to_string(); 

        if p1.contains('.') || p2.contains('.') {
            //println!("\nSolution found: {:?} {:?}", solution[0], solution[1]);
            continue;
        }

        //println!("True solution: A={} and B={}", p1, p2);

        result += solution[0] * 3.0 + solution[1];

        //println!("Solution: x={} y={}", solution[0], solution[1]);
    }
    println!("Result: {} tokens", result);
}


pub fn solution(input: &str){
    
    let lines = input.split("\n");

    let mut equations = Vec::<Vec<Vec<f64>>>::new();

    let re_button_a = Regex::new(r"Button A: X\+(\d+), Y\+(\d+)").unwrap();
    let re_button_b = Regex::new(r"Button B: X\+(\d+), Y\+(\d+)").unwrap();
    let re_button_y = Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();

    let mut current_equation = Vec::<Vec<f64>>::new();

    for line in lines {
        
        if line.is_empty() {continue;}

        for cap in re_button_a.captures_iter(line) {
            //println!("{:?}",cap);
            let x: f64 = cap[1].parse().unwrap();
            let y: f64 = cap[2].parse().unwrap();
            current_equation.push(vec![x,y]);
            continue;
        }
        
        for cap in re_button_b.captures_iter(line) {
            //println!("{:?}",cap);
            let x: f64 = cap[1].parse().unwrap();
            let y: f64 = cap[2].parse().unwrap();
            current_equation.push(vec![x,y]);
            continue;
        }
        
        for cap in re_button_y.captures_iter(line) {
            //println!("{:?}",cap);
            let x: f64 = cap[1].parse().unwrap();
            let y: f64 = cap[2].parse().unwrap();
            current_equation.push(vec![x,y]);

            equations.push(current_equation.clone());
            current_equation.clear();
            continue;
        }
    }


    part2(&equations);
    //naive_part1(&equations);



    //
    // 40296 -- too low!

}
