use std::fs;

const FILE_PATH: &str = "./input.txt";

fn main(){
    
    match fs::read_to_string(&FILE_PATH){
        Ok(contents) => {
            let mut result = 0;
            for line in contents.lines(){
                let value = parse(&line);
                result += value;
            }
            println!("{}", result);
        }
        Err(err) => {
            eprintln!("Erreur de lecture du fichier : {}", err);
            std::process::exit(1);
        }
    }
   
}

fn parse(line :&str) -> i32{
    let line = line.split(":").collect::<Vec<_>>();
    let id: i32 = line[0].split("Game").collect::<Vec<_>>()[1].replace(" ", "").parse().unwrap();
    let sets = line[1].split(";").collect::<Vec<_>>();
    
    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;
    
    for set in sets{
        let set = set.split(",").collect::<Vec<_>>();
        for color in set{
            if color.contains("red") {
                let color_value = color.split("red").collect::<Vec<_>>()[0].replace(" ", "").parse().unwrap();
                red = if color_value > red {color_value} else {red};
            }
            if color.contains("green") {
                let color_value = color.split("green").collect::<Vec<_>>()[0].replace(" ", "").parse().unwrap();
                green = if color_value > green {color_value} else {green}; 
            }
            if color.contains("blue") {
                let color_value = color.split("blue").collect::<Vec<_>>()[0].replace(" ", "").parse().unwrap();
                blue = if color_value > blue {color_value} else {blue};
            }
        }
    }
    // part 1
    //let output: i32 = if red > 12 || green > 13 || blue > 14 {0} else{ id };
    
    //part 2
    let output: i32 = red * blue * green;
    
    output
}


