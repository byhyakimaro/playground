use std::{io,io::Write};

pub fn readline(s: &str) -> String {
    print!("- {}", s);
    io::stdout().flush().unwrap();
    let mut text_str = String::new();
    io::stdin()
        .read_line(&mut text_str)
        .expect("Erro ao ler entrada!");

    return text_str.trim().to_string();
}
