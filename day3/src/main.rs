use regex::Regex;
use day3::readline;

fn is_email(s: &str) -> bool{
    let re =  Regex::new(r"^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+$").unwrap();
    return re.is_match(s);
}

fn main() {
    let email = readline("Escreva seu email: ");
    let regx_email = is_email(&email);
    println!("is email: {}", regx_email);
}
