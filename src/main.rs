use std::{io};

fn soma_entre_valores(x: i32, y: i32) -> i32 {
    let resultado: i32 = x + y;
    resultado
}
fn subtracao_entre_valores(x: i32, y: i32) -> i32 {
    let resultado: i32 = x - y;
    resultado
}

fn solicita_tabuada() {
    let mut numero = String::new();
    println!("Digite um número para ver a tabuada:");
    io::stdin()
        .read_line(&mut numero)
        .expect("Falha ao ler linha");
    
    let numero: i32 = numero.trim().parse().expect("Por favor, digite um número válido");
    
    for i in 1..=10 {
        println!("{} x {} = {}", numero, i, numero * i);
    }
    
}

fn solicita_soma_entre_valores(soma: bool) {
    let mut x = String::new();
    let mut y = String::new();  

    println!("Digite o primeiro valor:");
    io::stdin()
        .read_line(&mut x)
        .expect("Falha ao ler linha");

    let x: i32 = x.trim().parse().expect("Por favor, digite um número válido");


    println!("Digite o segundo valor:");
    io::stdin()
        .read_line(&mut y)
        .expect("Falha ao ler linha");

    let y: i32 = y.trim().parse().expect("Por favor, digite um número válido");

    if soma == true {
        println!("A soma de {} e {} é: {}", x, y, soma_entre_valores(x, y));
    } else {
        println!("A subtração de {} e {} é: {}", x, y, subtracao_entre_valores(x, y));
    }
}

fn menu() {
    loop {
        println!("Digite uma opção abaixo:");
        println!("
        1) Soma
        2) Subtração
        3) Tabuada De Um Número
        0) Sair
        ");

        let mut opcao = String::new();
        io::stdin()
            .read_line(&mut opcao)
            .expect("Falha ao ler linha");

        let opcao: i32 = opcao.trim().parse().expect("Por favor, digite um número válido");

        match opcao {
            1 => solicita_soma_entre_valores(true),
            2 => solicita_soma_entre_valores(false),
            3 => solicita_tabuada(),
            0 => break,
            _ => println!("Opção inválida, tente novamente."),
        }
    }
}

static VARIAVEL_STATIC: i32 = 10;

fn main() {
    menu()
}
