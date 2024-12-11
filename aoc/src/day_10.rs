use std::{collections::HashMap,collections::VecDeque,collections::HashSet};

struct Edge {
    source: (usize,usize),
    destination: (usize,usize),
    weight: i32
}

struct Vertix {
    coordinates: (usize,usize),
    height: i32,
    neighbours: Vec<(usize,usize)>
}


fn dfs(grid: &Vec<Vec<Vertix>>, start: (usize,usize), target: (usize,usize), visited: &mut HashSet<(usize,usize)>, path: &mut Vec<(usize,usize)>, all_paths: &mut Vec<Vec<(usize,usize)>>) {
    
    visited.insert(start);
    path.push(start);

    if start == target {all_paths.push(path.clone());}

    for n in &grid[start.1][start.0].neighbours {
    
        if visited.contains(n) {continue;}

        dfs(grid, *n, target, visited, path, all_paths);
    }

    visited.remove(&start);
    path.pop();

}


fn bfs(grid: &Vec<Vec<Vertix>>, start: (usize,usize), target: (usize,usize)) -> bool {

    let mut queue = VecDeque::<(usize,usize)>::new();
    let mut visited = Vec::<(usize,usize)>::new();
    let mut path = HashMap::<(usize,usize),(usize,usize)>::new(); 

    //initialize
    queue.push_back(start);
    visited.push(start);
    path.insert(start, (808080,808080));

    //Main loop
    while !queue.is_empty() {

        let current = queue.pop_front().unwrap();

        if current == target {return true;}
       
        for n in &grid[current.1][current.0].neighbours {
            
            if visited.contains(n) {continue;}

            visited.push(*n);
            path.insert(*n, current);
            queue.push_back(*n);

        }
    }
    false
}

fn calculate_neighbours(grid: &mut Vec<Vec<Vertix>>, x: usize, y: usize) {
   
    let x = grid[y][x].coordinates.0;
    let y = grid[y][x].coordinates.1;
    let height = grid[y][x].height;

    //println!("({},{} => Grid[{}][{}])", x,y,grid.len(),grid[0].len());

    //Top
    if y > 0 && (grid[y-1][x].height - height) == 1 {
        grid[y][x].neighbours.push((x,y-1));
    }

    //Bottom
    if y < grid.len() - 1 && (grid[y+1][x].height - height) == 1 {
        grid[y][x].neighbours.push((x,y+1));
    }

    //Left
        if x > 0 && (grid[y][x-1].height - height) == 1 {
        grid[y][x].neighbours.push((x-1,y));
    }

    //Right
    if x < grid[0].len() - 1 && (grid[y][x+1].height - height) == 1 {
        grid[y][x].neighbours.push((x+1,y));
    }
}

fn part1(grid: &Vec<Vec<Vertix>>, start_coords: &Vec<(usize,usize)>, end_coords: &Vec<(usize,usize)>) {

    let mut result: i32 = 0;
    for start in start_coords {

        //println!("\nAnalyzing start coordinate {:?}", *start);

        let mut trail_score: i32 = 0;
        for end in end_coords {

            let is_path = bfs(grid, *start, *end);
            if is_path {trail_score += 1;}
            //println!("Target: {:?}, path={is_path}", *end);
        }
        result += trail_score;
    }
    
    println!("Result: {}", result);
}


fn part2(grid: &Vec<Vec<Vertix>>, start_coords: &Vec<(usize,usize)>, end_coords: &Vec<(usize,usize)>) {

    let mut result: i32 = 0;
    for start in start_coords {
        let mut trail_rating: i32 = 0;
        for end in end_coords {

            let mut visited = HashSet::<(usize,usize)>::new();
            let mut path = Vec::<(usize,usize)>::new();
            let mut all_paths = Vec::<Vec<(usize,usize)>>::new();

            dfs(grid, *start, *end, &mut visited, &mut path, &mut all_paths);

            trail_rating += all_paths.len() as i32;
        }
        result += trail_rating;
    }
    println!("Result: {}", result);
}


pub fn solution(input: &str){
    
    let lines = input.split("\n");

    let mut grid = Vec::<Vec<Vertix>>::new();
    
    let mut start_coords = Vec::<(usize,usize)>::new();
    let mut end_coords = Vec::<(usize,usize)>::new();

    // Parse all the Vertices
    for (y, line) in lines.enumerate() {
        
        if line.is_empty() {continue;}

        let mut row = Vec::<Vertix>::new();
        for (x, c) in line.chars().enumerate() {
            let v = Vertix {
                coordinates: (x,y),
                height: c as i32 - 0x30,
                neighbours: Vec::<(usize,usize)>::new()
            };

            if v.height == 0 {start_coords.push(v.coordinates);}
            if v.height == 9 {end_coords.push(v.coordinates);}
            
            row.push(v);
        }
        grid.push(row);
    }

    // Parse all the neighbours
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            calculate_neighbours(&mut grid, x, y);
            //println!("({},{}) [{}] has neighbours: {:?}", x,y, grid[y][x].height,grid[y][x].neighbours);
        }
    }

    part2(&grid, &start_coords, &end_coords); 

    //let src = (0,0);
    //let target = (0,3);
    //println!("Path between src ({:?}) and ({:?}) => {:?}", src, target, bfs(&grid, src, target));


}
