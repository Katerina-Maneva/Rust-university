use std::io;

pub fn prompt_input(input: &str) -> String {
    println!("{}",input);
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_goes_into_input_above) => {},
        Err(_no_updates_is_fine) => {},
    }
    input.chars().filter(|c| c.is_alphabetic() || *c == '-').collect()
}

pub fn prompt_char(input: &str) -> Option<char> {
    let input = prompt_input(input);
    input.chars().next() 
}