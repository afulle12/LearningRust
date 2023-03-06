fn main() {
    let parsed: i32 = "5".parse().unwrap();
    //let turbo_parsed = "10".parse::<i32>().unwrap();
    let turbo_parsed: i32 = "10".parse().unwrap();

    let sum = parsed + turbo_parsed;
    println!("Sum: {:?}", sum);
}
