use std::i32;
use std::collections::HashMap;

#[derive(Copy, Clone)]
struct Guard {
    x: usize,
    y: usize,
    direction: char
}

fn turn_guard_right(grid: &mut Vec<Vec<char>>, guard: &mut Guard) {
    
    if guard.direction == '^' { guard.direction = '>'; }
    else if guard.direction == '>' { guard.direction = 'v'; }
    else if guard.direction == 'v' { guard.direction = '<'; }
    else { guard.direction = '^'; }

    grid[guard.y][guard.x] = guard.direction;

}


fn move_guard_forward(grid: &mut Vec<Vec<char>>, guard: &mut Guard) -> i32 {
    
    // Return: -1 => Out-of-Bounds
    // Return:  0 => Hit a obstacle!
    // Return:  1 => Step complete

    // Mark old step
    grid[guard.y][guard.x] = 'X';

    // Check for out-of-bounds
    if guard.direction == '^' && guard.y as i32 - 1 < 0 {return -1;}
    else if guard.direction == '>' && guard.x as i32 + 1 >= grid[0].len() as i32 {return -1;}
    else if guard.direction == 'v' && guard.y as i32 + 1 >= grid.len() as i32 {return -1;}
    else if guard.direction == '<' && guard.x as i32 - 1 < 0 {return -1;}

    // Check for obstacle
    if guard.direction == '^' && grid[guard.y-1][guard.x] == '#' {return 0;}
    else if guard.direction == '>' && grid[guard.y][guard.x+1] == '#' {return 0;}
    else if guard.direction == 'v' && grid[guard.y+1][guard.x] == '#' {return 0;}
    else if guard.direction == '<' && grid[guard.y][guard.x-1] == '#' {return 0;}
    
    // Make step
    if guard.direction == '^' { grid[guard.y-1][guard.x] = '^'; guard.y -= 1;}
    else if guard.direction == '>' { grid[guard.y][guard.x+1] = '>'; guard.x += 1;}
    else if guard.direction == 'v' { grid[guard.y+1][guard.x] = 'v'; guard.y += 1;}
    else if guard.direction == '<' { grid[guard.y][guard.x-1] = '<'; guard.x -= 1;}
    
    1

}

fn simulate_grid(grid: &mut Vec<Vec<char>>, guard: &mut Guard) -> i32 {
  
    //Return  1 => loop
    //Return  0 => normal

    //for c in &*grid {println!("{:?}", c);}
    //println!("\n\n");


    let mut visited = HashMap::new();

    loop {

        if !visited.contains_key(&(guard.x,guard.y)) {            
            visited.insert((guard.x, guard.y), 1);
        }

        visited.insert((guard.x, guard.y), visited.get(&(guard.x,guard.y)).unwrap()+1);
            
        if *visited.get(&(guard.x, guard.y)).unwrap() >= 100 {return 1;}


        //for row in &grid { println!("{:?}", row); }
        //println!("\n\n");

        let result = move_guard_forward(grid, guard);

        if result == 0 { turn_guard_right(grid, guard); }
        else if result == -1 { break; }

    }

    0

}


pub fn solution(input: &String){
    
    let lines: Vec<&str> = input.split("\n")
        .collect();

    let mut grid = Vec::<Vec<char>>::new();  
    let mut guard = Guard {x: 0, y: 0, direction: '^'};
    let mut obstacles_n = 0;
    let mut obstacles = Vec::<(i32,i32)>::new();
  
    let mut time_loops = Vec::<(i32,i32)>::new();

    for y in 0..lines.len(){
        
        if lines[y].is_empty() {continue;}
       
        
        let mut row = Vec::<char>::new();
        let mut x = 0;
        
        for c in lines[y].chars() {
            
            if c == guard.direction { guard.y = y; guard.x = x; }
            if c == '#' { obstacles_n += 1; obstacles.push((x as i32,y as i32)); }

            row.push(c);
            x += 1;
        }
        grid.push(row);
    }

    println!("Number of obstacles: {}", obstacles_n);
    let mut time_loop_n = 0; 
    
    // Brute force the time-loops
    for o1 in &obstacles {

        for o2 in &obstacles {

            if o1 == o2 {continue;}
            if (o1.1 - o2.1) != -1 {continue;}

            for o3 in &obstacles {

                if o3 == o1 {continue;}
                if o3 == o2 {continue;}
                if (o2.0 - o3.0) != 1 {continue;}

                let o4 = (o1.0 - 1, o3.1 - 1);

                if o4.0 < 0 || o4.1 < 0 {continue;}

                if grid[o4.1 as usize][o4.0 as usize] == '^' {continue;}

                time_loops.push(o4);

                time_loop_n += 1;
                println!("right-leaning {:?}, {:?}, {:?}, {:?}", o1, o2, o3, o4);
            }
        }
    }

    // Brute force the time-loops
    for o1 in &obstacles {

        for o2 in &obstacles {

            if o1 == o2 {continue;}
            if (o1.1 - o2.1) != 1 {continue;}

            for o3 in &obstacles {

                if o3 == o1 {continue;}
                if o3 == o2 {continue;}
                if (o2.0 - o3.0) != -1 {continue;}

                let o4 = (o1.0 + 1, o3.1 + 1);

                if o4.0 >= grid[0].len() as i32 || o4.1 >= grid.len() as i32 {continue;}

                if grid[o4.1 as usize][o4.0 as usize] == '^' {continue;}

                time_loops.push(o4);

                time_loop_n += 1;
                println!("left-leaning {:?}, {:?}, {:?}, {:?}", o1, o2, o3, o4);
            }
        }
    }

    
    println!("Time-loops: {}", time_loop_n);

    //for row in grid { println!("{:?}", row); }
    println!("Guard {} at: [{}:{}]", guard.direction, guard.x, guard.y);


    // Part 1
    let mut part1_grid = grid.to_vec();
    let mut part1_guard = guard.clone();
    simulate_grid(&mut part1_grid, &mut part1_guard);
    
    let mut count = 0;
    for row in &part1_grid {
        for c in row  {
            if *c == 'X' {count += 1;}
        }
    }
    println!("Part 1 result: {}", count);
 

    // Part 2
    /*
    let mut loop_count: i32 = 0;
    for o in time_loops {

        println!("simulating obstacle {:?}", o);


        let mut tmp_grid = grid.to_vec();
        let mut tmp_guard = guard.clone();
        tmp_grid[o.1 as usize][o.1 as usize] = 'O';
        loop_count += simulate_grid(&mut tmp_grid, &mut tmp_guard);

    }

    println!("Time loops: {}", loop_count);
    */


    let mut loop_count: i32 = 0;
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
    
            let mut tmp_grid = grid.to_vec();
            let mut tmp_guard = guard.clone();

            tmp_grid[y][x] = '#';
            loop_count += simulate_grid(&mut tmp_grid, &mut tmp_guard);
        }
    }

    println!("Time loops: {}", loop_count);


    //59 871 -> too high

}
