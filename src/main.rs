//! TrayNarrator - Lector TTS ultraligero para Windows usando Piper TTS
//!
//! Este programa escucha atajos de teclado globales para leer texto seleccionado.
//! - F8: Copia el texto seleccionado y lo lee en voz alta
//! - F9: Pausa/Reanuda la reproducción
//! - Ctrl+[: Reducir velocidad (más rápido)
//! - Ctrl+]: Aumentar velocidad (más lento)
//!
//! COMPILACIÓN CRUZADA (desde WSL2):
//! cargo build --release --target x86_64-pc-windows-gnu
//!
//! El binario estará en: target/x86_64-pc-windows-gnu/release/tray_narrator.exe

// Oculta la ventana de consola en Windows (aplicación GUI sin consola visible)
#![windows_subsystem = "windows"]

mod audio;
mod clipboard;
mod config;
mod hotkeys;
mod logging;
mod state;
#[cfg(windows)]
mod tray;
mod tts;

use std::sync::mpsc;
use std::thread;

use config::VELOCIDAD_INICIAL;
use logging::log;
use state::{ComandoAudio, CANAL_AUDIO};

fn main() {
    log("=== TrayNarrator iniciado ===");
    log(&format!("Velocidad inicial: {}%", VELOCIDAD_INICIAL));

    // Crear canal de audio
    let (sender, receiver) = mpsc::channel::<ComandoAudio>();
    {
        let mut guard = CANAL_AUDIO.lock();
        *guard = Some(sender);
    }

    // Iniciar hilo de audio
    thread::spawn(move || {
        audio::hilo_audio(receiver);
    });

    // Iniciar hilo de inputbot
    thread::spawn(|| {
        hotkeys::hilo_inputbot();
    });

    // En Windows, ejecutar el system tray en el hilo principal
    #[cfg(windows)]
    {
        tray::run_tray();
    }

    // En otros sistemas, simplemente esperar
    #[cfg(not(windows))]
    {
        use std::sync::atomic::Ordering;
        use std::time::Duration;
        loop {
            if state::DEBE_SALIR.load(Ordering::SeqCst) {
                break;
            }
            thread::sleep(Duration::from_millis(100));
        }
    }

    log("=== TrayNarrator terminado ===");
}
