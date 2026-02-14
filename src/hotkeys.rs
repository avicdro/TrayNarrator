//! Manejadores de teclas globales e integración con inputbot.

use std::sync::atomic::Ordering;
use std::thread;

use inputbot::KeybdKey;

use crate::clipboard::{leer_portapapeles, simular_copiar};
use crate::logging::log;
use crate::state::{
    enviar_comando, velocidad_preset_mas_lento, velocidad_preset_mas_rapido, ComandoAudio,
    ESTADO_AUDIO, ESTADO_IDLE,
};
use crate::tts::generar_audio_piper;

// ═══════════════════════════════════════════════════════════════════════════════
// MANEJADORES DE TECLAS
// ═══════════════════════════════════════════════════════════════════════════════

fn manejar_f8() {
    log("=== F8 presionado ===");
    enviar_comando(ComandoAudio::Detener);

    log("Simulando Ctrl+C...");
    if let Err(e) = simular_copiar() {
        log(&format!("Error Ctrl+C: {}", e));
        return;
    }

    log("Leyendo portapapeles...");
    let texto = match leer_portapapeles() {
        Ok(t) => {
            log(&format!("Texto: {} chars", t.len()));
            t
        }
        Err(e) => {
            log(&format!("Error portapapeles: {}", e));
            return;
        }
    };

    if let Err(e) = generar_audio_piper(&texto) {
        log(&format!("Error Piper: {}", e));
        return;
    }

    log("Enviando Reproducir...");
    enviar_comando(ComandoAudio::Reproducir);
}

fn manejar_f9() {
    let estado = ESTADO_AUDIO.load(Ordering::SeqCst);
    if estado != ESTADO_IDLE {
        enviar_comando(ComandoAudio::TogglePausa);
    }
}

fn manejar_mas_rapido() {
    log("Ctrl+[ - Más rápido");
    velocidad_preset_mas_rapido();
}

fn manejar_mas_lento() {
    log("Ctrl+] - Más lento");
    velocidad_preset_mas_lento();
}

// ═══════════════════════════════════════════════════════════════════════════════
// HILO DE INPUTBOT
// ═══════════════════════════════════════════════════════════════════════════════

/// Registra los atajos de teclado globales y arranca el bucle de eventos.
pub fn hilo_inputbot() {
    log("Hilo inputbot iniciado");

    // F8: Leer
    KeybdKey::F8Key.bind(|| {
        thread::spawn(manejar_f8);
    });

    // F9: Pausar/Reanudar
    KeybdKey::F9Key.bind(|| {
        manejar_f9();
    });

    // Ctrl+[ : Más rápido (siguiente preset)
    KeybdKey::LBracketKey.bind(|| {
        if KeybdKey::LControlKey.is_pressed() || KeybdKey::RControlKey.is_pressed() {
            manejar_mas_rapido();
        }
    });

    // Ctrl+] : Más lento (preset anterior)
    KeybdKey::RBracketKey.bind(|| {
        if KeybdKey::LControlKey.is_pressed() || KeybdKey::RControlKey.is_pressed() {
            manejar_mas_lento();
        }
    });

    // Bucle de eventos
    inputbot::handle_input_events();

    log("Hilo inputbot terminado");
}
