use solution::decimal;
use solution::hex;
use solution::octal;
use solution::binary;


#[test]
fn test_basic() {
    assert_eq!(decimal("345"), Some(3));
    assert_eq!(hex("345"), Some(0xc));

    assert_eq!(octal("1"), Some(1));
    assert_eq!(binary("1"), Some(1));

    let num = String::from("1");
    assert_eq!(binary(&num[..]), Some(1));
}
#[test]
fn test_hex_upper_case() {
    assert_eq!(None, hex("3B"));
    assert_eq!(Some(14), hex("3b"));
    assert_eq!(Some(3), hex("7b"));
    assert_eq!(None, hex("aC"));
}