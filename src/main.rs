use std::fs;
use std::path::Path;

use rust_json_parser::{tokenize};

fn main() {
    // Ruta del archivo JSON de prueba
    let path = Path::new("test_data/example.json");

    // Leer el archivo completo a un String
    let contents = match fs::read_to_string(path) {
        Ok(text) => text,
        Err(err) => {
            eprintln!("❌ Error reading {:?}: {}", path, err);
            return;
        }
    };

    println!("📄 Input JSON:\n{}\n", contents);

    // Tokenizar el contenido
    let tokens = tokenize(&contents);

    println!("🔍 Tokens found ({}):", tokens.len());
    for (i, token) in tokens.iter().enumerate() {
        println!("{:>3}: {:?}", i, token);
    }

    println!("\n✅ Tokenization process complete.");
}
