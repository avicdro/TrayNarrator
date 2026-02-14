//! Configuración y constantes del programa.
//!
//! Modifica estas constantes según tu instalación.

/// Ruta al ejecutable de Piper TTS (formato Windows)
pub const RUTA_PIPER: &str = r"C:\TrayNarrator\piper\piper.exe";

/// Ruta al modelo de voz .onnx de Piper (formato Windows)
pub const RUTA_MODELO: &str = r"C:\TrayNarrator\piper\es_ES-sharvard-medium.onnx";

/// Ruta del archivo temporal WAV
pub const RUTA_TEMP_WAV: &str = r"C:\TrayNarrator\temp.wav";

/// Ruta del archivo de log para debugging
pub const RUTA_LOG: &str = r"C:\TrayNarrator\log.txt";

/// Tiempo de espera después de simular Ctrl+C (milisegundos)
pub const DELAY_COPIAR_MS: u64 = 150;

/// Presets de velocidad: (etiqueta, length_scale × 100).
///
/// `length_scale = 1.0 / multiplicador`. Piper usa length_scale para controlar
/// la duración de los fonemas: menor = más rápido.
///
/// Ejemplo: x2 → length_scale 0.50 → almacenado como 50.
pub const VELOCIDADES_PRESET: &[(&str, u32)] = &[
    ("x0.5", 200),  // length_scale 2.00 — muy lento
    ("x0.75", 133), // length_scale 1.33
    ("x1", 100),    // length_scale 1.00 — normal
    ("x1.25", 80),  // length_scale 0.80
    ("x1.5", 67),   // length_scale 0.67
    ("x2", 50),     // length_scale 0.50
    ("x3", 33),     // length_scale 0.33 — muy rápido
];

/// Índice del preset por defecto (x1 = velocidad normal)
pub const VELOCIDAD_PRESET_DEFECTO: usize = 2;

/// Velocidad inicial (length_scale × 100). Corresponde al preset por defecto.
pub const VELOCIDAD_INICIAL: u32 = VELOCIDADES_PRESET[VELOCIDAD_PRESET_DEFECTO].1;

/// Versión de la aplicación (obtenida de Cargo.toml)
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

// Para ocultar la ventana de Piper en Windows
#[cfg(windows)]
pub const CREATE_NO_WINDOW: u32 = 0x08000000;
