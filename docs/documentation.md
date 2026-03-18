# Documentación del Código

Este documento explica el funcionamiento interno del extractor de transcripciones.

## Estructura del Proceso

El código se divide en cuatro etapas principales:

### 1. Entrada de Usuario
El programa solicita al usuario una URL de YouTube a través de la entrada estándar (`stdin`). Valida que la URL no esté vacía antes de continuar.

### 2. Interacción con yt-dlp
Utiliza `std::process::Command` para ejecutar `yt-dlp` como un proceso externo. Los argumentos configurados son:
- `--write-sub` y `--write-auto-sub`: Para obtener tanto subtítulos creados por humanos como automáticos.
- `--sub-lang es`: Prioriza el idioma español.
- `--skip-download`: Evita descargar el video (solo nos interesan las letras).
- `-o temp_sub`: Fuerza un nombre de archivo predecible para facilitar su manipulación posterior.

### 3. Procesamiento y Limpieza (Lógica Central)
Una vez que `yt-dlp` genera el archivo `.es.vtt`, el programa lo abre y lo procesa línea por línea utilizando un `BufReader`.

Se aplican los siguientes filtros:
- **Saltar metadatos:** Se ignoran líneas que contienen `WEBVTT`, `Kind:`, `Language:`, `Style:` o marcas de tiempo (`-->`).
- **Limpieza de Regex:** Se utiliza la expresión regular `<[^>]+>` para eliminar etiquetas de estilo (como las que definen colores o posiciones en el archivo original).
- **Deduplicación:** El formato VTT de YouTube suele repetir palabras línea tras línea para simular el desplazamiento. El código guarda la `last_line` y solo escribe en el archivo de salida si la línea actual es diferente a la anterior.

### 4. Finalización
- El texto limpio se escribe en `transcripcion_final.txt`.
- Se utiliza `fs::remove_file` para borrar el archivo temporal `.vtt` y mantener limpio el directorio de trabajo.

## Manejo de Errores
El programa utiliza el sistema de `Result` de Rust para manejar fallos comunes:
- Errores de red o de `yt-dlp`.
- Ausencia de subtítulos en el video solicitado.
- Permisos de escritura en el disco.
