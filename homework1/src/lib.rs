use std::char::from_digit;

#[cfg(test)]
mod tests {
    #[test]
    fn test_num_to_string() {
        assert_eq!("5", super::num_to_string(5, 10));
        assert_eq!("12", super::num_to_string(12, 10));
        assert_eq!("c", super::num_to_string(12, 16));
        assert_eq!("10", super::num_to_string(16, 16));
        assert_eq!("12", super::num_to_string(10, 8));
    }

    #[test]
    fn test_correct_input() {
        assert!(!super::correct_input("80", 8));
        assert!(super::correct_input("0", 8));
        assert!(super::correct_input("101", 2));
        assert!(super::correct_input("0111", 2));
        assert!(super::correct_input("7b", 16));
        assert!(!super::correct_input("7B", 16));
        assert!(super::correct_input("9989", 10));
        assert!(!super::correct_input("9a", 10));
        assert!(!super::correct_input("g", 16));
        assert!(!super::correct_input("А", 16))
    }
    
}

//Ф-ия намираща dr по зададен низ и бройна с-ма
fn digital_root(string: &str, base: u32) -> char {
    let length = string.len();
    if length == 1 {
        return string.chars().next().unwrap();
    }

    let mut sum = 0;
    for ch in string.chars() {
        let digit = ch.to_digit(base);
        debug_assert!(digit.is_some());
        sum += digit.unwrap();
    }
    let sum_in_string = num_to_string(sum, base);
    return digital_root(&sum_in_string, base);
}

fn num_to_string(num: u32, base: u32) -> String {
    let mut result_string = String::from("");
    let mut number = num;
    while number > 0 {
        let number_in_char = from_digit(number % base, base); 
        debug_assert!(number_in_char.is_some());
        let num_in_char = number_in_char.unwrap();
        result_string.insert(0, num_in_char);
        number /= base;
    }
    result_string
} 
fn incorrect_char(ch : char, base: u32) -> bool {
    !ch.is_digit(base) || (ch >= 'A' && ch <= 'Z')
}

fn correct_input(string: &str, base:u32) -> bool {
    let mut result = true;
    for ch in string.chars() {
       if incorrect_char(ch, base) {
           result = false;
           break;
        }
   }
   result
}

/// Десетична бройна система: 0-9
pub fn decimal(input: &str) -> Option<u32> {
    if !correct_input(input, 10) {
        return None;
    }
    let dr = digital_root(input, 10);
    dr.to_digit(10)
}

/// Шестнадесетична бройна система: 0-9, последвано от a-f
pub fn hex(input: &str) -> Option<u32> {
    if !correct_input(input, 16) {
        return None;
    }
    let dr = digital_root(input, 16);
    dr.to_digit(16)
}

/// Осмична бройна система: 0-7
pub fn octal(input: &str) -> Option<u32> {
    if !correct_input(input, 8) {
        return None;
    }
    let dr = digital_root(input, 8);
    dr.to_digit(8)
}

/// Двоична бройна система: 0-1
pub fn binary(input: &str) -> Option<u32> {
    if !correct_input(input, 2) {
        return None;
    }
    let dr = digital_root(input, 2);
    dr.to_digit(2)
}