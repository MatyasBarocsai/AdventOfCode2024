use std::collections::HashMap;
use std::io;
use std::{thread, time};

#[derive(PartialEq)]
enum CellType {
    Box,
    Wall,
    Robot
}

struct Object {
    positions: Vec<(usize,usize,char)>,
    cell_type: CellType,
}

fn manual_mode(){
    
    let mut input = String::new(); 
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
}

fn timer_mode(){
    let mm = time::Duration::from_millis(100);
    thread::sleep(mm);
}


fn calculate_gps(objects: &Vec<Object>) -> usize {

    let mut result = 0;

    for o in objects{
        if o.cell_type == CellType::Box {
            result += (100 * o.positions[0].1) + o.positions[0].0;
        }
    }
    result
}

fn find_at_position(position: (usize,usize), objects: &Vec<Object>, object_index: usize) -> (i32, usize) {
    
    // 0  == empty
    // 1  == Box
    // -1 == Wall 
    for (i,o) in objects.into_iter().enumerate() {

        if i == object_index {continue;}

        for (x,y,_c) in &o.positions {
            if (*x,*y) == position {
                if o.cell_type == CellType::Wall {return (-1,i);}
                else if o.cell_type == CellType::Box {return (1,i);}
            }
        }
    }
    (0,0)
}

fn move_object(grid: (usize,usize), objects: &mut Vec<Object>, object_index: usize, direction: char) {

    let mut new_positions = Vec::<(usize,bool,Vec<(i32,i32,char)>)>::new(); 
    recursive_collision(grid, objects, object_index, direction, &mut new_positions);

    println!("Collision map: {:?}", new_positions);
    
    //Check for collision, return if even one collission occurs
    for p in &new_positions {if !p.1 {return;}}

    //Move all the objects
    for p in &new_positions {
        
        let index = p.0;
        let new_positions = &p.2;

        // Perform move on all positions
        for i in 0..new_positions.len() {
            objects[index].positions[i] = (new_positions[i].0 as usize, new_positions[i].1 as usize, new_positions[i].2);
        }     
    }
}

fn recursive_collision(grid: (usize,usize), objects: &mut Vec<Object>, object_index: usize, direction: char, np: &mut Vec<(usize,bool,Vec<(i32,i32,char)>)>) {

    let object = &mut objects[object_index];

    // Clone old positions
    let mut new_positions = Vec::<(i32,i32,char)>::new();
    for (x,y,c) in &object.positions {
        new_positions.push((*x as i32,*y as i32,*c));
    } 

    // Calculate all new positions
    for new_position in &mut new_positions {
        match direction {
            '^' => new_position.1 -= 1,
            '>' => new_position.0 += 1,
            'v' => new_position.1 += 1,
            '<' => new_position.0 -= 1,
            _   => println!("Unknown direction found [{}]", direction) 
        }
    }

    //println!("{:?} -> {:?}", object.positions, new_positions);

    // Check if any of the new positions are out-of-bounds
    for new_position in &new_positions {
        if new_position.0 < 0 || new_position.0 >= grid.0 as i32 || new_position.1 < 0 || new_position.1 >= grid.1 as i32 {
            np.push((object_index, false, new_positions));
            return;
        }
    }

    // Check for all collisions
    let mut occupied = HashMap::<usize,usize>::new();
    for new_position in &new_positions {
        let collision = find_at_position((new_position.0 as usize, new_position.1 as usize), objects, object_index);
        
        // If even one pos has Wall collison, return
        if collision.0 == -1 {
            np.push((object_index, false, new_positions));
            return;
        }

        // Add to collision mapping, unique boxes will have their own key
        if collision.0 == 1 {
            *occupied.entry(collision.1).or_insert(0) += 1;
        }
    }

    // Schedule all positions to move
    np.push((object_index, true, new_positions));

    // Move all boxes that collide
    for index in occupied.keys() {

        //println!("Need to move box at: {:?}", objects[*index].positions);
        recursive_collision(grid, objects, *index, direction, np);
        // If even one box collides with a wall, return false
        //if !next_move {return;}
    } 

    // Perform move on all positions
    //for i in 0..new_positions.len() {
    //    objects[object_index].positions[i] = (new_positions[i].0 as usize, new_positions[i].1 as usize, new_positions[i].2);
    //}     
}


fn clear_grid(width: usize, height: usize) -> Vec<Vec<char>> {
    
    let mut grid = Vec::<Vec<char>>::new();
    for _y in 0..height {
        let mut row = Vec::<char>::new();
        for _x in 0..width {row.push('.');}
        grid.push(row);
    }
    grid
}


fn print_grid_view(objects: &Vec<Object>, msg: char, width: usize, height: usize) {
  
    // Fresh grid
    let mut grid = clear_grid(width, height);
    
    // Cls
    print!("\x1B[2J\x1B[1;1H");
    println!("Iteration: {}", msg);

    //Place objects
    for o in objects {
        for (x,y,symbol) in &o.positions {
            grid[*y][*x] = *symbol;
        }
        //grid[o.position.1][o.position.0] = o.symbol;
    }

    //Print grid
    for row in grid { 
        let s: String = row.into_iter().collect();
        println!("{}", s); 
    }

}


pub fn solution(input: &str){

    let lines = input.split("\n");
    
    let mut objects = Vec::<Object>::new();
    
    let mut robot_index: usize = 0;
    let mut robot_path = Vec::<char>::new();

    let mut width: usize = 0;
    let mut height: usize = 0;
   
    // 0 1
    // 1 2

    for (y, line) in lines.enumerate() {

        if line.is_empty() {continue;}
   
        //Parse map
        if line.contains('.') || line.contains('#') {

            let mut x: usize = 0;
            for c in line.chars() {

                if y == 0 {width += 2;}

                if c == '#' {
                    objects.push(Object{
                        positions: vec![(x,y,'#'),(x+1,y,'#')],
                        cell_type: CellType::Wall
                    });
                }
                else if c == 'O' {
                    objects.push(Object{
                        positions: vec![(x,y,'['),(x+1,y,']')],
                        cell_type: CellType::Box
                    });
                }
                else if c == '@' {
                     objects.push(Object{
                        positions: vec![(x,y,'@')],
                        cell_type: CellType::Robot
                     });
                     robot_index = objects.len() - 1;
                }
                x += 2;
            }
            height += 1;
        }

        //Parse input
        else {
            for c in line.chars(){robot_path.push(c);}
        }
    }

    for dir in &robot_path {
        //manual_mode();
        //timer_mode();
        move_object((width, height), &mut objects, robot_index, *dir);
        //print_grid_view(&objects, *dir, width, height);
    }


    print_grid_view(&objects, '-', width, height);

    println!("Total GPS is: {}", calculate_gps(&objects));


    //
    // 1474312 --> too high
    // 1446175
    //
    //
    //


} 
