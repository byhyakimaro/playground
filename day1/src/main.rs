use day1::{info_int, readline};

fn main() {
    let name: &str = "Paulo";
    println!("Òla, {}", name);

    let altura: u16 = readline("Qual sua altura em centimetros?")
        .trim() // remove caracteres invisiveis tipo \n
        .parse() // tenta converter para o tipo
        .expect("Digite um número válido para altura"); // retorna um erro caso nao consiga
    println!("sua altura em centrimetros é {} ", altura);
    info_int(altura);
}
