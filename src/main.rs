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

// static VARIAVEL_STATIC: i32 = 10;
// const CONSTANTE: i32 = 20;

// fn main() {
//     menu()
// }

// fn main() {

//     let x: i32 = 40;
//     let mut y: i32 = x;

//     println!("==========================");
//     println!("O valor de x é: {} - referencia {:p}", x, &x);
//     println!("O valor de y é: {} - refremcoa {:p}", y, &y);
//     println!("==========================");

//     y = 50;

//     println!("==========================");
//     println!("O valor de x é: {} - referencia {:p}", x, &x);
//     println!("O valor de y é: {} - refremcoa {:p}", y, &y);
//     println!("==========================");

// }

// fn main() {

//     let x: i32 = 10;
//     let y: &i32 = &x;

//     println!("==========================");
//     println!("O valor de x é: {} - referencia {:p}", x, &x);
//     println!("O valor de y é: {} - refremcoa", y);
//     println!("==========================");

// }
// fn main() {
//     // Declarando uma variável inteira x
//     let x: i32 = 10;

//     // Criando uma referência para x
//     let y: &i32 = &x; 

//     println!("O valor de x é: {} - endereço de x: {:p}", x, &x);
//     println!("O valor de y (referência para x) é: {} - endereço de y: {:p}", y, &y);

//     // Criando uma nova referência t, que aponta para o mesmo valor que y (ou seja, x)
//     let t: &i32 = y;

//     println!("O valor de t (referência para x via y) é: {} - endereço de t: {:p}", t, &t);

//     // Desreferenciando y para obter o valor de x e armazenando em w
//     let w: i32 = *y;

//     println!("O valor de w (valor de x via desreferência de y) é: {} - endereço de w: {:p}", w, &w);
// }


// fn main() {
//    let mut x: i32 = 10;
//    let y: &i32 = &x; // Criando uma referência para x
   
//     println!("O valor de x é: {} - endereço de x: {:p}", x, &x);
//     println!("O valor de y (referência para x) é: {} - endereço de y: {:p}", y, &y);

//     x = 20; // Alterando o valor de x

//     println!("Após alterar x:");
//     println!("O valor de x é: {} - endereço de x: {:p}", x, &x);
//     // println!("O valor de y (referência para x) ainda é: {} - endereço de y: {:p}", y, &y);

// }

fn main() {
    let x:i32 = 10;
    let y: &i32 = &x; // Criando uma referência para x

    imprime_valor(&x);
    imprime_valor(&y);
}

fn imprime_valor(valor: &i32) {
    println!("O valor é: {} - endereço: {:p}", valor, valor);
}