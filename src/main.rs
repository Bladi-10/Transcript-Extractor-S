use regex::Regex;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Write};
use std::process::Command;

fn main() -> io::Result<()> {
    // 1. Pedir la URL al usuario desde la terminal
    print!("Introduce el enlace del video de YouTube: ");
    io::stdout().flush()?; // Asegura que el texto se muestre antes de pedir el input
    
    let mut url = String::new();
    io::stdin().read_line(&mut url)?;
    let url = url.trim(); // Quitar saltos de línea y espacios

    if url.is_empty() {
        println!("Error: La URL no puede estar vacía.");
        return Ok(());
    }

    println!("Descargando subtítulos con yt-dlp... (esto puede tomar unos segundos)");

    // 2. Ejecutar yt-dlp desde Rust
    // Usamos "-o temp_sub" para forzar el nombre del archivo de salida
    let status = Command::new("yt-dlp")
        .args([
            "--write-sub",
            "--write-auto-sub",
            "--sub-lang", "es",
            "--skip-download",
            "-o", "temp_sub",
            url,
        ])
        .status()?;

    if !status.success() {
        eprintln!("Error al descargar. Verifica el enlace, tu conexión, o si el video tiene subtítulos.");
        return Ok(());
    }

    // yt-dlp agregará automáticamente la extensión .es.vtt
    let input_path = "temp_sub.es.vtt";
    let output_path = "transcripcion_final.txt";

    // 3. Procesar y limpiar el archivo (la lógica que ya estudiamos)
    let file = match File::open(input_path) {
        Ok(f) => f,
        Err(_) => {
            eprintln!("No se encontró el archivo temporal. Es posible que el video no tenga subtítulos.");
            return Ok(());
        }
    };
    
    let reader = BufReader::new(file);
    let mut output = File::create(output_path)?;
    let tag_re = Regex::new(r"<[^>]+>").unwrap();
    let mut last_line = String::new();

    println!("Limpiando y estructurando el texto...");

    for line_result in reader.lines() {
        let line = line_result?;
        let trimmed = line.trim();

        if trimmed.is_empty()
            || trimmed.contains("-->")
            || trimmed.starts_with("WEBVTT")
            || trimmed.starts_with("Kind:")
            || trimmed.starts_with("Language:")
            || trimmed.starts_with("Style:")
        {
            continue;
        }

        let cleaned_line = tag_re.replace_all(trimmed, "");
        let final_line = cleaned_line.trim();

        if !final_line.is_empty() && final_line != last_line {
            writeln!(output, "{}", final_line)?;
            last_line = final_line.to_string();
        }
    }

    // 4. Limpieza del sistema: Borrar el archivo .vtt original para no dejar basura
    fs::remove_file(input_path)?;

    println!("¡Éxito! La transcripción limpia se ha guardado en: {}", output_path);

    Ok(())
}