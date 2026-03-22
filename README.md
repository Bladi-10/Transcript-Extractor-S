# Transcript Extractor for YouTube (Rust)

Este proyecto es una herramienta de línea de comandos (CLI) escrita en Rust diseñada para extraer y limpiar las transcripciones de videos de YouTube de forma eficiente.

## Funcionalidades

- Descarga subtítulos automáticos o manuales usando `yt-dlp`.
- Limpia el formato VTT eliminando marcas de tiempo, etiquetas HTML y metadatos.
- Elimina líneas duplicadas consecutivas para una lectura fluida.
- Genera un archivo de texto plano (`.txt`) con la transcripción procesada.

## Requisitos Previos

Para ejecutar este proyecto, necesitas tener instalado:

1. **Rust**: [Instalar Rust](https://www.rust-lang.org/tools/install)
2. **yt-dlp**: Herramienta externa necesaria para la descarga de subtítulos.
   - En Linux: `sudo curl -L https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp -o /usr/local/bin/yt-dlp && sudo chmod a+rx /usr/local/bin/yt-dlp`
   - O vía pip: `pip install yt-dlp`

## Instalación y Ejecución

1. Clona el repositorio:
   ```bash
   git clone https://github.com/tu-usuario/transcript-extractor-rs.git
   cd transcript-extractor-s
   ```
2. Compila y ejecuta:
   ```bash
   cargo run
   ```
3. Introduce la URL del video de YouTube cuando se te solicite.

## Detalles Técnicos

- **Nombre del Paquete:** transcript-extractor-s
- **Versión:** 0.1.0
- **Licencia:** GPL-3.0-or-later (Open Source)
- **Dependencias:**
  - `regex`: Para la limpieza avanzada de etiquetas y filtrado de texto.

---

## ⚖️ Legal Disclaimer / Aviso Legal

### English
This project is an **unofficial tool** and is not affiliated with, endorsed by, or in any way officially connected with **YouTube**, **Google LLC**, or any of its subsidiaries. 

The tool is provided for educational and personal use only. Users are responsible for complying with YouTube's [Terms of Service](https://www.youtube.com/t/terms) and respecting the intellectual property rights of content creators. This project does not circumvent any technological protection measures.

### Español
Este proyecto es una **herramienta no oficial** y no está afiliada, respaldada ni conectada de ninguna manera con **YouTube**, **Google LLC** ni ninguna de sus subsidiarias.

La herramienta se proporciona únicamente para uso educativo y personal. Los usuarios son responsables de cumplir con los [Términos de Servicio de YouTube](https://www.youtube.com/t/terms) y de respetar los derechos de propiedad intelectual de los creadores de contenido. Este proyecto no elude ninguna medida tecnológica de protección.
