// move_semantics6.rs
//
// You can't change anything except adding or removing references.
//
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand
// for a hint.

// I AM NOT DONE

fn main() {
    let data = "Rust is great!".to_string();

    let last_char = get_char(&data); // Pass a reference to avoid transferring ownership
    println!("Last char: {}", last_char);

    string_uppercase(data); // Transfer ownership
}

// Should not take ownership, so take a reference
fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}

// Should take ownership
fn string_uppercase(data: String) {
    let uppercased_data = data.to_uppercase();
    println!("{}", uppercased_data);
}
