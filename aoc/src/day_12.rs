use std::io;
use std::collections::HashMap;

struct Region {
    id: usize,
    symbol: char,
    coordinates: Vec<(usize,usize)>,
    area: u32,
    perimeter: i32,
    borders: HashMap<(usize,usize),Vec<bool>>
}

fn parse_region(grid: &Vec<Vec<char>>, region: &mut Region, starting: (usize,usize)) {

    let mut search_space = vec![starting];
    region.coordinates.push(starting);

    while !search_space.is_empty() {
  
        //println!("Looking for {} in {:?}", region.symbol, search_space);

        /*let mut input = String::new(); 
        io::stdin()
            .read_line(&mut input) // Read user input and append it to the string
            .expect("Failed to read line");
        */
        
        let loop_space = search_space.clone();
        search_space.clear();
        for (x,y) in loop_space {
           
            let mut tmp = Vec::<(usize,usize)>::new();

            //if x as i32 - 1 < 0 || y as i32 - 1 < 0 || x + 1 == grid[0].len() || y + 1 == grid.len() {continue;}

            if x > 0 && grid[y][x-1] == region.symbol {tmp.push((x-1,y));}
            if y > 0 && grid[y-1][x] == region.symbol {tmp.push((x,y-1));}

            if x + 1 < grid[0].len() && grid[y][x+1] == region.symbol {tmp.push((x+1,y));}
            if y + 1 < grid.len() && grid[y+1][x] == region.symbol {tmp.push((x,y+1));}
        
            for v in tmp {
                if !region.coordinates.contains(&v) {
                    search_space.push(v);
                    region.coordinates.push(v);
                }
            }
        }
    }
    region.area = region.coordinates.len() as u32;
}

fn naive_perimeter_calculation(grid: &Vec<Vec<char>>, region: &mut Region) {

    for (x,y) in &region.coordinates {
      
        // top:0 -> right:1 -> bottom:2 -> left:3

        //println!("({},{}) count={}",x,y,count);
        let mut borders = vec![false,false,false,false];

        // If out-of-bounds
        if *y == 0 {borders[0] = true;} 
        if *x + 1 == grid[0].len() {borders[1] = true;} 
        if *y + 1 == grid.len() {borders[2] = true;}
        if *x == 0 {borders[3] = true;} 
    
        // In-bounds, but a border-coordinate
        if *y > 0 && grid[*y-1][*x] != region.symbol {borders[0] = true;}
        if *x + 1 < grid[0].len() && grid[*y][*x+1] != region.symbol {borders[1] = true;}
        if *y + 1 < grid.len() && grid[*y+1][*x] != region.symbol {borders[2] = true;}
        if *x > 0 && grid[*y][*x-1] != region.symbol {borders[3] = true;}
   
        region.borders.insert((*x,*y),borders);
    }

    // Part 1
    let mut result = 0;
    for (_key,value) in region.borders.iter() {
        for b in value.iter() { if *b {result += 1;} }
    }

    region.perimeter = result;

    //println!("\nAnalyzing region: {} with {} perimeter", region.symbol, region.perimeter);

    let mut visited_pairs = Vec::<((usize,usize),(usize,usize))>::new();
    for (k1,v1) in region.borders.iter() {

        for (k2,v2) in region.borders.iter() {
            
            //Edge-cases
            if k1 == k2 {continue;}
            if visited_pairs.contains(&(*k1,*k2)) {continue;}
            
            let dx = (k1.0 as i32 - k2.0 as i32).abs();
            let dy = (k1.1 as i32 - k2.1 as i32).abs();

            //println!("{:?} - {:?} = ({},{})", k1,k2,dx,dy);
            if (dx,dy) == (1,0) || (dx,dy) == (0,1) {
            //if (k1.0 as i32 - k2.0 as i32).abs() <= 1 && (k1.1 as i32 - k2.1 as i32).abs() <= 1 {

                //println!("Connected cells: {:?}{:?} and {:?}{:?}", k1,v1,k2,v2);

                let mut shared_sides = 0;
                for i in 0..4 {
                    if v1[i] && v2[i] { shared_sides += 1; }
                }

                region.perimeter -= shared_sides;
                visited_pairs.push((*k1,*k2));
                visited_pairs.push((*k2,*k1));
            }
        }
    }
}


pub fn solution(input: &str){
    
    let lines = input.split("\n");

    let mut regions = Vec::<Region>::new();
    let mut grid = Vec::<Vec<char>>::new();
    // Parse the grid
    for (line_index, line) in lines.enumerate()
    {
        if line.is_empty() {continue;}
        let mut row = Vec::<char>::new();
        for (char_index, c) in line.chars().enumerate() { row.push(c); }
        grid.push(row);
    }
    println!("Grid Parsing complete...");

    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            
            //println!("({},{})", x, y);
            
            let c = grid[y][x];

            //Push the first value automatically
            if y == 0 && x == 0 {
                let r = Region{
                    id: 0,
                    symbol: c,
                    coordinates: Vec::<(usize,usize)>::new(),
                    area: 0,
                    perimeter: 0,
                    borders: HashMap::<(usize,usize),Vec<bool>>::new()
                };
                regions.push(r);
                parse_region(&grid, &mut regions[0], (0,0));

                continue;
            }

            // Check all existing regions with the current char,
            // If the char is located in an existing region, add it,
            // Otherwise create a brand new region.
            let mut no_match = true;
            for r_id in 0..regions.len() {
     
                // If current char and region symbol is different, continue
                if c != regions[r_id].symbol {continue;}

                // Check if char in region, 808080 == No, otherwise return correct region_id
                let match_in_region = &regions[r_id].coordinates.contains(&(x,y));

                if *match_in_region {
                    //regions[r_id].coordinates.push((x, y));
                    no_match = false;
                    break;
                }
            }
           
            // If no match in any region, add a new region
            if no_match {
                let r = Region{
                    id: regions.len(), 
                    symbol: c, 
                    coordinates: Vec::<(usize,usize)>::new(),
                    area: 0,
                    perimeter: 0,
                    borders: HashMap::<(usize,usize),Vec<bool>>::new()
                };

                let index = regions.len();
                regions.push(r);

                parse_region(&grid, &mut regions[index], (x,y));
            }
        }
    }
    println!("Regions parsing complete...");

    for i in 0..regions.len() {
        naive_perimeter_calculation(&grid, &mut regions[i]);
    }
    println!("Perimeter calculation complete...");


    let mut result = 0;
    for r in &regions {
        println!("Region [{}] with symbol [{}], area [{}] perimeter [{}] {:?}", r.id, r.symbol, r.area, r.perimeter, r.coordinates);
        
        result += r.area as i32 * r.perimeter;
    }

    println!("Number of regions identified: {}, part 1: {}", &regions.len(), result);




    // 1509750 -> too high
        // 1508316 -> wrong
            //1473620 1473620
        // 1473608 -> wrong 
    // 1473366 -> too low
    // 16568
}
