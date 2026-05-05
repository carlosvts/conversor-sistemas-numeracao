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

// daqui poderiamos, por exemplo, criar outras funcoes etc,

fn main() {
    println!("Hello World!");
}
