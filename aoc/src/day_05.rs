struct Rule {
    before: i32,
    after: i32,
}

fn valid_pair(rules: &Vec<Rule>, before: i32, after: i32) -> bool {
    
    for r in rules {
        if r.before == after && r.after == before { return false; } 
    }
    
    true
}


fn part2(rules: &Vec<Rule>, updates: &Vec<Vec<i32>>) {
    
    //for r in rules { println!("Rule found: {}|{}", r.before, r.after); }
    let mut result: i32 = 0;

    for u in updates { 

        let mut invalid = false;
        //println!("{:?}", u); 
        
        for i in 1..u.len() {
            if !valid_pair(&rules, u[i-1], u[i]) { 
                //println!("Break ordering: {:?}", u); 
                invalid = true; 
                break; 
            }
        }
        if invalid {
           
            let mut tmp = u.to_vec();

            println!("{:?}", tmp);

            tmp.sort_by(|a, b| {
                
                if valid_pair(rules,*a,*b) { return std::cmp::Ordering::Less }
                else { return  std::cmp::Ordering::Greater; }
                
            });
            
            println!("{:?}", tmp);

            result += tmp[tmp.len()/2];
        }
    } 
    println!("Result Part 1: {}", result);
}

fn part1(rules: &Vec<Rule>, updates: &Vec<Vec<i32>>) {
    
    //for r in rules { println!("Rule found: {}|{}", r.before, r.after); }
    let mut result: i32 = 0;

    for u in updates { 

        let mut invalid = false;
        //println!("{:?}", u); 
        
        for i in 1..u.len() {
            if !valid_pair(&rules, u[i-1], u[i]) { 
                //println!("Break ordering: {:?}", u); 
                invalid = true; 
                break; 
            }
        }

        let middle = u.len() / 2;

        if !invalid { result += u[middle]; }
    }
    println!("Result Part 1: {}", result);
}

pub fn solution(input: &String) {

    let lines: Vec<&str> = input.split("\n")
        .collect();

    let mut rules = Vec::<Rule>::new();
    let mut updates = Vec::<Vec<i32>>::new();
    
    let mut is_rules_done = false;

    for line in lines {
        
        if !is_rules_done {

            if line.is_empty() { is_rules_done = true; continue; }

            //println!("{line}");

            let parts: Vec<&str> = line.split("|")
                .collect();

            rules.push( Rule {
                before: parts[0].parse().unwrap(),
                after: parts[1].parse().unwrap()
            });
        }
        else {
            
            if line.is_empty() { break; }
           
            let mut pages = Vec::<i32>::new();
            
            let parts: Vec<&str> = line.split(",")
                .collect();

            for p in parts { pages.push(p.parse().unwrap()); }

            updates.push(pages);
            //println!("{}", line);
        }
    }

    part1(&rules, &updates);
    part2(&rules, &updates);

}
