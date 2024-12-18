// src/main.rs
pub mod types;
pub mod parser;
pub mod generator;
mod tokenizer;

use std::env;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Uso: compiler.exe <archivo.py>");
        return;
    }
    let input_path = &args[1];
    match compile_python(input_path) {
        Ok(output_path) => println!("Éxito! El ejecutable está en: {}", output_path),
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn compile_python(input_path: &str) -> Result<String, String> {
    let python_code = fs::read_to_string(input_path)
        .map_err(|e| format!("Error al leer el archivo Python: {}", e))?;

    let tokens = tokenizer::tokenize(&python_code);
    let state = parser::extract_state(tokens);
    let rust_code = generator::generate_rust(&state);

    // Obtener el nombre base del archivo
    let output_path = Path::new(input_path)
        .file_stem()
        .ok_or("Error al extraer el nombre del archivo base")?
        .to_string_lossy()
        .into_owned();

    // Crear el directorio del proyecto
    fs::create_dir_all(&output_path)
        .map_err(|e| format!("Error al crear el directorio del proyecto: {}", e))?;

    // Crear el Cargo.toml
    let cargo_toml = format!(r#"[package]
name = "{}"
version = "0.1.0"
edition = "2021"

[dependencies]
sdl2 = {{ version = "0.35", features = ["bundled"] }}
"#, output_path);

    let cargo_path = Path::new(&output_path).join("Cargo.toml");
    let mut cargo_file = fs::File::create(&cargo_path)
        .map_err(|e| format!("Error al crear Cargo.toml: {}", e))?;
    cargo_file.write_all(cargo_toml.as_bytes())
        .map_err(|e| format!("Error al escribir Cargo.toml: {}", e))?;

    // Crear el directorio src y el archivo main.rs
    let src_dir = Path::new(&output_path).join("src");
    fs::create_dir_all(&src_dir)
        .map_err(|e| format!("Error al crear el directorio src: {}", e))?;

    let rust_path = src_dir.join("main.rs");
    let mut rust_file = fs::File::create(&rust_path)
        .map_err(|e| format!("Error al crear main.rs: {}", e))?;
    rust_file.write_all(rust_code.as_bytes())
        .map_err(|e| format!("Error al escribir código Rust: {}", e))?;

    // Compilar el proyecto usando cargo
    let status = if cfg!(target_os = "windows") {
        Command::new("cargo")
            .current_dir(&output_path)
            .args(["build", "--release"])
            .status()
            .map_err(|e| format!("Error al ejecutar cargo: {}", e))?
    } else {
        Command::new("cargo")
            .current_dir(&output_path)
            .args(["build", "--release"])
            .status()
            .map_err(|e| format!("Error al ejecutar cargo: {}", e))?
    };

    if !status.success() {
        return Err("Error durante la compilación con cargo".to_string());
    }

    // La ubicación del ejecutable compilado
    let exe_extension = if cfg!(target_os = "windows") { ".exe" } else { "" };
    let exe_path = Path::new(&output_path)
        .join("target")
        .join("release")
        .join(format!("{}{}", output_path, exe_extension));

    Ok(exe_path.to_string_lossy().into_owned())
}