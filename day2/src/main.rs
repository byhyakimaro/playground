struct Pessoa {
    nome: String,
    idade: u8,
}

impl Pessoa {
    fn maior_idade(p: &Pessoa) {
        match p.idade {
            0..=17 => println!("{} é menor de idade.", p.nome),
            18..=u8::MAX => println!("{} é maior de idade.", p.nome),
        }
    }
}

fn main() {
    let mut saldo: i16 = 0;
    saldo += 126;
    let p = Pessoa {
        nome: "ana".to_string(),
        idade: 18
    };
    Pessoa::maior_idade(&p);
    println!("{} tem {} anos", p.nome, p.idade);
    println!("contador: {}", saldo);
}
