use std::collections::HashMap;


// TODO: This can give potential errors, watch out!
fn distance(a: &(usize,usize), b: &(usize, usize)) -> i32 {
    let dx = (a.0 as f32 - b.0 as f32).abs();
    let dy = (a.1 as f32 - b.1 as f32).abs();
    ((dx*dx) + (dy*dy)).sqrt() as i32
}

fn find_antinodes_in_line(grid: &Vec<Vec<char>>, a: &(usize,usize), b: &(usize,usize), n: i32) -> Vec<(usize,usize)> {

    //println!("analyzing: {:?} and {:?}", a, b);

    let mut nodes = Vec::<(usize,usize)>::new();

    let dx = a.0 as i32 - b.0 as i32;
    let dy = a.1 as i32 - b.1 as i32;

    for i in 0..n+1 {
        
        // Return if out-of-bounds
        if (a.0 as i32) + (dx*i) < 0 || (a.0 as i32) + (dx*i) >= grid[0].len() as i32 {return nodes;}
        if (a.1 as i32) + (dy*i) < 0 || (a.1 as i32) + (dy*i) >= grid.len() as i32 {return nodes;}

        let search_location: (usize,usize) = ((a.0 as i32 + (dx*i)).try_into().unwrap(),(a.1 as i32 + (dy*i)).try_into().unwrap());
        nodes.push(search_location);
    
    }

    nodes
}


fn find_antinodes_in_radius(grid: &Vec<Vec<char>>, a: &(usize,usize), b: &(usize,usize), radius: i32) -> Vec<(usize,usize)> {

    println!("analyzing: {:?} and {:?}", a, b);

    let mut nodes = Vec::<(usize,usize)>::new();

    // Find antinodes at antennta a
    for y in ((a.1 as i32) - radius)..((a.1 as i32) + radius) {
        
        if y < 0 || y >= grid.len() as i32 {continue;}

        for x in ((a.0 as i32) - radius)..((a.0 as i32) + radius) {
                
            if x < 0 || x >= grid[0].len() as i32 {continue;}
        
            let search_location: (usize,usize) = (x.try_into().unwrap(),y.try_into().unwrap());
            if distance(&search_location, b) == radius * 2 {
                nodes.push(search_location);
            }
        }  
    } 

    nodes
}

fn part1(grid: &Vec<Vec<char>>, antennas: &HashMap<char, Vec<(usize,usize)>>) {

    //let mut antinodes = HashMap::<char, Vec<(usize,usize)>>::new();
    let mut antinodes = HashMap::<(usize,usize), Vec<char>>::new();
    //let mut antinodes = Vec::<(char, Vec<(usize,usize)>)
    
    for (key,value) in antennas {

        //println!("{}: {:?}", key,value); 
        
        let mut nodes = Vec::<(usize,usize)>::new();
        for v1 in value {
            for v2 in value {

                if v1 == v2 {continue;}

                nodes.extend(find_antinodes_in_line(grid, v1, v2, 100));
            }
        }

        for n in &nodes {
            antinodes.entry(*n).or_insert_with(Vec::new).push(*key);
        }

        //antinodes.insert(*key, nodes);
        //println!("a: {:?}, b:{:?}, distance={}", value[i-1], value[i], distance(&value[i-1], &value[i]));
        
    }

    let mut tmp_grid = grid.clone();
    for (node,antenna) in &antinodes {
        tmp_grid[node.1][node.0] = '#';
        println!("Anode: {:?} created by Antennas: {:?}", node,antenna);
    }

    //for r in tmp_grid {println!("{:?}",r);}

    println!("Unique antinodes: {}", antinodes.keys().len()); 


}


pub fn solution(input: &str){
    
    let lines: Vec<&str> = input.split("\n")
        .collect();

    let mut grid = Vec::<Vec<char>>::new();
    let mut antennas = HashMap::<char, Vec<(usize,usize)>>::new();
  
    for y in 0..lines.len(){
        
        if lines[y].is_empty() {continue;}
        
        let mut row = Vec::<char>::new();

        for (x,c) in lines[y].chars().enumerate() { 
            row.push(c);
          
            if c != '.' {

                let coordinate = (x,y);
                antennas.entry(c).or_insert_with(Vec::new).push(coordinate);
            }
        }
        grid.push(row);
    }
    
    //for r in &grid {println!("{:?}",r);}

    part1(&grid, &antennas);


}
