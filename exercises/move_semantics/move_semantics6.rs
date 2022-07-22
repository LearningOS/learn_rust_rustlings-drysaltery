// move_semantics6.rs
// Make me compile! `rustlings hint move_semantics6` for hints
// You can't change anything except adding or removing references


fn main() {
    let data = "Rust is great!".to_string();
    
    //66
    get_char(data.clone());

    string_uppercase(&data);
}

// Should not take ownership
fn get_char(data: String) -> char {
    data.chars().last().unwrap()
}

// Should take ownership
fn string_uppercase(mut data: &String) {
    //data = &data.to_uppercase();

    //println!("{}", data);
    //66 
    let data1 = &data.to_uppercase();

    println!("{}", data1);

}
