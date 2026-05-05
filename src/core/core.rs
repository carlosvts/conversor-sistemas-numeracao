// Definir as structs aqui e fazer a lógica de backend aqui

// Para rodar somente esse código como forma de debug, só usar
// cargo run -bin --core-debug

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

// daqui poderiamos, por exemplo, criar outras funcoes etc,

fn main() {
    // debug
    println!("{}", maximum(2, 3));
}
