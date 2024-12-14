use regex::Regex;
use std::collections::HashMap;
use std::io;
use std::{thread, time};

const WIDTH: i32 = 101;
const HEIGHT: i32 = 103;

struct Robot {
    position: (i32,i32),
    velocity: (i32,i32)
}

fn move_robot(robot: &mut Robot) {

    // Update X position
    robot.position.0 += robot.velocity.0;
    
    if robot.position.0 < 0 {robot.position.0 += WIDTH;}
    else if robot.position.0 > WIDTH - 1 {robot.position.0 -= WIDTH;}
    
    // Update Y position
    robot.position.1 += robot.velocity.1;
    if robot.position.1 < 0 {robot.position.1 += HEIGHT;}
    else if robot.position.1 > HEIGHT - 1 {robot.position.1 -= HEIGHT;}

}

fn print_robots(robots: &Vec<Robot>) {
    println!("\n");
    for r in robots {
        println!("Robot at {:?} with speed {:?}", r.position, r.velocity);
    }
}

fn clear_grid() -> Vec<Vec<char>> {
    
    let mut grid = Vec::<Vec<char>>::new();
    for _y in 0..HEIGHT {
        let mut row = Vec::<char>::new();
        for _x in 0..WIDTH {row.push('.');}
        grid.push(row);
    }
    grid
}

fn print_grid_view(robots: &Vec<Robot>, msg: &str) {
  
    // Fresh grid
    let mut grid = clear_grid();
    
    // Cls
    print!("\x1B[2J\x1B[1;1H");
    println!("Iteration: {}", msg);

    //Construct hashmap
    let mut robot_mapping = HashMap::<(i32,i32),usize>::new();
    for r in robots {
        *robot_mapping.entry(r.position).or_insert(0) += 1;
    }

    for (key,value) in robot_mapping { grid[key.1 as usize][key.0 as usize] = 'X';}

    for row in grid { 
        let s: String = row.into_iter().collect();
        println!("{}", s); 
    }

}

fn calculate_quadrants(robots: &Vec<Robot>) -> i32 {
    
    //  q1 | q2
    //  -------
    //  q3 | q4

    let mut q1: i32 = 0;
    let mut q2: i32 = 0;
    let mut q3: i32 = 0;
    let mut q4: i32 = 0;

    for r in robots {

        // Q1
        if r.position.0 < (WIDTH - 1)/2 && r.position.1 < (HEIGHT - 1)/2 {q1 += 1;}
        
        //Q2
        if r.position.0 > (WIDTH - 1)/2 && r.position.1 < (HEIGHT - 1)/2 {q2 += 1;} 

        //Q3
        if r.position.0 < (WIDTH - 1)/2 && r.position.1 > (HEIGHT - 1)/2 {q3 += 1;} 
    
        //Q4
        if r.position.0 > (WIDTH - 1)/2 && r.position.1 > (HEIGHT - 1)/2 {q4 += 1;} 
    }
    q1*q2*q3*q4
    //println!("Quadrands: {},{},{},{}, Safety factor: {}", q1,q2,q3,q4,q1*q2*q3*q4);
}

pub fn solution(input: &str){
    let mut robots = Vec::<Robot>::new();

    let lines = input.split("\n");
    for line in lines{
        
        if line.is_empty() {continue;}

        let re = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();
       
        for cap in re.captures_iter(line) {
            robots.push(Robot{
                position: (cap[1].parse().unwrap(),cap[2].parse().unwrap()),
                velocity: (cap[3].parse().unwrap(),cap[4].parse().unwrap())
            })
        }
    }
   

    //print_robots(&robots);
    
    //preloop
    for i in 0..6510 {
        for r_index in 0..robots.len() {
            move_robot(&mut robots[r_index]);
        }
    }
   
    
    let mut scores = Vec::<(i32,i32)>::new();
    for i in 6510..6520 {
        
        let mut input = String::new(); 
        io::stdin()
            .read_line(&mut input) // Read user input and append it to the string
            .expect("Failed to read line");
        
        //let mm = time::Duration::from_millis(300);
        //thread::sleep(mm);

        print_grid_view(&robots, &i.to_string());
        
        for r_index in 0..robots.len() {
            move_robot(&mut robots[r_index]);
        }

        //scores.push((i, calculate_quadrants(&robots)))
    }

    scores.sort_by(|a,b| a.1.cmp(&b.1));

    println!("i={} has score={}", scores[0].0, scores[0].1);
    //print_robots(&robots);
    //calculate_quadrants(&robots);
} 
