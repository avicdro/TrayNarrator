//! Estado global, comandos de audio y funciones de control de velocidad.

use std::sync::atomic::{AtomicBool, AtomicU32, AtomicU8, Ordering};
use std::sync::mpsc::Sender;

use lazy_static::lazy_static;
use parking_lot::Mutex;

use crate::config::{VELOCIDAD_INICIAL, VELOCIDAD_MAX, VELOCIDAD_MIN, VELOCIDAD_PASO};
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

/// Velocidad actual (length_scale * 100)
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

/// Obtiene la velocidad actual como float
pub fn obtener_velocidad() -> f32 {
    VELOCIDAD_ACTUAL.load(Ordering::SeqCst) as f32 / 100.0
}

/// Ajusta la velocidad (positivo = más lento, negativo = más rápido)
pub fn ajustar_velocidad(incremento: i32) {
    let actual = VELOCIDAD_ACTUAL.load(Ordering::SeqCst);
    let nueva = if incremento > 0 {
        (actual + VELOCIDAD_PASO).min(VELOCIDAD_MAX)
    } else {
        actual.saturating_sub(VELOCIDAD_PASO).max(VELOCIDAD_MIN)
    };

    if nueva != actual {
        VELOCIDAD_ACTUAL.store(nueva, Ordering::SeqCst);
        log(&format!(
            "Velocidad ajustada: {}% (length_scale: {:.2})",
            nueva,
            nueva as f32 / 100.0
        ));
    }
}
