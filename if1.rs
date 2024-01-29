// if1.rs
//
// Execute `rustlings hint if1` or use the `hint` watch subcommand for a hint.

// I AM NOT DONE

pub fn bigger(a: i32, b: i32) -> i32 {
    // Complete this function to return the bigger number!
    // Do not use:
    // - another function call
    // - additional variables
    if a < b {
        b
    } else {
        a
    }
}

// Don't mind this for now :)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ten_is_bigger_than_eight() {
        assert_eq!(10, bigger(10, 8));
    }

    #[test]
    fn fortytwo_is_bigger_than_thirtytwo() {
        assert_eq!(42, bigger(32, 42));
    }

    #[test]
    fn equal_numbers() {
        assert_eq!(42, bigger(42, 42));
    }
}

fn main(){
    let _out: i32 = bigger(5i32,4i32);
    println!("{}", _out);
    //return _out; Can't return so print statement it is
}

//This is garbage, WTF is this
//Specify the return type or don't either way

/* andrew@framework-mint:~/Code/Rust/rustlings/exercises/03_if$ rustc if1.rs 
error[E0277]: `main` has invalid return type `i32`
  --> if1.rs:40:14
   |
40 | fn main() -> i32{
   |              ^^^ `main` can only return types that implement `Termination`
   |
   = help: consider using `()`, or a `Result`

error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
andrew@framework-mint:~/Code/Rust/rustlings/exercises/03_if$ rustc if1.rs 
error[E0308]: mismatched types
  --> if1.rs:43:12
   |
40 | fn main(){
   |          - expected `()` because of default return type
...
43 |     return _out;
   |            ^^^^ expected `()`, found `i32`

error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`. */
