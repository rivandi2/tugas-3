//Soal 3. Tuple
fn rgb_code (e: &str) -> Result<(i32, i32, i32, i32), String>{
    match e {
        "red" => Ok((255,0,0,1)),
        "green" => Ok((0,255,0,1)),
        "blue" => Ok((0,0,255,1)),
        "yellow" => Ok((255,255,0,1)),
        "cyan" => Ok((0,255,255,1)),
        "magenta" => Ok((255,0,255,1)),
        "black" => Ok((0,0,0,1)),
        "white" => Ok((0,0,0,0)),
        "purple" => Ok((128,0,128,1)),
        "orange" => Ok((255,165,0,1)),
        _ => Err("Color not found!".to_string())
    }
}

fn main() {
    let color = "cyan".to_string();
    let colorcheck = rgb_code(&color.to_lowercase());
    
    match colorcheck {
        Ok(content) => println!("r: {:?}, g: {:?}, b: {:?}, a: {:?}", content.0, 

content.1, content.2, content.3),
        Err(e) => println!("{:?}", e),
    };
}