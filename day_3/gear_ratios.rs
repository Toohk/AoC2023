use std::fs;

const FILE_PATH: &str = "./input.txt";

fn main(){
    
    match fs::read_to_string(&FILE_PATH){
        Ok(contents) => {
            
            part1(&contents);
            part2(&contents);
        }
        Err(err) => {
            eprintln!("Erreur de lecture du fichier : {}", err);
            std::process::exit(1);
        }
    }
   
}

fn colision(schematic: &Vec<Vec<char>>, x:usize, y:usize) -> bool{
    
    let mut is : bool = false;
    if x > 0 && schematic[x-1][y] != '.' && !schematic[x-1][y].is_digit(10)  {is = true}
    if x < 139 && schematic[x+1][y] != '.' && !schematic[x+1][y].is_digit(10) {is = true}
    if y > 0 && schematic[x][y-1] != '.' && !schematic[x][y-1].is_digit(10) {is = true}
    if y < 139 && schematic[x][y+1] != '.' && !schematic[x][y+1].is_digit(10) {is = true}
  
    if x > 0 && y < 139 && schematic[x-1][y+1] != '.' && !schematic[x-1][y+1].is_digit(10)  {is = true}
    if x > 0 && y > 0 && schematic[x-1][y-1] != '.' && !schematic[x-1][y-1].is_digit(10) {is = true}
    if x < 139 && y < 139 && schematic[x+1][y+1] != '.' && !schematic[x+1][y+1].is_digit(10)  {is = true}
    if x < 139 && y > 0 && schematic[x+1][y-1] != '.' && !schematic[x+1][y-1].is_digit(10) {is = true}
    
    is
}

fn numbers_colision(schematic: &Vec<Vec<char>>, x:usize, y:usize) -> Vec<[usize; 2]> {

    let mut coords = Vec::new();

    if y > 0 && schematic[x][y-1].is_digit(10) {coords.push([x, y-1])}
    if y < 139 && schematic[x][y+1].is_digit(10) {coords.push([x, y+1])}
  
    if x > 0 && y > 0 && schematic[x-1][y-1].is_digit(10) {coords.push([x-1, y-1])}
    if x > 0 && schematic[x-1][y].is_digit(10) {coords.push([x-1, y] )}
    if x > 0 && y < 139 && schematic[x-1][y+1].is_digit(10) {coords.push([x-1, y+1])}
    
    if x < 139 && y > 0 && schematic[x+1][y-1].is_digit(10) {coords.push([x+1, y-1])}
    if x < 139 && schematic[x+1][y].is_digit(10) {coords.push([x+1, y])}
    if x < 139 && y < 139 && schematic[x+1][y+1].is_digit(10)  {coords.push([x+1, y+1])}
    
    coords
}

fn part1(contents: &str){
   
    let mut result = 0;

    let schematic: Vec<Vec<char>> = contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();
   
    for (x , line) in schematic.iter().enumerate(){
        let mut pass = false;
        for (y , char) in line.iter().enumerate() {
            if !char.is_digit(10) {pass=false}
            if pass {continue}
            if char.is_digit(10){
                if colision(&schematic, x, y) {
                    pass = true;
                    let mut number = String::new();
                    let mut c = y;
                    while schematic[x][c].is_digit(10){
                        number = schematic[x][c].to_string() + &number;
                        if c > 0 {
                            c = c - 1;
                        }else{
                            break
                        }
                    }
                    c = y+1;
                    while schematic[x][c].is_digit(10){
                        number = number + &schematic[x][c].to_string();
                        if c < 139 {
                            c = c + 1;
                        }else{
                            break
                        }
                    }
                    result = &result + number.parse::<i32>().unwrap();
                        
                }else{
                    pass = false;
                }
            }
            
        }
    }

    println!("Part 1 result : {}", result);
}

fn part2(contents: &str){
    let mut result = 0;
    type Coord = [usize; 2];
    let schematic: Vec<Vec<char>> = contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();
   
    for (x , line) in schematic.iter().enumerate(){
        for (y , char) in line.iter().enumerate() {
            if *char == '*'{
                let  mut coords = numbers_colision(&schematic, x, y);
                let mut pass = false;
                let mut prev_coord: Coord = [0,0];
                println!("_{}", char);
                let mut numbers = Vec::new();
                for co in coords {
                    if prev_coord[1] == co[1]-1 {pass = true}else{pass=false}
                    prev_coord = co;
                    if pass {continue}
                    let mut number = String::new();
                    let mut c = co[1];
                    while schematic[co[0]][c].is_digit(10){
                        number = schematic[co[0]][c].to_string() + &number;
                        if c > 0 {
                            c = c - 1;
                        }else{
                            break
                        }
                    }
                    c = co[1]+1;
                    while schematic[co[0]][c].is_digit(10){
                        number = number + &schematic[co[0]][c].to_string();
                        if c < 139 {
                            c = c + 1;
                        }else{
                            break
                        }
                    }
                    println!("{}", number); 
                    numbers.push(number);
                }
                if numbers.len() == 2{
                    result = result + (numbers[0].parse::<i32>().unwrap() * numbers[1].parse::<i32>().unwrap());
                }
            }            
        }
    }


    println!("Part 2 result : {}", result);
}

