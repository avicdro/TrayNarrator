//! Función de logging para debugging.

use std::fs::OpenOptions;
use std::io::Write;

use crate::config::RUTA_LOG;

/// Escribe un mensaje en el archivo de log para debugging
pub fn log(mensaje: &str) {
    if let Ok(mut archivo) = OpenOptions::new().create(true).append(true).open(RUTA_LOG) {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        let _ = writeln!(archivo, "[{}] {}", timestamp, mensaje);
    }
}
