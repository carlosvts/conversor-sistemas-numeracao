// Definir as structs aqui e fazer a lógica de backend aqui

// Para rodar somente esse código como forma de debug, só usar
// cargo run -bin --core-debug

use core::num;
use crossterm::style::Stylize;
use csv::Reader;
// para ler csv
use std::collections::HashMap; // para mapear hex
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

fn bin_to_hex(mut bin: String) -> String {
    let mut hex: String = String::new();
    let mut chunks: Vec<String> = Vec::new(); // guarda os grupos de binario 
    let mut bin_to_hex: HashMap<String, &str> = HashMap::new();
    bin_to_hex.insert("0000".to_string(), "0");
    bin_to_hex.insert("0001".to_string(), "1");
    bin_to_hex.insert("0010".to_string(), "2");
    bin_to_hex.insert("0011".to_string(), "3");
    bin_to_hex.insert("0100".to_string(), "4");
    bin_to_hex.insert("0101".to_string(), "5");
    bin_to_hex.insert("0110".to_string(), "6");
    bin_to_hex.insert("0111".to_string(), "7");
    bin_to_hex.insert("1000".to_string(), "8");
    bin_to_hex.insert("1001".to_string(), "9");
    bin_to_hex.insert("1010".to_string(), "A");
    bin_to_hex.insert("1011".to_string(), "B");
    bin_to_hex.insert("1100".to_string(), "C");
    bin_to_hex.insert("1101".to_string(), "D");
    bin_to_hex.insert("1110".to_string(), "E");
    bin_to_hex.insert("1111".to_string(), "F");
    let mut count: u16 = 0;
    //println!("bin recebido: {}", bin);
    // padding para 16 bits
    // adiciona um padding
    let binary_lenght = bin.chars().count();
    if binary_lenght < 8 {
        let num_padding = 8 - binary_lenght;
        //println!("num_padding {}", num_padding);
        for _ in 0..num_padding {
            bin.insert(0, '0');
        }
    }
    //println!("bin com padding {}", bin);

    for b in bin.chars() {
        hex.push(b);
        count += 1;
        if count == 4 {
            chunks.push(hex.clone());
            hex.clear();
            count = 0;
        }
    }

    if hex.len() >= 1 {
        chunks.push(hex.clone());
    }
    // limpa o buffer do hex
    hex.clear();

    for chunk in chunks.iter_mut().rev() {
        // adiciona um padding
        if chunk.len() < 4 {
            let num_padding = 4 - chunk.len();
            //println!("num_padding {}", num_padding);
            for _ in 0..num_padding {
                chunk.insert(0, '0');
                //println!("chunk padding --> {}", chunk);
            }
        }
        //println!("chunk {}", chunk);
        match bin_to_hex.get(chunk) {
            Some(value) => hex.push_str(value),
            None => (),
        }
    }

    //println!("hex: {}", hex);
    hex
}

fn main() {
    // debug
    println!("maximum: {}", maximum(2, 3));
    // 12 em binario é 1100 --> octal vira 14, tudo certo
    println!("bin_to_oct: {}", bin_to_oct("1100".to_string()));
    println!("bin_to_hex: {}", bin_to_hex("111111111".to_string()));
}
