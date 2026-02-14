//! Estado global, comandos de audio y funciones de control de velocidad.

use std::sync::atomic::{AtomicBool, AtomicU32, AtomicU8, Ordering};
use std::sync::mpsc::Sender;

use lazy_static::lazy_static;
use parking_lot::Mutex;

use crate::config::{VELOCIDADES_PRESET, VELOCIDAD_INICIAL};
use crate::logging::log;

// ═══════════════════════════════════════════════════════════════════════════════
// COMANDOS Y ESTADO GLOBAL
// ═══════════════════════════════════════════════════════════════════════════════

/// Comandos que se envían al hilo de audio
#[derive(Debug)]
pub enum ComandoAudio {
    /// Reproducir el archivo WAV temporal
    Reproducir,
    /// Detener la reproducción actual
    Detener,
    /// Pausar/Reanudar la reproducción
    TogglePausa,
}

// Estados de reproducción
pub const ESTADO_IDLE: u8 = 0;
pub const ESTADO_REPRODUCIENDO: u8 = 1;
pub const ESTADO_PAUSADO: u8 = 2;

/// Estado atómico de reproducción
pub static ESTADO_AUDIO: AtomicU8 = AtomicU8::new(ESTADO_IDLE);

/// Velocidad actual (length_scale × 100)
pub static VELOCIDAD_ACTUAL: AtomicU32 = AtomicU32::new(VELOCIDAD_INICIAL);

/// Flag para indicar que la aplicación debe terminar
pub static DEBE_SALIR: AtomicBool = AtomicBool::new(false);

lazy_static! {
    /// Canal para enviar comandos al hilo de audio
    pub static ref CANAL_AUDIO: Mutex<Option<Sender<ComandoAudio>>> = Mutex::new(None);
}

// ═══════════════════════════════════════════════════════════════════════════════
// FUNCIONES DE CONTROL
// ═══════════════════════════════════════════════════════════════════════════════

/// Envía un comando al hilo de audio
pub fn enviar_comando(comando: ComandoAudio) {
    let guard = CANAL_AUDIO.lock();
    if let Some(ref sender) = *guard {
        let _ = sender.send(comando);
    }
}

/// Obtiene la velocidad actual como float (length_scale para Piper)
pub fn obtener_velocidad() -> f32 {
    VELOCIDAD_ACTUAL.load(Ordering::SeqCst) as f32 / 100.0
}

/// Establece la velocidad a un valor absoluto de length_scale × 100.
pub fn establecer_velocidad(length_scale_x100: u32) {
    let actual = VELOCIDAD_ACTUAL.load(Ordering::SeqCst);
    if length_scale_x100 != actual {
        VELOCIDAD_ACTUAL.store(length_scale_x100, Ordering::SeqCst);
        let etiqueta = etiqueta_velocidad_actual();
        log(&format!(
            "Velocidad establecida: {} (length_scale: {:.2})",
            etiqueta,
            length_scale_x100 as f32 / 100.0
        ));
    }
}

/// Devuelve el índice del preset actual en `VELOCIDADES_PRESET`.
/// Si el valor no coincide exactamente, devuelve el más cercano.
pub fn indice_preset_actual() -> usize {
    let actual = VELOCIDAD_ACTUAL.load(Ordering::SeqCst);
    VELOCIDADES_PRESET
        .iter()
        .enumerate()
        .min_by_key(|(_, (_, ls))| (*ls as i32 - actual as i32).unsigned_abs())
        .map(|(i, _)| i)
        .unwrap_or(0)
}

/// Devuelve la etiqueta del preset de velocidad actual (ej: "x1.25").
pub fn etiqueta_velocidad_actual() -> &'static str {
    let idx = indice_preset_actual();
    VELOCIDADES_PRESET[idx].0
}

/// Cicla al siguiente preset más rápido (mayor multiplicador → menor length_scale).
pub fn velocidad_preset_mas_rapido() {
    let idx = indice_preset_actual();
    if idx + 1 < VELOCIDADES_PRESET.len() {
        let (etiqueta, ls) = VELOCIDADES_PRESET[idx + 1];
        establecer_velocidad(ls);
        log(&format!("Hotkey: Velocidad → {}", etiqueta));
    }
}

/// Cicla al siguiente preset más lento (menor multiplicador → mayor length_scale).
pub fn velocidad_preset_mas_lento() {
    let idx = indice_preset_actual();
    if idx > 0 {
        let (etiqueta, ls) = VELOCIDADES_PRESET[idx - 1];
        establecer_velocidad(ls);
        log(&format!("Hotkey: Velocidad → {}", etiqueta));
    }
}
