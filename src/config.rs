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

/// Velocidad inicial (length_scale * 100, ej: 80 = 0.8)
pub const VELOCIDAD_INICIAL: u32 = 80;

/// Velocidad mínima (50 = 0.5, muy rápido)
pub const VELOCIDAD_MIN: u32 = 50;

/// Velocidad máxima (150 = 1.5, muy lento)
pub const VELOCIDAD_MAX: u32 = 150;

/// Incremento/decremento de velocidad
pub const VELOCIDAD_PASO: u32 = 10;

// Para ocultar la ventana de Piper en Windows
#[cfg(windows)]
pub const CREATE_NO_WINDOW: u32 = 0x08000000;
