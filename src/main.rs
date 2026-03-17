mod check;
use check::check_valid;

fn main() {
    println!("Hello, world!");

    let codes = vec![
        "b", "c", "by", "i", 
        "l", "q", "ub", "ui", 
        "ul", "uq", "f", "d", 
        "s", "byt", "a", "dy",
        "ad", "p", "rc", "w",
        "un", "r", "re", "t",
        "te", "o", "v", "fu"
    ];

    check_valid(&codes);
}
