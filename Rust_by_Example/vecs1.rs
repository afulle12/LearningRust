// vecs1.rs
//
// Your task is to create a `Vec` which holds the exact same elements as in the
// array `a`.
//
// Make me compile and pass the test!
//
// Execute `rustlings hint vecs1` or use the `hint` watch subcommand for a hint.

// I AM NOT DONE

fn main() -> ([i32; 4], Vec<i32>) {
    let a = [10, 20, 30, 40]; // a plain array
    let v = vec![1, 2, 3, 4];

    (a, v)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_and_vec_similarity() {
        let (a, v) = main();
        assert_eq!(a, v[..]);
    }
}

andrewfuller@pop-os:~/Tresors/Code/Rust/rustlings/exercises/05_vecs$ rustc vecs1.rs 
error[E0601]: `main` function not found in crate `vecs1`
  --> vecs1.rs:28:2
   |
28 | }
   |  ^ consider adding a `main` function to `vecs1.rs`

error: aborting due to previous error

For more information about this error, try `rustc --explain E0601`.
andrewfuller@pop-os:~/Tresors/Code/Rust/rustlings/exercises/05_vecs$ rustc vecs1.rs 
error[E0277]: `main` has invalid return type `([i32; 4], Vec<i32>)`
  --> vecs1.rs:12:14
   |
12 | fn main() -> ([i32; 4], Vec<i32>) {
   |              ^^^^^^^^^^^^^^^^^^^^ `main` can only return types that implement `Termination`
   |
   = help: consider using `()`, or a `Result`
   = help: the trait `Termination` is implemented for `()`

error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.

