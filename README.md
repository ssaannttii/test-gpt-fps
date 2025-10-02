# PiperDesk

PiperDesk es un MVP de escritorio construido con **Tauri + Svelte** que permite importar documentos, generar una cola de lectura y sintetizar audio con el motor [Piper TTS](https://github.com/rhasspy/piper). La aplicación incluye controles de reproducción básicos, selector de voz y velocidad, exportación a WAV/MP3 y automatizaciones de QA para Rust, frontend y scripts de importación.

## Tabla de contenidos

- [Características](#características)
- [Requisitos](#requisitos)
- [Instalación](#instalación)
- [Configuración de Piper](#configuración-de-piper)
- [Ejecución](#ejecución)
- [Uso](#uso)
  - [Importar documentos](#importar-documentos)
  - [Cola de lectura y síntesis](#cola-de-lectura-y-síntesis)
  - [Exportación de audio](#exportación-de-audio)
- [Automatización y QA](#automatización-y-qa)
- [Estructura del repositorio](#estructura-del-repositorio)
- [Contribuir](#contribuir)
- [Licencia](#licencia)

## Características

- Interfaz Svelte responsiva integrada en Tauri.
- Importadores de EPUB/PDF/TXT mediante scripts Python.
- Cola de lectura con estados de progreso y eventos en vivo.
- Selector de voz, control de velocidad y directorio de exportación configurable.
- Exportación a WAV o MP3 usando Piper + FFmpeg.
- Pruebas unitarias en Rust, Vitest para el frontend y Pytest para los scripts.
- Workflow de GitHub Actions para ejecutar linting y suites de pruebas.

## Requisitos

| Herramienta | Versión sugerida |
|-------------|------------------|
| Rust | 1.72+ (instalado con `rustup`) |
| Node.js | 18+ |
| pnpm | 8+ |
| Python | 3.10+ |
| FFmpeg | 4+ |
| Piper TTS | Binario `piper` accesible en `$PATH` |

Adicionalmente, los scripts Python necesitan `pypdf` para la extracción de PDF. Puedes instalar dependencias opcionales con:

```bash
python -m pip install -r scripts/requirements.txt
```

## Instalación

1. Clona este repositorio.
2. Instala dependencias de Rust y compila los binarios de Tauri:

   ```bash
   rustup target add x86_64-unknown-linux-gnu  # Ajusta según tu plataforma
   ```

3. Instala dependencias del frontend:

   ```bash
   pnpm install --recursive
   ```

4. (Opcional) Instala dependencias Python para los importadores:

   ```bash
   python -m pip install -r scripts/requirements.txt
   ```

## Configuración de Piper

1. Descarga el binario de Piper desde la [página oficial](https://github.com/rhasspy/piper/releases) y colócalo en tu `$PATH` o define la variable `PIPER_BIN`.
2. Descarga al menos un modelo `.onnx` y sitúalo en un directorio accesible. Indica la ruta mediante `PIPER_VOICES` o colócalo en `~/.local/share/piper/voices`.
3. (Opcional) Define `PIPER_DEFAULT_VOICE` para seleccionar la voz predeterminada.
4. Asegúrate de tener FFmpeg disponible para las exportaciones a MP3.

## Ejecución

Para iniciar el modo desarrollo con recarga en caliente:

```bash
pnpm tauri dev
```

Para generar un build de producción:

```bash
pnpm --dir ui build
cargo tauri build
```

## Uso

### Importar documentos

- Desde la tarjeta "Importar documento" selecciona un archivo EPUB, PDF o TXT.
- El contenido se carga en el área de texto y puedes editarlo antes de añadirlo a la cola.
- El script Python correspondiente procesa el archivo y devuelve texto plano.

### Cola de lectura y síntesis

- Cada elemento en la cola muestra título, voz y estado.
- Usa "Reproducir siguiente" para sintetizar el elemento pendiente más antiguo.
- Los eventos `queue::completed` y `queue::failed` actualizan la UI al finalizar Piper.
- Ajusta la voz y la velocidad desde el panel de ajustes.

### Exportación de audio

- Define el directorio de exportación desde los ajustes.
- Los elementos completados incluyen un enlace directo al archivo generado.
- Para exportaciones manuales a MP3 se utiliza FFmpeg; asegúrate de tenerlo instalado.

## Automatización y QA

### Comandos útiles

```bash
# Formato + test Rust
cargo fmt -- --check
cargo test

# Lint y pruebas frontend
pnpm --dir ui lint
pnpm --dir ui test

# Pruebas de scripts Python
pytest
```

### Integración continua

El workflow `ci.yml` ejecuta automáticamente:

1. Formateo y pruebas de Rust (`cargo fmt`, `cargo test`).
2. Linter y pruebas del frontend (`pnpm --dir ui lint`, `pnpm --dir ui test`).
3. Pytest para los importadores (`pytest`).

## Estructura del repositorio

```
.
├── package.json               # Scripts del workspace
├── pnpm-workspace.yaml        # Configuración de pnpm
├── pyproject.toml             # Ajustes de pytest
├── scripts/                   # Importadores Python
├── src-tauri/                 # Proyecto Rust + Tauri
├── tests/                     # Tests de Python
└── ui/                        # Frontend Svelte + Vite
```

## Contribuir

1. Abre un issue describiendo la mejora o bug.
2. Crea una rama desde `main` y abre un PR.
3. Ejecuta todas las pruebas locales antes de solicitar revisión.

## Licencia

Este proyecto se distribuye bajo la licencia MIT. Consulta el archivo `LICENSE` si está disponible o agrega una según tus necesidades.
