fn main() {
    // Integer addition
    println!("1 + 2 = {}", 1i32 + 2);

    // Integer subtraction
    println!("1 - 2 = {}", 1i32 - 2);
    // TODO ^ Try changing `1i32` to `1u32` to see why the type is important
    //Unsigned can only be positive
    //Signed numbers can be negative and positive

    // Short-circuiting boolean logic
    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);

    // Bitwise operations
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("1100 XOR 1010 is {:04b}", 0b1100u32 ^ 0b1010);
    //XOR 1 in only 1 gives you one 1100 XOR 1010 = 0110
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

    // Use underscores to improve readability!
    println!("One million is written as {}", 1_____000000u32);
    //Underscores are ignored
}
