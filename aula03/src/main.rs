// Escreva um algoritmo em Rust para calcular a idade de alguém, 
// sabendo seu ano de nascimento
use std::io;

fn exercicio1() {
    println!("Digite seu ano de nascimento:");

    let mut buffer: String = String::new();
    io::stdin().read_line(&mut buffer).expect("Erro ao ler linha");

    // Remove quebra de linha recebida pelo usuário
    let ano_string = buffer.trim().to_string();

    // Converte ano em string para u16
    //      i -> Inteiros assinados
    //      u -> Inteiros não assinados
    let ano: u16 = ano_string.parse::<u16>().expect("Erro ao converter número");

    // Cálculo da idade
    let idade: u16 = 2026 - ano;

    println!("Você tem {idade} anos de idade");
}

fn exercicio2() {
    println!("Digite a quantidade de dias para alugar o carro: ");

    let mut buffer: String = String::new();
    io::stdin().read_line(&mut buffer).expect("Erro ao ler linha");

    let dias: u16 = buffer.trim().parse().expect("Erro ao converter linha");
    
    // dias = 32
    // dias_float = float(dias)
    let valor: f32 = dias as f32 * 100.00;

    println!("O valor total é {valor}");
}

fn main() {
    exercicio1();
    exercicio2();
}