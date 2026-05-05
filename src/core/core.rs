// Definir as structs aqui e fazer a lógica de backend aqui

// Para rodar somente esse código como forma de debug, só usar
// cargo run -bin --core-debug

use crossterm::style::Stylize;
use csv::Reader;
use std::fs::File; //std filestream file 

// Recebe um path de arquivo, retorna um reader de CSV (um iterador)
#[warn(dead_code)] // coloquei a flag pro compilador nao reclamar
fn from_csv(path: String) -> Result<Reader<File>, csv::Error> {
    let reader: Reader<File> = Reader::from_path(path)?;
    Ok(reader)
}

// Função calcula o máximo de um sistema de certa base e de certa qtd de digitos
fn maximum(base: u32, num_digits: u32) -> String {
    let max = base.pow(num_digits) - 1;
    let suffix = format!("{base}^{num_digits} - 1 = ");
    let response = suffix + &max.to_string();
    response
}

fn bin_to_oct(bin: String) -> String {
    // para octal agrupamos de 3 em 3, portanto
    let mut oct: String = String::new();
    // len retorna o numero de bytes, chars cria um iterador com todos os chars, count conta
    // quantos termos até ele achar None
    let mut count: u8 = 0;
    let mut group: u8 = 0;

    for b in bin.as_bytes().iter().rev() {
        // bit 0 ou 1
        let bit = b - b'0';
        group |= bit << count;
        count += 1;
        if count == 3 {
            // salva a resposta na string de output
            oct.push_str(&group.to_string());
            // reseta os contadores
            count = 0;
            group = 0;
        }
    }

    // adiciona um padding
    if count > 0 {
        oct.push((group + b'0') as char);
    }

    // Revertemos la em cima o 1100 para 0011, entao revertemos novamente
    oct.chars().rev().collect()
}

// daqui poderiamos, por exemplo, criar outras funcoes etc,

fn main() {
    // debug
    println!("{}", maximum(2, 3));
    // 12 em binario é 1100 --> octal vira 14, tudo certo
    println!("{}", bin_to_oct("1100".to_string()));
}
