// move_semantics1.rs
//
// Execute `rustlings hint move_semantics1` or use the `hint` watch subcommand
// for a hint.

// I AM NOT DONE

fn main() {
    // Create an empty vector
    let vec0 = Vec::new();

    // Pass the ownership of vec0 to fill_vec and get a new vector with added values
    let mut vec1 = fill_vec(vec0);

    // Print the contents of vec1
    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    // Modify vec1 by pushing a new value
    vec1.push(88);

    // Print the modified vec1
    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

// A function that takes a vector, modifies it, and returns the modified vector
fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}
