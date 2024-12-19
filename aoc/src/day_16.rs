use std::collections::{HashMap, HashSet};

#[derive(PartialEq, Debug, Eq, Hash)]
enum Grain {
    Horizontal,
    Vertical
}

#[derive(Debug)]
struct Edge {
    source: usize,
    destination: usize,
    value: i32
}

struct Vertix {
    coordinates: (i32,i32),
    edges: Vec<Edge>
}

fn print_grid(grid: &[Vec<char>]) {
    for y in 0..grid.len() {
        let mut row = String::new();
        for x in 0..grid[0].len() {
            row.push(grid[y][x]);
        }
        println!("{:?}", row);
    }
}

fn paint_line(grid: &mut[Vec<char>], start: (i32,i32), target: (i32,i32)) {

    //Walk in the negative y-direction
    if start.0 - target.0 == 0 && start.1 - target.1 > 0 {
        for i in 0..(start.1 - target.1).abs() {
            grid[start.1 as usize - i as usize][start.0 as usize] = 'O'; 
        }
    }

    //Walk in the positive y-direction
    else if start.0 - target.0 == 0 && start.1 - target.1 < 0 {
        for i in 0..(start.1 - target.1).abs() {
            grid[start.1 as usize + i as usize][start.0 as usize] = 'O';
        }
    }


    //Walk in the negative x-direction
    else if start.1 - target.1 == 0 && start.0 - target.0 > 0{
        for i in 0..(start.0 - target.0).abs() {
            grid[start.1 as usize][start.0 as usize - i as usize] = 'O';
        }
    }

     //Walk in the positive x-direction
    else if start.1 - target.1 == 0 && start.0 - target.0 < 0{
        for i in 0..(start.0 - target.0).abs() {
            grid[start.1 as usize][start.0 as usize + i as usize] = 'O';
        }
    }
}

fn walk_grid(grid: &[Vec<char>], start: (i32,i32), target: (i32,i32)) -> bool {

    //Walk in the negative y-direction
    if start.0 - target.0 == 0 && start.1 - target.1 > 0 {
        for i in 1..(start.1 - target.1).abs() {
            if grid[start.1 as usize - i as usize][start.0 as usize] != '.' {return false;}
        }
    }

    //Walk in the positive y-direction
    if start.0 - target.0 == 0 && start.1 - target.1 < 0 {
        for i in 1..(start.1 - target.1).abs() {
            if grid[start.1 as usize + i as usize][start.0 as usize] != '.' {return false;}
        }
    }


    //Walk in the negative x-direction
    else if start.1 - target.1 == 0 && start.0 - target.0 > 0{
        for i in 1..(start.0 - target.0).abs() {
            if grid[start.1 as usize][start.0 as usize - i as usize] != '.' {return false;}
        }
    }

     //Walk in the positive x-direction
    else if start.1 - target.1 == 0 && start.0 - target.0 < 0{
        for i in 1..(start.0 - target.0).abs() {
            if grid[start.1 as usize][start.0 as usize + i as usize] != '.' {return false;}
        }
    }

    true
}


fn reconstruct_rec(pathmap: &HashMap<(usize,usize), Vec<(usize,usize)>>, current: (usize,usize), path: &mut Vec<usize>, paths: &mut Vec<Vec<usize>>)
{
    
    if current == (80808080, 80808080) {
        paths.push(path.to_vec());
        return;
    }

    path.push(current.0);

    for parent in pathmap.get(&current).unwrap() {
        reconstruct_rec(pathmap, *parent, path, paths);
    }
}

/*
fn reconstruct(path: &HashMap<(usize,usize),Vec<(usize,usize)>>, start: (usize,usize)) -> Vec<Vec<usize>> {
    let mut p = Vec::<Vec<usize>>::new();
    let mut current = start;
    while current != (80808080,80808080) {
        p.push(current.0);
        current = *path.get(&current).unwrap();
    }
    p.reverse();
    p
}*/

fn path_grain(vertices: &Vec<Vertix>, edge: &Edge) -> Grain
{

    let source = &vertices[edge.source];
    let destination = &vertices[edge.destination];
       
    if (source.coordinates.0 - destination.coordinates.0).abs() == 0 {return Grain::Vertical} 

    Grain::Horizontal
}

fn heuristic(vertices: &Vec<Vertix>, source: usize, target: usize) -> i32 {
    (vertices[source].coordinates.0 - vertices[target].coordinates.0).abs() + (vertices[source].coordinates.1 - vertices[target].coordinates.1).abs() + 1000
}

fn priority(queue: &HashSet<(usize,usize)>, f_score: &HashMap<(usize,usize),i32>) -> (usize,usize) {
   
    let mut smallest_value: i32 = 80808080;
    let mut smallest_key: (usize,usize) = (0,0);
    for key in queue {
        if f_score.get(key).expect("11") < &smallest_value {
            smallest_key = *key;
            smallest_value = *f_score.get(key).expect("11");
        }
    }
    smallest_key
}

fn astar(vertices: &Vec<Vertix>, start: usize, target: usize, path: &mut HashMap<(usize,usize),Vec<(usize,usize)>>) {
   
    // 0 -> Horizontal
    // 1 -> Vertical

    let mut open = HashSet::<(usize,usize)>::new();
    let mut g_score = HashMap::<(usize, usize),i32>::new();
    let mut f_score = HashMap::<(usize, usize),i32>::new();
    //let mut path = HashMap::<(usize,usize),Vec<(usize,usize)>>::new(); 

    open.insert((start,0));
    g_score.insert((start, 0), 0);
    f_score.insert((start, 0), 0);
    path.insert((start,0), vec![(80808080,80808080)]);

    while !open.is_empty() {
   
        println!("{:?}", open.len());

        let current_index = priority(&open, &f_score);
        
        if current_index.0 == target {
            //let mut tmp_path = Vec::<usize>::new();
            //println!("\n{:?}", path);
            //reconstruct_rec(&path, current_index, &mut tmp_path, paths);
        }

        let current = &vertices[current_index.0];
        let current_grain = current_index.1;//path_grain_index(vertices, *path.get(&current_index).unwrap(), current_index);
        
        //println!("Analyzing: Vertix at {:?} direction: {:?}", current.coordinates, current_grain);

        open.remove(&current_index);
        for edge in &current.edges {

            let edge_grain_enum = path_grain(vertices, edge);
            let mut edge_grain = 0;
            if edge_grain_enum == Grain::Horizontal {edge_grain = 0;}
            else if edge_grain_enum == Grain::Vertical {edge_grain = 1;}

            let mut extra = 0;
            if edge_grain != current_grain {
                //println!("{:?} Is against the current path grain: {:?}", edge_grain, current_grain);
                extra = 1000;
            }

            let current_g_score = *g_score.entry((edge.destination, edge_grain)).or_insert(80808080);
            let tentative_g_score = *g_score.entry(current_index).or_insert(80808080) + edge.value + extra;
            let heuristic_score: i32 = heuristic(vertices, edge.destination, target);
            let total_f_score = tentative_g_score + heuristic_score;

            //println!("  Edge -> {:?} costs: {:?} grain: ({:?}) tg: ({:?}), g: ({:?}), h: ({:?}) f: ({:?})", &vertices[edge.destination].coordinates, edge.value, path_grain(vertices, edge), tentative_g_score, current_g_score, heuristic_score, total_f_score);

            if tentative_g_score < current_g_score {

                //println!("      <Adding edge to list>");

                path.insert((edge.destination,edge_grain), vec![current_index]);
                //path_edges.push(edge);

                *g_score.entry((edge.destination, edge_grain)).or_insert(80808080) = tentative_g_score; 
                *f_score.entry((edge.destination, edge_grain)).or_insert(80808080) = tentative_g_score + heuristic(vertices, edge.destination, target);
            
                if !open.contains(&(edge.destination, edge_grain)) {
                    open.insert((edge.destination, edge_grain));
                } 
            }
            else if tentative_g_score == current_g_score {
                path.entry((edge.destination,edge_grain)).or_insert(vec![current_index]).push(current_index); 
            }
        }
    }
}

fn calculate_scrore(grid: &mut Vec<Vec<char>>, vertices: &Vec<Vertix>, path: &Vec<usize>) -> i32 {
    
    let mut steps = 0;
    let mut turns = 0;

    let mut grain = Grain::Horizontal;
    for i in 0..path.len()-1 {
        //println!("Analyzing Vertix: {:?} {:?}", i, vertices[path[i]].coordinates); 
        for e in &vertices[path[i]].edges {
            if e.destination == path[i+1] {
     
                paint_line(grid, vertices[e.source].coordinates, vertices[e.destination].coordinates);
               
                let new_grain = path_grain(vertices, e);

                if new_grain != grain {
                    turns += 1;
                }

                //println!("  Edge {:?} to {:?} has value {:?}", vertices[path[i]].coordinates, vertices[path[i+1]].coordinates, e.value) 
                grain = new_grain;
                steps += e.value;
            }
        }
    }

    steps + (turns * 1000)
}

fn part1(grid: &mut Vec<Vec<char>>, vertices: &Vec<Vertix>, start: usize, target: usize) {
    

    let mut pathmap = HashMap::<(usize,usize),Vec<(usize,usize)>>::new();
    let mut path = Vec::<usize>::new();
    let mut paths = Vec::<Vec<usize>>::new();
    

    astar(vertices, start, target, &mut pathmap);
    reconstruct_rec(&pathmap, (target,0), &mut path, &mut paths);

    for p in paths {
        let score = calculate_scrore(grid, vertices, &p);
        println!("Path score: {:?}", score);
    }

    let mut result = 0;
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == 'O' {result += 1;}
        }
    }

    print_grid(grid);
    
    println!("Number of good places: {:?}", result + 1);

}

pub fn solution(input: &str){
    
    let lines = input.split("\n");

    let mut grid = Vec::<Vec<char>>::new();
    let mut vertices = Vec::<Vertix>::new();

    let mut start: (i32,i32) = (0,0);
    let mut target: (i32,i32) = (0,0); 

    // Step 0. Pre parse the Grid with its chars
    for (y, line) in lines.enumerate() {
        
        if line.is_empty() {continue;}

        let mut row = Vec::<char>::new();
        for (x, c) in line.chars().enumerate() {
           
            if c == 'S' {start = (x as i32,y as i32);}
            if c == 'E' {target = (x as i32, y as i32);}
           
            row.push(c);
        }
        grid.push(row);
    }

    // Step 1. Parse all vertices that are a crossroad
    for y in 1..grid.len() - 1 {
        for x in 1..grid[0].len() - 1 {
           
            //Skip wall
            if grid[y][x] == '#' {continue;}

            //Count true neighbours
            let neighbour_coordinates = [(x-1,y), (x+1,y), (x,y-1), (x,y+1)];
            let true_neighbours: Vec<_> = neighbour_coordinates
                .iter()
                .filter(|(x2,y2)| grid[*y2][*x2] == '.')
                .collect();
               
            // If less than two neighbours skip
            if true_neighbours.len() < 2 && grid[y][x] != 'S' && grid[y][x] != 'E' {continue;} 
            
            // If two neighbours but not corner, skip
            if true_neighbours.len() == 2 {
                if (true_neighbours[0].0 as i32 - true_neighbours[1].0 as i32).abs() == 0  {continue;}
                else if (true_neighbours[0].1 as i32 - true_neighbours[1].1 as i32).abs() == 0  {continue;}
            }
            
            vertices.push(Vertix{
                coordinates: (x as i32, y as i32),
                edges: vec![]
            }); 
        }
    }

    for v in &vertices {
        grid[v.coordinates.1 as usize][v.coordinates.0 as usize] = '@';
    }

    // Step 2. Fill the vertices with their neighbours
    for i1 in 0..vertices.len() {
        for i2 in 0..vertices.len() {
           
            // If both indexes point to the same vertix
            if i1 == i2 {continue;}

            let v1 = vertices[i1].coordinates;
            let v2 = vertices[i2].coordinates;

            // If not walkable distance
            if !walk_grid(&grid, v1, v2) {continue;}

            //If they have the same X value
            let mut value: i32 = 0;
            if v1.0 == v2.0 {value = (v1.1 - v2.1).abs();}
            else if v1.1 == v2.1 {value = (v1.0 - v2.0).abs();}

            if value == 0 {continue;}

            vertices[i1].edges.push(Edge{
                source: i1,
                destination: i2,
                value
            });
        }
    }

    // Step 3. Find start and end vertixes
    let mut start_index = 0;
    let mut target_index = 0;
    for i in 0..vertices.len() {

        println!("{:?} is not {:?}", vertices[i].coordinates, start);

        if vertices[i].coordinates == start {start_index = i;}
        else if vertices[i].coordinates == target {target_index = i;}
    }


    println!("Starting point: {:?} -> {:?}", start, &vertices[start_index].coordinates);
    println!("Ending   point: {:?} -> {:?}", target, &vertices[target_index].coordinates);

    part1(&mut grid, &vertices, start_index, target_index);
    //print_grid(&grid);
    

}
