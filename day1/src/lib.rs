use std::io;
use std::mem;

pub fn info_int(numero: u16) {
    println!("seu numero é {}", numero);
    println!(
        "u16 vai de {} até {}, e tem {} bytes, ele e 16², o i16 aceita de {} ate {}",
        u16::MIN,
        u16::MAX,
        mem::size_of::<u16>(),
        i16::MIN,
        i16::MAX,
    );
}

pub fn readline(text: &str) -> String {
    println!("{}", text);
    let mut text_str = String::new();
    io::stdin()
        .read_line(&mut text_str)
        .expect("Erro ao ler entrada!");

    return text_str;
}
