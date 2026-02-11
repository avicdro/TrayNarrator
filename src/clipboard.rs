//! Funciones de portapapeles y simulación de input.

use std::thread;
use std::time::Duration;

use arboard::Clipboard;
use enigo::{Enigo, Key, Keyboard, Settings};

use crate::config::DELAY_COPIAR_MS;

/// Simula la pulsación de Ctrl+C para copiar el texto seleccionado
pub fn simular_copiar() -> Result<(), String> {
    let mut enigo = Enigo::new(&Settings::default())
        .map_err(|e| format!("Error inicializando Enigo: {}", e))?;

    // Simular Ctrl+C
    enigo
        .key(Key::Control, enigo::Direction::Press)
        .map_err(|e| format!("Error presionando Ctrl: {}", e))?;
    enigo
        .key(Key::Unicode('c'), enigo::Direction::Click)
        .map_err(|e| format!("Error presionando C: {}", e))?;
    enigo
        .key(Key::Control, enigo::Direction::Release)
        .map_err(|e| format!("Error soltando Ctrl: {}", e))?;

    thread::sleep(Duration::from_millis(DELAY_COPIAR_MS));
    Ok(())
}

/// Lee el texto del portapapeles
pub fn leer_portapapeles() -> Result<String, String> {
    let mut clipboard =
        Clipboard::new().map_err(|e| format!("Error accediendo al portapapeles: {}", e))?;

    let texto = clipboard
        .get_text()
        .map_err(|e| format!("Error leyendo texto del portapapeles: {}", e))?;

    if texto.trim().is_empty() {
        return Err("El portapapeles está vacío o no contiene texto".to_string());
    }

    Ok(texto)
}
