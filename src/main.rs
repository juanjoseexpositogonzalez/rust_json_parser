use std::fs;
use std::path::Path;

use rust_json_parser::{parse_json, tokenize, JsonError};

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

    // Intentar parsear a JsonValue
    match parse_json(&contents) {
        Ok(value) => {
            println!("\n✅ Parsed JsonValue:\n{:?}", value);
        }
        Err(err) => {
            eprintln!("\n⚠️  Error al parsear JSON:");
            // Desglosar si es posible para dar más contexto
            match err {
                JsonError::UnexpectedToken { expected, found, position } => {
                    eprintln!("  Tipo: UnexpectedToken");
                    eprintln!("  Esperado: {}", expected);
                    eprintln!("  Encontrado: {}", found);
                    eprintln!("  Posición: {}", position);
                }
                JsonError::UnexpectedEndOfInput { expected, position } => {
                    eprintln!("  Tipo: UnexpectedEndOfInput");
                    eprintln!("  Esperado: {}", expected);
                    eprintln!("  Posición: {}", position);
                }
                JsonError::InvalidNumber { value, position } => {
                    eprintln!("  Tipo: InvalidNumber");
                    eprintln!("  Valor: {}", value);
                    eprintln!("  Posición: {}", position);
                }
            }
        }
    }
}

