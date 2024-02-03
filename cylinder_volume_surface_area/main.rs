use std::io;

fn main(){
    let mut height = String::new();
    let mut radius = String::new();
    let mut radius_int = 0;
    let mut height_int = 0;
    println!("What is the height of the cylinder?");
    io::stdin().read_line(&mut height).expect("Invalid input");
    println!("What is the radius of the cylinder?");
    io::stdin().read_line(&mut radius).expect("Invalid input");
    height_int = height.parse::<i32>().unwrap();
    radius_int = radius.parse::<i32>().unwrap();
    println!("Height: {}", height_int.to_string());
    println!("Radius: {}", radius_int.to_string());
    //println!("Radius: {}", radius_int.trim().parse().expect("Couldn't parse!"));
}